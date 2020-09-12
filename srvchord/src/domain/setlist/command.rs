use crate::command::{Command, CommandExecutor};
use crate::diesel::QueryDsl;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::error::SrvError;
use crate::schema::setlist::dsl::setlist as all_setlists;
use diesel::{self, prelude::*};
use libchordr::prelude::Setlist;

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

fn get_setlist_db_entries(setlist: &Setlist) -> Vec<SetlistDbEntry> {
    let setlist_db = as_setlist_db(setlist);
    setlist
        .as_song_list()
        .iter()
        .map(|e| SetlistDbEntry::from(&e, &setlist_db))
        .collect()
}

fn insert_setlist_db_entries(setlist: &Setlist, command: &Command) -> Result<(), SrvError> {
    diesel::insert_into(crate::schema::setlist_entry::table)
        .values(get_setlist_db_entries(setlist))
        .execute(command.connection)?;

    Ok(())
}

impl CommandExecutor for &Setlist {
    type Error = SrvError;

    fn add(self, command: Command) -> Result<(), Self::Error> {
        let setlist_db = as_setlist_db(self);

        command.connection.transaction::<(), Self::Error, _>(|| {
            diesel::insert_into(crate::schema::setlist::table)
                .values(&setlist_db)
                .execute(command.connection)?;

            insert_setlist_db_entries(self, &command)
        })?;

        Ok(())
    }

    fn update(self, command: Command) -> Result<(), Self::Error> {
        let setlist_db_query = all_setlists.find(self.id());
        let setlist_db_instance = match setlist_db_query.get_result::<SetlistDb>(command.connection)
        {
            Ok(setlist_db_instance) => setlist_db_instance,
            Err(_) => {
                return Err(SrvError::persistence_error(format!(
                    "Original object with ID '{}' could not be found",
                    self.id()
                )));
            }
        };
        command.connection.transaction::<(), Self::Error, _>(|| {
            diesel::update(setlist_db_query)
                .set(as_setlist_db(self))
                .execute(command.connection)?;

            // Delete the current associated Setlist Entries
            diesel::delete(SetlistDbEntry::belonging_to(&setlist_db_instance))
                .execute(command.connection)?;

            // Insert the updated Setlist Entries
            insert_setlist_db_entries(&self, &command)
        })?;

        Ok(())
    }

    fn delete(self, command: Command) -> Result<(), Self::Error> {
        diesel::delete(all_setlists.find(self.id())).execute(command.connection)?;
        Ok(())
    }
}

impl CommandExecutor for Setlist {
    type Error = SrvError;

    fn add(self, command: Command) -> Result<(), Self::Error> {
        CommandExecutor::add(&self, command)
    }

    fn update(self, command: Command) -> Result<(), Self::Error> {
        CommandExecutor::update(&self, command)
    }

    fn delete(self, command: Command) -> Result<(), Self::Error> {
        CommandExecutor::delete(&self, command)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::setlist::db::SetlistDb;
    use crate::domain::setlist_entry::db::SetlistDbEntry;
    use crate::test_helpers::*;
    use crate::ConnectionType;
    use chrono::Utc;
    use libchordr::prelude::{FileType, SetlistEntry, User, Username};

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

            CommandExecutor::perform(new_setlist, Command::add(&conn)).unwrap();

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

            CommandExecutor::perform(new_setlist, Command::add(&conn)).unwrap();

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
                Command::update(&conn),
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
                Command::update(&conn),
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
                Command::update(&conn),
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
                Setlist::new(
                    "My setlist #918",
                    918, // This is important
                    User::unknown(),
                    None,
                    None,
                    Utc::now(),
                    Utc::now(),
                    vec![],
                ),
                Command::delete(&conn),
            )
            .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 1);
        })
    }

    // fn insert_test_setlist(conn: &ConnectionType, id: i32, user: i32) {
    //     CommandExecutor::perform(
    //         UserSetlist {
    //             id,
    //             user,
    //             user_name: "Saul".to_string(),
    //             sorting: id,
    //             entries: vec![
    //                 SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
    //                 SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
    //                 SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
    //             ],
    //         },
    //         Command::add(conn),
    //     )
    //         .unwrap();
    // }

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
