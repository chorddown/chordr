use crate::diesel::QueryDsl;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user::dsl::user as all_users;
use crate::ConnectionType;
use diesel::{self, prelude::*};

#[allow(deprecated)]
impl crate::command::CommandExecutor for &UserDb {
    type Error = SrvError;

    fn add(self, command: crate::command::Command) -> Result<(), Self::Error> {
        diesel::insert_into(crate::schema::user::table)
            .values(self)
            .execute(command.connection)?;
        Ok(())
    }

    fn update(self, command: crate::command::Command) -> Result<(), Self::Error> {
        let user_query = all_users.find(&self.username);
        if let Err(_) = user_query.get_result::<UserDb>(command.connection) {
            return Err(SrvError::persistence_error(format!(
                "Original object with ID '{}' could not be found",
                self.username
            )));
        }

        diesel::update(user_query)
            .set(self)
            .execute(command.connection)?;

        Ok(())
    }

    fn delete(self, command: crate::command::Command) -> Result<(), Self::Error> {
        diesel::delete(all_users.find(&self.username)).execute(command.connection)?;
        Ok(())
    }
}

#[allow(deprecated)]
impl crate::command::CommandExecutor for UserDb {
    type Error = SrvError;

    fn add(self, command: crate::command::Command) -> Result<(), Self::Error> {
        crate::command::CommandExecutor::add(&self, command)
    }

    fn update(self, command: crate::command::Command) -> Result<(), Self::Error> {
        crate::command::CommandExecutor::update(&self, command)
    }

    fn delete(self, command: crate::command::Command) -> Result<(), Self::Error> {
        crate::command::CommandExecutor::delete(&self, command)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::user::UserDb;
    use crate::test_helpers::*;
    use crate::traits::Count;
    use crate::ConnectionType;

    #[test]
    fn test_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let new_user = UserDb {
                username: "superhacker-918".to_string(),
                first_name: "Super".to_string(),
                last_name: "Hacker".to_string(),
                password_hash: "123456".to_string(),
            };

            CommandExecutor::perform(new_user, Command::add(&conn)).unwrap();

            assert_eq!(count_all_users(&conn), 1);
        })
    }

    #[test]
    fn test_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            insert_test_user(&conn, "saul-panther-918", "Saul", "Panther");
            insert_test_user(&conn, "roger-mulliger-8", "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            CommandExecutor::perform(
                UserDb {
                    username: "saul-panther-918".to_string(), // Same username
                    first_name: "Paul".to_string(),           // New name
                    last_name: "Panther".to_string(),
                    password_hash: "123456".to_string(),
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

            insert_test_user(&conn, "saul-panther-918", "Saul", "Panther");
            insert_test_user(&conn, "roger-mulliger-8", "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            CommandExecutor::perform(
                UserDb {
                    username: "saul-panther-918".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    password_hash: "".to_string(),
                },
                Command::delete(&conn),
            )
            .unwrap();

            assert_eq!(count_all_users(&conn), 1);
        })
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
