use crate::diesel::QueryDsl;
use crate::domain::cqs_context::CqsContext;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist::setlist_db_id::ToSetlistDbId;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::error::SrvError;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::schema::setlist::id;
use crate::ConnectionType;
use cqrs::prelude::{Command, CommandExecutor};
use diesel::{self, prelude::*, NotFound};
use libchordr::prelude::Setlist;

pub(crate) struct SetlistCommandExecutor<'a> {
    connection: &'a ConnectionType,
}

impl<'a> SetlistCommandExecutor<'a> {
    pub(crate) fn new_with_connection(connection: &'a ConnectionType) -> Self {
        Self { connection }
    }

    fn insert_setlist_db_entries(
        &self,
        setlist: &Setlist,
        _command: &Command<Setlist, ()>,
    ) -> Result<(), SrvError> {
        fn get_setlist_db_entries(setlist: &Setlist) -> Vec<SetlistDbEntry> {
            let setlist_db = SetlistDb::from(setlist);
            setlist
                .as_song_list()
                .iter()
                .map(|e| SetlistDbEntry::from(e, &setlist_db))
                .collect()
        }

        diesel::insert_into(crate::schema::setlist_entry::table)
            .values(get_setlist_db_entries(setlist))
            .execute(self.connection)?;

        Ok(())
    }

    fn explain_update_error(&self, setlist: &Setlist) -> SrvError {
        let username = setlist.owner().username();
        let setlist_id = setlist.id();
        let similar_setlist: Result<SetlistDb, _> = all_setlists
            .filter(id.eq(setlist_id))
            .get_result(self.connection);
        let description = match similar_setlist {
            Ok(similar_setlist) if &similar_setlist.owner != username.as_ref() => {
                "Owner does not match"
            }
            Ok(_) => "UID does not match",
            Err(NotFound) => "Setlist does not exist",
            Err(e) => panic!("{}", e),
        };

        return SrvError::persistence_error(format!(
            "Original object with ID '{}' for user '{}' could not be found: {} (UID: '{}')",
            setlist_id,
            username,
            description,
            setlist.to_setlist_db_uid(),
        ));
    }
}

impl<'a> CommandExecutor for SetlistCommandExecutor<'_> {
    type RecordType = Setlist;
    type Error = SrvError;
    type Context = CqsContext;

    fn upsert(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record();
        let setlist_db_query = all_setlists.find(setlist.to_setlist_db_uid());
        if let Ok(1) = setlist_db_query.count().get_result::<i64>(self.connection) {
            self.update(command)
        } else {
            self.add(command)
        }
    }

    fn add(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record();
        let setlist_db = SetlistDb::from(setlist);

        self.connection.transaction::<(), Self::Error, _>(|| {
            diesel::insert_into(crate::schema::setlist::table)
                .values(&setlist_db)
                .execute(self.connection)?;

            self.insert_setlist_db_entries(setlist, &command)
        })?;

        Ok(())
    }

    fn update(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record();
        let setlist_db_query = all_setlists.find(setlist.to_setlist_db_uid());
        let setlist_db_instance: SetlistDb = match setlist_db_query.get_result(self.connection) {
            Ok(setlist_db_instance) => setlist_db_instance,
            Err(_) => {
                return Err(self.explain_update_error(setlist));
            }
        };
        self.connection.transaction::<(), Self::Error, _>(|| {
            diesel::update(setlist_db_query)
                .set(SetlistDb::from(setlist))
                .execute(self.connection)?;

            // Delete the current associated Setlist Entries
            diesel::delete(SetlistDbEntry::belonging_to(&setlist_db_instance))
                .execute(self.connection)?;

            // Insert the updated Setlist Entries
            self.insert_setlist_db_entries(setlist, &command)
        })?;

        Ok(())
    }

    fn delete(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record();
        diesel::delete(all_setlists.find(setlist.to_setlist_db_uid())).execute(self.connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use rocket::form::validate::Contains;

    use libchordr::prelude::{FileType, SetlistEntry, User};

    use crate::domain::setlist::db::SetlistDb;
    use crate::domain::setlist_entry::db::SetlistDbEntry;
    use crate::test_helpers::*;
    use crate::ConnectionType;

    use super::*;

    #[test]
    fn test_add_empty() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn);
            let init_setlist_entries = SetlistDbEntry::all(&conn);

            let new_setlist = Setlist::new(
                "My setlist #918",
                918,
                create_test_user("paul-819"),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![],
            );

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::add(new_setlist, ()),
            )
            .unwrap();

            let new_setlists = SetlistDb::all(&conn);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);

            let new_setlist_entries = SetlistDbEntry::all(&conn);
            // There should be no new entries
            assert_eq!(new_setlist_entries.len(), init_setlist_entries.len());
        })
    }

    #[test]
    fn test_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn);

            let new_setlist = Setlist::new(
                "My setlist #918",
                918, // Same ID
                // New User:
                create_test_user("paul-819"),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
                ],
            );

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::add(new_setlist.clone(), ()),
            )
            .unwrap();

            let new_setlists = SetlistDb::all(&conn);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);
            assert!(new_setlists.contains(&SetlistDb::from(new_setlist)));

            let new_setlist_entries = SetlistDbEntry::all(&conn);
            assert_eq!(new_setlist_entries.len(), 3);
        })
    }

    #[test]
    fn test_update_empty() {
        run_database_test(|conn| {
            clear_database(&conn);
            create_setlist(&conn, 918, "819");
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            let empty_setlist = Setlist::new(
                "My setlist #918",
                918, // Same ID
                // Same User:
                create_test_user("819"),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![],
            );
            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(empty_setlist, ()),
            )
            .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 3);
        })
    }

    #[test]
    fn test_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            create_setlist(&conn, 918, "819");
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            let result = CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(
                    Setlist::new(
                        "My setlist #918",
                        918, // Same ID
                        // Same User:
                        create_test_user("819"),
                        None,
                        None,
                        Utc::now(),
                        Utc::now(),
                        vec![SetlistEntry::new(
                            "song-4",
                            FileType::Chorddown,
                            "Song 4",
                            None,
                        )],
                    ),
                    (),
                ),
            );

            assert!(result.is_ok(), "{}", result.unwrap_err());
            result.unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 4);
        })
    }

    #[test]
    fn test_update_with_different_user() {
        run_database_test(|conn| {
            clear_database(&conn);
            create_setlist(&conn, 918, "819");
            assert_eq!(SetlistDbEntry::count_all(&conn), 3);

            let result = CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(
                    Setlist::new(
                        "My setlist #918",
                        918, // Same ID
                        // New User:
                        create_test_user("paul-8190"),
                        None,
                        None,
                        Utc::now(),
                        Utc::now(),
                        vec![],
                    ),
                    (),
                ),
            );

            assert!(result.is_err());

            let error = result.unwrap_err();
            assert_eq!(error.to_string(),"Original object with ID '918' for user 'paul-8190' could not be found: Owner does not match (UID: '-2223442598701511142')");
        })
    }

    #[test]
    fn test_update_not_existing() {
        run_database_test(|conn| {
            clear_database(&conn);
            let result = CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(
                    Setlist::new(
                        "My setlist #918",
                        10001,
                        User::unknown(),
                        None,
                        None,
                        Utc::now(),
                        Utc::now(),
                        vec![SetlistEntry::new(
                            "song-4",
                            FileType::Chorddown,
                            "Song 4",
                            None,
                        )],
                    ),
                    (),
                ),
            );
            assert_eq!(
                result.unwrap_err().to_string(),
                "Original object with ID '10001' for user 'unknown' could not be found: Setlist does not exist (UID: '-5386760737432561227')".to_owned(),
            );
        })
    }

    #[test]
    fn test_upsert_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn);

            let new_setlist = Setlist::new(
                "My setlist #918",
                918,
                create_test_user("paul-819"),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
                ],
            );

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::upsert(new_setlist.clone(), ()),
            )
            .unwrap();

            let new_setlists = SetlistDb::all(&conn);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);

            assert!(new_setlists.contains(SetlistDb::from(new_setlist)));
        })
    }

    #[test]
    fn test_upsert_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            create_setlist(&conn, 918, "819");
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            let setlist = Setlist::new(
                "My setlist #918",
                918, // Same ID
                // Same User:
                create_test_user("819"),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![SetlistEntry::new(
                    "song-4",
                    FileType::Chorddown,
                    "Song 4",
                    None,
                )],
            );
            let result = CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::upsert(setlist.clone(), ()),
            );

            assert!(result.is_ok(), "{}", result.unwrap_err());
            result.unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 4);

            let updated_setlists = SetlistDb::all(&conn);
            assert!(updated_setlists.contains(SetlistDb::from(setlist)));
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);
            create_setlist(&conn, 918, "819");
            create_setlist(&conn, 1918, "1819");

            let now = Utc::now();
            let setlist_to_delete = Setlist::new(
                "My setlist",
                918,
                create_test_user("819"),
                None,
                None,
                now,
                now,
                vec![],
            );
            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::delete(setlist_to_delete.clone(), ()),
            )
            .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 1);
            assert!(!SetlistDb::all(&conn).contains(SetlistDb::from(setlist_to_delete)));
        })
    }

    fn clear_database(conn: &ConnectionType) {
        assert!(
            SetlistDb::delete_all(conn),
            "Failed to delete all data before testing"
        );
        assert!(
            SetlistDbEntry::delete_all(conn),
            "Failed to delete all data before testing"
        );
    }
}
