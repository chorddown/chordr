use crate::command::{Command, CommandExecutor};
use crate::diesel::QueryDsl;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist::UserSetlist;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::domain::user::User;
use crate::error::SrvError;
use crate::schema::setlist;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::traits::*;
use crate::ConnectionType;
use diesel::{self, prelude::*};

pub struct UserSetlistRepository {}

impl UserSetlistRepository {
    pub fn new() -> Self {
        Self {}
    }

    /// Return all [`UserSetlist`]'s for the given [`User`]
    pub fn find_by_user(
        &self,
        connection: &ConnectionType,
        user: &User,
    ) -> Result<Vec<UserSetlist>, SrvError> {
        self.populate_entries(
            connection,
            all_setlists
                .order(setlist::sorting.asc())
                .filter(crate::schema::setlist::user.eq(user.id))
                .load::<SetlistDb>(connection)?,
        )
    }

    /// Return the [`UserSetlist`] with `setlist_id` for the given [`User`]
    pub fn find_by_user_and_setlist_id(
        &self,
        connection: &ConnectionType,
        user: &User,
        setlist_id: i32,
    ) -> Result<UserSetlist, SrvError> {
        let sl = all_setlists
            .filter(crate::schema::setlist::user.eq(user.id))
            .filter(crate::schema::setlist::id.eq(setlist_id))
            .first::<SetlistDb>(connection)?;

        let entries = SetlistDbEntry::find_by_setlist(connection, &sl)?;

        Ok(UserSetlist::from_data(sl, entries))
    }

    fn populate_entries(
        &self,
        connection: &ConnectionType,
        setlists: Vec<SetlistDb>,
    ) -> Result<Vec<UserSetlist>, SrvError> {
        let entries: Vec<SetlistDbEntry> =
            SetlistDbEntry::belonging_to(&setlists).load(connection)?;
        let grouped_entries: Vec<Vec<SetlistDbEntry>> = entries.into_iter().grouped_by(&setlists);

        Ok(setlists
            .into_iter()
            .zip(grouped_entries)
            .map(|t| UserSetlist::from(t))
            .collect())
    }
}

impl RepositoryTrait for UserSetlistRepository {
    type ManagedType = UserSetlist;
    type Error = SrvError;

    fn find_all(&self, connection: &ConnectionType) -> Result<Vec<Self::ManagedType>, Self::Error> {
        self.populate_entries(
            connection,
            all_setlists
                .order(setlist::sorting.asc())
                .load::<SetlistDb>(connection)?,
        )
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
                create_setlist(&conn, 8, 819),
                create_setlist(&conn, 918, 819),
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
            create_setlist(&conn, random_id, 819);

            let repository = UserSetlistRepository::new();
            assert_eq!(
                repository.find_by_id(&conn, random_id).unwrap().id,
                random_id
            );
        })
    }

    #[test]
    fn test_find_by_user() {
        run_database_test(|conn| {
            clear_database(&conn);

            let random_id = rand::random::<u16>() as i32;
            let random_user_id = rand::random::<i32>();
            let sl1 = create_setlist(&conn, random_id + 100, random_user_id);
            let sl2 = create_setlist(&conn, random_id + 200, random_user_id);

            let repository = UserSetlistRepository::new();
            let setlists_result = repository.find_by_user(&conn, &build_test_user(random_user_id));
            assert!(setlists_result.is_ok());
            let setlists = setlists_result.unwrap();
            assert_eq!(setlists.len(), 2);
            assert_eq!(setlists[0], sl1);
            assert_eq!(setlists[1], sl2);
        })
    }

    #[test]
    fn test_find_by_user_and_setlist_id() {
        run_database_test(|conn| {
            clear_database(&conn);

            let random_id = rand::random::<u16>() as i32;
            let random_user_id = rand::random::<i32>();
            let sl1 = create_setlist(&conn, random_id + 100, random_user_id);
            let _sl2 = create_setlist(&conn, random_id + 200, random_user_id);

            let repository = UserSetlistRepository::new();
            let setlist_result = repository.find_by_user_and_setlist_id(
                &conn,
                &build_test_user(random_user_id),
                sl1.id,
            );
            assert!(setlist_result.is_ok());
            let setlist = setlist_result.unwrap();
            assert_eq!(setlist, sl1);
        })
    }

    #[test]
    fn test_count_all() {
        run_database_test(|conn| {
            clear_database(&conn);
            let repository = UserSetlistRepository::new();
            assert_eq!(repository.count_all(&conn).unwrap(), 0);

            create_setlist(&conn, 1, 819);
            create_setlist(&conn, 2, 819);
            create_setlist(&conn, 3, 819);
            create_setlist(&conn, 4, 819);

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
                sorting: 918,
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
                sorting: 918,
                entries: vec![
                    SetlistEntry::new("song-1", FileType::Chorddown, "Song 1", None),
                    SetlistEntry::new("song-2", FileType::Chorddown, "Song 2", None),
                    SetlistEntry::new("song-3", FileType::Chorddown, "Song 3", None),
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
            create_setlist(&conn, 918, 819);
            create_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            UserSetlistRepository::new()
                .update(
                    &conn,
                    UserSetlist {
                        id: 918,                       // Same ID
                        user: 8190,                    // New User
                        user_name: "Paul".to_string(), // New name
                        sorting: 918,
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
            create_setlist(&conn, 918, 819);
            create_setlist(&conn, 8, 819);

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            UserSetlistRepository::new()
                .update(
                    &conn,
                    UserSetlist {
                        id: 918,                       // Same ID
                        user: 8190,                    // New User
                        user_name: "Paul".to_string(), // New name
                        sorting: 918,
                        entries: vec![SetlistEntry::new("song-4", FileType::Chorddown, "Song 4", None)],
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

            create_setlist(&conn, 918, 819);
            create_setlist(&conn, 1918, 1819);

            UserSetlistRepository::new()
                .delete(
                    &conn,
                    UserSetlist {
                        id: 918, // This is important
                        user: 0,
                        user_name: "".to_string(),
                        sorting: 918,
                        entries: vec![],
                    },
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

    fn build_test_user(random_user_id: i32) -> User {
        User {
            id: random_user_id,
            username: "geoffrey".to_string(),
            first_name: "Geoffrey".to_string(),
            last_name: "Puppetham".to_string(),
            password: "$$$".to_string(),
        }
    }
}
