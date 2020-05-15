use crate::command::{Command, CommandExecutor};
use crate::diesel::QueryDsl;
use crate::domain::user::User;
use crate::error::SrvError;
use crate::schema::user::dsl::user as all_users;
use diesel::{self, prelude::*};

impl CommandExecutor for &User {
    type Error = SrvError;

    fn add(self, command: Command) -> Result<(), Self::Error> {
        diesel::insert_into(crate::schema::user::table)
            .values(self)
            .execute(command.connection)?;
        Ok(())
    }

    fn update(self, command: Command) -> Result<(), Self::Error> {
        let user_query = all_users.find(self.id);
        if let Err(_) = user_query.get_result::<User>(command.connection) {
            return Err(SrvError::persistence_error(format!(
                "Original object with ID '{}' could not be found",
                self.id
            )));
        }

        diesel::update(user_query)
            .set(self)
            .execute(command.connection)?;

        Ok(())
    }

    fn delete(self, command: Command) -> Result<(), Self::Error> {
        diesel::delete(all_users.find(self.id)).execute(command.connection)?;
        Ok(())
    }
}

impl CommandExecutor for User {
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
    use crate::domain::user::User;
    use crate::test_helpers::*;
    use crate::traits::Count;
    use crate::ConnectionType;

    #[test]
    fn test_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let new_user = User {
                id: 918,
                username: "superhacker".to_string(),
                first_name: "Super".to_string(),
                last_name: "Hacker".to_string(),
                password: "123456".to_string(),
            };

            CommandExecutor::perform(new_user, Command::add(&conn)).unwrap();

            assert_eq!(count_all_users(&conn), 1);
        })
    }

    #[test]
    fn test_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            insert_test_user(&conn, 918, "Saul", "Panther");
            insert_test_user(&conn, 8, "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            CommandExecutor::perform(
                User {
                    id: 918,                               // Same ID
                    username: "paul-panther".to_string(), // New username
                    first_name: "Paul".to_string(),        // New name
                    last_name: "Panther".to_string(),
                    password: "123456".to_string(),
                },
                Command::update(&conn),
            )
                .unwrap();

            assert_eq!(count_all_users(&conn), 2);
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            insert_test_user(&conn, 918, "Saul", "Panther");
            insert_test_user(&conn, 8, "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            CommandExecutor::perform(
                User {
                    id: 918, // This is important
                    username: "".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    password: "".to_string(),
                },
                Command::delete(&conn),
            )
                .unwrap();

            assert_eq!(count_all_users(&conn), 1);
        })
    }

    fn insert_test_user<S: Into<String>>(
        conn: &ConnectionType,
        id: i32,
        first_name: S,
        last_name: S,
    ) -> User {
        let first_name = first_name.into();
        let last_name = last_name.into();
        let new_user = User {
            id,
            username: format!("{}-{}", first_name, last_name),
            first_name,
            last_name,
            password: "123456".to_string(),
        };

        CommandExecutor::perform(&new_user, Command::add(conn)).unwrap();

        new_user
    }

    fn count_all_users(conn: &ConnectionType) -> Count {
        all_users.count().get_result(conn).unwrap()
    }

    fn clear_database(conn: &ConnectionType) {
        assert!(
            diesel::delete(all_users).execute(conn).is_ok(),
            "Failed to delete all data before testing"
        );
    }
}
