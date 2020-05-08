use libchordr::prelude::SetlistEntry;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::command::{CommandExecutor, Command};
use diesel::prelude::*;
use crate::error::SrvError;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::diesel::QueryDsl;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSetlist {
    pub id: i32,
    pub user: i32,
    pub user_name: String,
    pub entries: Vec<SetlistEntry>,
}

impl UserSetlist {
    pub fn from_data(setlist_db: SetlistDb, db_entries: Vec<SetlistDbEntry>) -> Self {
        let entries = db_entries
            .iter()
            .map(SetlistEntry::from_song)
            .collect::<Vec<SetlistEntry>>();

        Self {
            id: setlist_db.id,
            user: setlist_db.user,
            user_name: setlist_db.user_name,
            entries,
        }
    }

    fn as_setlist_db(&self) -> SetlistDb {
        SetlistDb {
            id: self.id,
            user: self.user,
            user_name: self.user_name.clone(),
        }
    }

    fn get_setlist_db_entries(&self) -> Vec<SetlistDbEntry> {
        let setlist_db = self.as_setlist_db();
        self.entries.iter().map(|e| SetlistDbEntry::from(e, &setlist_db)).collect()
    }

    fn insert_setlist_db_entries(&self, command: &Command) -> Result<(), SrvError> {
        diesel::insert_into(crate::schema::setlist_entry::table)
            .values(&self.get_setlist_db_entries())
            .execute(command.connection)?;

        Ok(())
    }
}

impl CommandExecutor for UserSetlist {
    type Error = SrvError;

    fn add(self, command: Command) -> Result<(), Self::Error> {
        let setlist_db = self.as_setlist_db();

        command.connection.transaction::<(), Self::Error, _>(|| {
            diesel::insert_into(crate::schema::setlist::table)
                .values(&setlist_db)
                .execute(command.connection)?;

            self.insert_setlist_db_entries(&command)
        })?;

        Ok(())
    }

    fn update(self, command: Command) -> Result<(), Self::Error> {
        let setlist_db_query = all_setlists.find(self.id);
        let setlist_db_instance = match setlist_db_query.get_result::<SetlistDb>(command.connection) {
            Ok(setlist_db_instance) => setlist_db_instance,
            Err(_) => return Err(SrvError::persistence_error(format!("Original object with ID '{}' could not be found", self.id)))
        };
        command.connection.transaction::<(), Self::Error, _>(|| {
            diesel::update(setlist_db_query)
                .set(self.as_setlist_db())
                .execute(command.connection)?;

            // Delete the current associated Setlist Entries
            diesel::delete(SetlistDbEntry::belonging_to(&setlist_db_instance))
                .execute(command.connection)?;

            // Insert the updated Setlist Entries
            self.insert_setlist_db_entries(&command)
        })?;

        Ok(())
    }

    fn delete(self, command: Command) -> Result<(), Self::Error> {
        diesel::delete(all_setlists.find(self.id))
            .execute(command.connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::setlist::db::SetlistDb;
    use crate::test_helpers::*;
    use crate::domain::setlist_entry::db::SetlistDbEntry;
    use crate::DbConn;
    use libchordr::models::file_type::FileType;

    #[test]
    fn test_add_empty() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn.0);
            let init_setlist_entries = SetlistDbEntry::all(&conn.0);

            let new_setlist = UserSetlist {
                id: 918,
                user: 819,
                user_name: "Paul".to_string(),
                entries: vec![],
            };

            CommandExecutor::perform(new_setlist, Command::add(&conn.0)).unwrap();

            let new_setlists = SetlistDb::all(&conn.0);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);

            let new_setlist_entries = SetlistDbEntry::all(&conn.0);
            // There should be no new entries
            assert_eq!(new_setlist_entries.len(), init_setlist_entries.len());
        })
    }

    #[test]
    fn test_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn.0);

            let new_setlist = UserSetlist {
                id: 918,
                user: 819,
                user_name: "Paul".to_string(),
                entries: vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1"),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2"),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3")
                ],
            };

            CommandExecutor::perform(new_setlist, Command::add(&conn.0)).unwrap();

            let new_setlists = SetlistDb::all(&conn.0);
            assert_eq!(new_setlists.len(), init_setlists.len() + 1);
            assert_eq!(new_setlists.len(), 1);

            let new_setlist_entries = SetlistDbEntry::all(&conn.0);
            assert_eq!(new_setlist_entries.len(), 3);
        })
    }

    #[test]
    fn test_update_empty() {
        run_database_test(|conn| {
            clear_database(&conn);
            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn.0), 6);


            CommandExecutor::perform(
                UserSetlist {
                    id: 918, // Same ID
                    user: 8190, // New User
                    user_name: "Paul".to_string(), // New name
                    entries: vec![],
                },
                Command::update(&conn.0),
            ).unwrap();

            assert_eq!(SetlistDb::count_all(&conn.0), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn.0), 3);
        })
    }

    #[test]
    fn test_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn.0), 6);

            CommandExecutor::perform(
                UserSetlist {
                    id: 918, // Same ID
                    user: 8190, // New User
                    user_name: "Paul".to_string(), // New name
                    entries: vec![
                        SetlistEntry::new("song-4", FileType::Chorddown, "Song 4"),
                    ],
                },
                Command::update(&conn.0),
            ).unwrap();

            assert_eq!(SetlistDb::count_all(&conn.0), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn.0), 4);
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 1918, 1819);

            CommandExecutor::perform(
                UserSetlist {
                    id: 918, // This is important
                    user: 0,
                    user_name: "".to_string(),
                    entries: vec![],
                },
                Command::delete(&conn.0),
            ).unwrap();

            assert_eq!(SetlistDb::count_all(&conn.0), 1);
        })
    }

    fn insert_test_setlist(conn: &DbConn, id: i32, user: i32) {
        CommandExecutor::perform(
            UserSetlist {
                id,
                user,
                user_name: "Saul".to_string(),
                entries: vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1"),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2"),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3"),
                ],
            },
            Command::add(&conn.0),
        ).unwrap();
    }

    fn clear_database(conn: &DbConn) {
        assert!(
            SetlistDb::delete_all(&conn.0),
            "Failed to delete all data before testing"
        );
        assert!(
            SetlistDbEntry::delete_all(&conn.0),
            "Failed to delete all data before testing"
        );
    }
}
