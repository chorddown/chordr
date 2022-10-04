use diesel::{self, prelude::*};

use cqrs::prelude::{CommandExecutor, Count, RepositoryTrait};
use libchordr::prelude::{RecordTrait, Setlist, Team, User, Username};
use tri::Tri;

use crate::diesel::QueryDsl;
use crate::domain::setlist::command::SetlistCommandExecutor;
use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist::setlist_from_data;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::domain::user::repository::UserRepository;
use crate::error::SrvError;
use crate::schema::setlist;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::ConnectionType;

pub struct SetlistRepository<'a> {
    connection: &'a ConnectionType,
}

type PopulateResult = (SetlistDb, Vec<SetlistDbEntry>);

impl<'a> SetlistRepository<'a> {
    pub fn new(connection: &'a ConnectionType) -> Self {
        Self { connection }
    }

    /// Return all [`Setlist`]'s for the given [`Username`]
    pub fn find_by_username(&self, username: &Username) -> Result<Vec<Setlist>, SrvError> {
        let search = all_setlists
            .order(crate::schema::setlist::sorting.asc())
            .filter(crate::schema::setlist::owner.eq(&username.to_string()))
            .load::<SetlistDb>(self.connection)?;

        let owner = UserRepository::new(self.connection)
            .find_by_name(username.to_string())?
            .try_to_user()?;
        let populated_entries = self.populate_entries(search)?;

        self.build_setlist(populated_entries, owner)
    }

    /// Return the [`Setlist`] with `setlist_id` for the given [`Username`]
    pub fn find_by_username_and_setlist_id(
        &self,
        username: &Username,
        setlist_id: i32,
    ) -> Result<Setlist, SrvError> {
        let sl = all_setlists
            .filter(crate::schema::setlist::owner.eq(username.as_ref()))
            .filter(crate::schema::setlist::id.eq(setlist_id))
            .first::<SetlistDb>(self.connection)?;

        let entries = SetlistDbEntry::find_by_setlist(self.connection, &sl)?;
        let owner = self.get_user(username)?;
        // TODO: Add support for Teams
        let team: Option<Team> = None;

        Ok(setlist_from_data(sl, entries, owner, team))
    }

    // Add `find_by_user`?
    // The question is what happens with the given user if the user-data in the database changed?
    //
    // /// Return all [`Setlist`]'s for the given [`User`]
    // pub fn find_by_user(
    //     &self,
    //     _connection: &ConnectionType,
    //     owner: &User,
    // ) -> Result<Vec<Setlist>, SrvError> {
    //     let search = all_setlists
    //         .order(setlist::sorting.asc())
    //         .filter(crate::schema::setlist::owner.eq(&owner.username().to_string()))
    //         .load::<SetlistDb>(self.connection)?;
    //
    //     let populated_entries = self.populate_entries(self.connection, search)?;
    //
    //     self.build_setlist(populated_entries, owner)
    // }

    fn build_setlist(
        &self,
        populated_entries: Vec<(SetlistDb, Vec<SetlistDbEntry>)>,
        owner: User,
    ) -> Result<Vec<Setlist>, SrvError> {
        Ok(populated_entries
            .into_iter()
            .map(|(setlist_db, entries)| {
                let team = match setlist_db.team {
                    // TODO: Add support for Teams
                    Some(_) => unimplemented!("Load teams"),
                    None => None,
                };

                setlist_from_data(setlist_db, entries, owner.clone(), team)
            })
            .collect::<Vec<Setlist>>())
    }

    fn populate_entries(&self, setlists: Vec<SetlistDb>) -> Result<Vec<PopulateResult>, SrvError> {
        let entries: Vec<SetlistDbEntry> =
            <SetlistDbEntry as BelongingToDsl<&Vec<SetlistDb>>>::belonging_to(&setlists)
                .load(self.connection)?;
        let grouped_entries: Vec<Vec<SetlistDbEntry>> = entries.into_iter().grouped_by(&setlists);

        Ok(setlists.into_iter().zip(grouped_entries).collect())
    }

    fn get_user(&self, username: &Username) -> Result<User, SrvError> {
        UserRepository::new(self.connection)
            .find_by_name(username.to_string())?
            .try_to_user()
    }

    fn get_users(&self) -> Result<Vec<User>, SrvError> {
        let raw_users = UserRepository::new(self.connection).find_all()?;
        let users = raw_users
            .into_iter()
            .filter_map(|u| u.try_to_user().ok())
            .collect();
        Ok(users)
    }

    fn get_command_executor(&self, connection: &'a ConnectionType) -> SetlistCommandExecutor<'a> {
        SetlistCommandExecutor::new_with_connection(connection)
    }
}

impl<'a> cqrs::prelude::RepositoryTrait for SetlistRepository<'a> {
    type ManagedType = Setlist;
    type Error = SrvError;

    fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error> {
        let search = all_setlists
            .order(setlist::sorting.asc())
            .load::<SetlistDb>(self.connection)?;
        let populated_entries: Vec<PopulateResult> = self.populate_entries(search)?;

        let users = self.get_users()?;

        Ok(populated_entries
            .into_iter()
            .map(|x| assign_owner_to_populated_result(x, &users).unwrap())
            .collect())
    }

    fn count_all(&self) -> Result<Count, Self::Error> {
        Ok(all_setlists.count().get_result(self.connection)?)
    }

    fn find_by_id(&self, _id: <Setlist as RecordTrait>::Id) -> Tri<Self::ManagedType, Self::Error> {
        unimplemented!("The ID of the Setlist and the UID of the SetlistDb are not the same -> find_by_id() cannot be implemented");
    }

    fn save(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(cqrs::prelude::Command::upsert(instance, ()))
    }

    fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(cqrs::prelude::Command::add(instance, ()))
    }

    fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(cqrs::prelude::Command::update(instance, ()))
    }

    fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(cqrs::prelude::Command::delete(instance, ()))
    }
}

fn assign_owner_to_populated_result(
    populate_entry: PopulateResult,
    users: &[User],
) -> Result<Setlist, SrvError> {
    let setlist_db = populate_entry.0;

    let team = match setlist_db.team {
        // TODO: Add support for Teams
        Some(_) => unimplemented!("Load teams"),
        None => None,
    };

    let owner = users
        .iter()
        .find(|user| user.username().as_ref() == setlist_db.owner);

    match owner {
        Some(owner) => Ok(setlist_from_data(
            setlist_db,
            populate_entry.1,
            owner.clone(),
            team,
        )),
        None => {
            println!("{:#?}", users);
            Err(SrvError::object_not_found_error(format!(
                "User '{}' could not be found",
                setlist_db.owner
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use rocket::form::validate::Contains;

    use cqrs::prelude::RepositoryTrait;
    use libchordr::models::file_type::FileType;
    use libchordr::prelude::{Setlist, SetlistEntry, User, Username};

    use crate::domain::setlist::db::SetlistDb;
    use crate::domain::setlist_entry::db::SetlistDbEntry;
    use crate::test_helpers::*;

    use super::*;

    #[test]
    fn test_find_all() {
        run_database_test(|conn| {
            clear_database(&conn);
            let repository = SetlistRepository::new(&conn);
            assert_eq!(repository.count_all().unwrap(), 0);
            assert_eq!(repository.find_all().unwrap(), vec![]);

            insert_test_user(&conn, "user-819", "Saul", "Doe");
            let inserted_setlists = vec![
                create_setlist(&conn, 8, "user-819"),
                create_setlist(&conn, 918, "user-819"),
            ];

            assert_eq!(repository.count_all().unwrap(), 2);
            assert_eq!(repository.find_all().unwrap(), inserted_setlists);
        })
    }

    #[test]
    fn test_find_by_id() {
        run_database_test(|conn| {
            clear_database(&conn);

            // The ID of the Setlist and the UID of the SetlistDb are not the same -> find_by_id() cannot be implemented

            // let random_id = rand::random::<i32>();
            // insert_test_user(&conn, "user-819", "Saul", "Doe");
            // create_setlist(&conn, random_id, "user-819");
            //
            // let repository = SetlistRepository::new(&conn);
            // assert_eq!(repository.find_by_id(random_id).unwrap().id(), random_id);
        })
    }

    #[test]
    fn test_find_by_username() {
        run_database_test(|conn| {
            clear_database(&conn);

            let random_id = rand::random::<u16>() as i32;
            let random_user_id = rand::random::<i32>();
            let user_id_string = format!("{}", random_user_id);
            let user_db = insert_test_user(&conn, &user_id_string, "Saul", "Doe");
            let user = user_db.try_to_user().unwrap();

            let sl1 = create_setlist(&conn, random_id + 100, &user_id_string);
            let sl2 = create_setlist(&conn, random_id + 200, &user_id_string);

            let repository = SetlistRepository::new(&conn);
            let setlists_result = repository.find_by_username(&user.username());
            assert!(setlists_result.is_ok());
            let setlists = setlists_result.unwrap();
            assert_eq!(setlists.len(), 2);

            assert!(setlists.contains(sl1), "{:?}", setlists);
            assert!(setlists.contains(sl2), "{:?}", setlists);
        })
    }

    #[test]
    fn test_find_by_username_and_setlist_id() {
        run_database_test(|conn| {
            clear_database(&conn);

            let random_id = rand::random::<u16>() as i32;
            let random_user_id = rand::random::<i32>();
            let user_id_string = format!("{}", random_user_id);
            insert_test_user(&conn, &user_id_string, "Saul", "Doe");
            let sl1 = create_setlist(&conn, random_id + 100, &user_id_string);
            let _sl2 = create_setlist(&conn, random_id + 200, &user_id_string);

            let repository = SetlistRepository::new(&conn);
            let setlist_result = repository.find_by_username_and_setlist_id(
                &Username::new(user_id_string).unwrap(),
                Setlist::id(&sl1),
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
            let repository = SetlistRepository::new(&conn);
            assert_eq!(repository.count_all().unwrap(), 0);
            insert_test_user(&conn, "819", "Saul", "Doe");

            create_setlist(&conn, 1, "819");
            create_setlist(&conn, 2, "819");
            create_setlist(&conn, 3, "819");
            create_setlist(&conn, 4, "819");

            assert_eq!(repository.count_all().unwrap(), 4);
        })
    }

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

            SetlistRepository::new(&conn).add(new_setlist).unwrap();

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

            SetlistRepository::new(&conn).add(new_setlist).unwrap();

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
            create_setlist(&conn, 918, "819");
            create_setlist(&conn, 8, "819");

            assert_eq!(SetlistDbEntry::count_all(&conn), 6);

            let empty_setlist = Setlist::new(
                "My setlist #918",
                918, // Same ID
                // Same User:
                User::new(
                    Username::new("819").unwrap(),
                    "Saul",
                    "Doe",
                    create_test_password(),
                ),
                None,
                None,
                Utc::now(),
                Utc::now(),
                vec![],
            );
            let result = SetlistRepository::new(&conn).update(empty_setlist);

            assert!(result.is_ok(), "{}", result.unwrap_err());

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

            SetlistRepository::new(&conn)
                .update(Setlist::new(
                    "My setlist #918",
                    918, // Same ID
                    // Same User:
                    User::new(
                        Username::new("819").unwrap(),
                        "Saul",
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
                ))
                .unwrap();

            assert_eq!(SetlistDb::count_all(&conn), 2);
            assert_eq!(SetlistDbEntry::count_all(&conn), 4);
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            create_setlist(&conn, 918, "user-819");
            create_setlist(&conn, 1918, "user-1819");

            SetlistRepository::new(&conn)
                .delete(Setlist::new(
                    "My setlist #918",
                    918,                          // This is important
                    create_test_user("user-819"), // This is important
                    None,
                    None,
                    Utc::now(),
                    Utc::now(),
                    vec![],
                ))
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
