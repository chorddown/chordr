use diesel::{self, prelude::*};

use cqrs::prelude::{Command, CommandExecutor};
use libchordr::prelude::Setlist;

use crate::diesel::QueryDsl;
use crate::domain::cqs_context::CqsContext;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::error::SrvError;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::ConnectionType;

fn as_setlist_db(setlist: &Setlist) -> SetlistDb {
    SetlistDb {
        name: setlist.name().to_owned(),
        id: setlist.id(),
        owner: setlist.owner().username().to_string(),
        team: setlist.team().as_ref().map(|t| t.id().to_string()),
        gig_date: setlist.gig_date().map(|d| d.naive_utc()),
        creation_date: setlist.creation_date().naive_utc(),
        sorting: 0, // setlist.sorting(),
        modification_date: setlist.modification_date().naive_utc(),
    }
}

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
            let setlist_db = as_setlist_db(setlist);
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
}

impl<'a> CommandExecutor for SetlistCommandExecutor<'_> {
    type RecordType = Setlist;
    type Error = SrvError;
    type Context = CqsContext;

    fn add(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record().unwrap();
        let setlist_db = as_setlist_db(setlist);

        self.connection.transaction::<(), Self::Error, _>(|| {
            diesel::insert_into(crate::schema::setlist::table)
                .values(&setlist_db)
                .execute(self.connection)?;

            self.insert_setlist_db_entries(setlist, &command)
        })?;

        Ok(())
    }

    fn update(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), Self::Error> {
        let setlist = command.record().unwrap();
        let setlist_db_query = all_setlists.find(setlist.id());
        let setlist_db_instance = match setlist_db_query.get_result::<SetlistDb>(self.connection) {
            Ok(setlist_db_instance) => setlist_db_instance,
            Err(_) => {
                return Err(SrvError::persistence_error(format!(
                    "Original object with ID '{}' could not be found",
                    setlist.id()
                )));
            }
        };
        self.connection.transaction::<(), Self::Error, _>(|| {
            diesel::update(setlist_db_query)
                .set(as_setlist_db(setlist))
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
        diesel::delete(all_setlists.find(command.id().unwrap())).execute(self.connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use libchordr::prelude::{FileType, SetlistEntry, User, Username};

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
                User::new(
                    Username::new("paul-819").unwrap(),
                    "Paul",
                    "Doe",
                    create_test_password(),
                ),
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
                User::new(
                    Username::new("paul-819").unwrap(),
                    "Paul",
                    "Doe",
                    create_test_password(),
                ),
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
                Command::add(new_setlist, ()),
            )
            .unwrap();

            let new_setlists = SetlistDb::all(&conn);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);

            let new_setlist_entries = SetlistDbEntry::all(&conn);
            assert_eq!(new_setlist_entries.len(), 3);
        })
    }

    #[test]
    fn test_update_empty() {
        run_database_test(|conn| {
            clear_database(&conn);
            // insert_test_setlist(&conn, 918, 819);
            create_setlist(&conn, 918, "819");
            // insert_test_setlist(&conn, 8, 819);
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(
                    Setlist::new(
                        "My setlist #918",
                        918, // Same ID
                        // New User:
                        User::new(
                            Username::new("paul-8190").unwrap(),
                            "Paul",
                            "Doe",
                            create_test_password(),
                        ),
                        None,
                        None,
                        Utc::now(),
                        Utc::now(),
                        vec![],
                    ),
                    (),
                ),
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
            // insert_test_setlist(&conn, 918, 819);
            create_setlist(&conn, 918, "819");
            // insert_test_setlist(&conn, 8, 819);
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::update(
                    Setlist::new(
                        "My setlist #918",
                        918, // Same ID
                        // New User:
                        User::new(
                            Username::new("paul-8190").unwrap(),
                            "Paul",
                            "Doe",
                            create_test_password(),
                        ),
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
            )
            .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 4);
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
                "Original object with ID '10001' could not be found".to_owned(),
            );
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            // insert_test_setlist(&conn, 918, 819);
            create_setlist(&conn, 918, "819");
            // insert_test_setlist(&conn, 1918, 1819);
            create_setlist(&conn, 1918, "1819");

            CommandExecutor::perform(
                &SetlistCommandExecutor::new_with_connection(&conn),
                Command::delete(918, ()),
            )
            .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 1);
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
