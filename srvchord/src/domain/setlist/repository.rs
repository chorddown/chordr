use crate::command::{Command, CommandExecutor};
use crate::diesel::QueryDsl;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist::UserSetlist;
use crate::error::SrvError;
use crate::schema::setlist;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::traits::*;
use crate::ConnectionType;
use diesel::{self, prelude::*};
use crate::domain::setlist_entry::db::SetlistDbEntry;

pub struct UserSetlistRepository {}

impl UserSetlistRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl RepositoryTrait for UserSetlistRepository {
    type ManagedType = UserSetlist;
    type Error = SrvError;

    fn find_all(&self, connection: &ConnectionType) -> Result<Vec<Self::ManagedType>, Self::Error> {
        let all_setlist_instances: Vec<SetlistDb> = all_setlists
            .order(setlist::id.desc())
            .load(connection)?;
        let entries: Vec<SetlistDbEntry> = SetlistDbEntry::belonging_to(&all_setlist_instances)
            .load(connection)?;
        let grouped_entries: Vec<Vec<SetlistDbEntry>> = entries
            .into_iter()
            .grouped_by(&all_setlist_instances);

        Ok(all_setlist_instances
            .into_iter()
            .zip(grouped_entries)
            .map(|t| UserSetlist::from(t))
            .collect())
    }

    fn count_all(&self, connection: &ConnectionType) -> Result<Count, Self::Error> {
        Ok(all_setlists.count().get_result(connection)?)
    }

    fn find_by_id(
        &self,
        connection: &ConnectionType,
        id: <UserSetlist as RecordIdTrait>::Id,
    ) -> Result<Self::ManagedType, Self::Error> {
        match all_setlists.find(id).get_result::<SetlistDb>(connection) {
            Ok(setlist_db_instance) => {
                let entries = setlist_db_instance.entries(connection);

                Ok(UserSetlist::from_data(setlist_db_instance, entries))
            }
            Err(_) => Err(SrvError::object_not_found_error(format!(
                "Object with ID '{}' could not be found",
                id
            ))),
        }
    }

    fn add(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        CommandExecutor::perform(instance, Command::add(connection))
    }

    fn update(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        CommandExecutor::perform(instance, Command::update(connection))
    }

    fn delete(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        CommandExecutor::perform(instance, Command::delete(connection))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::setlist::db::SetlistDb;
    use crate::domain::setlist_entry::db::SetlistDbEntry;
    use crate::test_helpers::*;
    use libchordr::models::file_type::FileType;
    use libchordr::prelude::SetlistEntry;

    #[test]
    fn test_find_all() {
        run_database_test(|conn| {
            clear_database(&conn);
            let repository = UserSetlistRepository::new();
            assert_eq!(repository.count_all(&conn).unwrap(), 0);
            assert_eq!(repository.find_all(&conn).unwrap(), vec![]);

            let inserted_setlists = vec![
                insert_test_setlist(&conn, 918, 819),
                insert_test_setlist(&conn, 8, 819)
            ];

            assert_eq!(repository.count_all(&conn).unwrap(), 2);
            assert_eq!(repository.find_all(&conn).unwrap(), inserted_setlists);
        })
    }

    #[test]
    fn test_find_by_id() {
        run_database_test(|conn| {
            clear_database(&conn);

            let random_id = rand::random::<i32>();
            insert_test_setlist(&conn, random_id, 819);

            let repository = UserSetlistRepository::new();
            assert_eq!(repository.find_by_id(&conn, random_id).unwrap().id, random_id);
        })
    }

    #[test]
    fn test_count_all() {
        run_database_test(|conn| {
            clear_database(&conn);
            let repository = UserSetlistRepository::new();
            assert_eq!(repository.count_all(&conn).unwrap(), 0);

            insert_test_setlist(&conn, 1, 819);
            insert_test_setlist(&conn, 2, 819);
            insert_test_setlist(&conn, 3, 819);
            insert_test_setlist(&conn, 4, 819);

            assert_eq!(repository.count_all(&conn).unwrap(), 4);
        })
    }

    #[test]
    fn test_add_empty() {
        run_database_test(|conn| {
            clear_database(&conn);

            let init_setlists = SetlistDb::all(&conn);
            let init_setlist_entries = SetlistDbEntry::all(&conn);

            let new_setlist = UserSetlist {
                id: 918,
                user: 819,
                user_name: "Paul".to_string(),
                entries: vec![],
            };

            UserSetlistRepository::new()
                .add(&conn, new_setlist)
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

            let new_setlist = UserSetlist {
                id: 918,
                user: 819,
                user_name: "Paul".to_string(),
                entries: vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1"),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2"),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3"),
                ],
            };

            UserSetlistRepository::new()
                .add(&conn, new_setlist)
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
            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            UserSetlistRepository::new()
                .update(
                    &conn,
                    UserSetlist {
                        id: 918,                       // Same ID
                        user: 8190,                    // New User
                        user_name: "Paul".to_string(), // New name
                        entries: vec![],
                    },
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
            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            UserSetlistRepository::new()
                .update(
                    &conn,
                    UserSetlist {
                        id: 918,                       // Same ID
                        user: 8190,                    // New User
                        user_name: "Paul".to_string(), // New name
                        entries: vec![SetlistEntry::new("song-4", FileType::Chorddown, "Song 4")],
                    },
                )
                .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 4);
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            insert_test_setlist(&conn, 918, 819);
            insert_test_setlist(&conn, 1918, 1819);

            UserSetlistRepository::new()
                .delete(
                    &conn,
                    UserSetlist {
                        id: 918, // This is important
                        user: 0,
                        user_name: "".to_string(),
                        entries: vec![],
                    },
                )
                .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 1);
        })
    }

    fn insert_test_setlist(conn: &ConnectionType, id: i32, user: i32) -> UserSetlist {
        let setlist = UserSetlist {
            id,
            user,
            user_name: "Saul".to_string(),
            entries: vec![
                SetlistEntry::new("song-1", FileType::Chorddown, "Song 1"),
                SetlistEntry::new("song-2", FileType::Chorddown, "Song 2"),
                SetlistEntry::new("song-3", FileType::Chorddown, "Song 3"),
            ],
        };
        CommandExecutor::perform(
            &setlist,
            Command::add(conn),
        )
            .unwrap();

        setlist
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
