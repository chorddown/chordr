use crate::diesel::QueryDsl;
use crate::domain::cqs_context::CqsContext;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user::dsl::user as all_users;
use crate::ConnectionType;
use cqrs::prelude::Command;
use diesel::{self, prelude::*};

pub(crate) struct UserCommandExecutor<'a> {
    connection: &'a ConnectionType,
}

impl<'a> UserCommandExecutor<'a> {
    pub(crate) fn new_with_connection(connection: &'a ConnectionType) -> Self {
        Self { connection }
    }
}

impl<'a> cqrs::prelude::CommandExecutor for UserCommandExecutor<'_> {
    type RecordType = UserDb;
    type Error = SrvError;
    type Context = CqsContext;

    fn upsert(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        let user = command.record();
        let user_query = all_users.find(Identifiable::id(user));
        if let Ok(1) = user_query.count().get_result::<i64>(self.connection) {
            self.update(command)
        } else {
            self.add(command)
        }
    }

    fn add(&self, command: &Command<Self::RecordType, CqsContext>) -> Result<(), Self::Error> {
        diesel::insert_into(crate::schema::user::table)
            .values(command.record())
            .execute(self.connection)?;
        Ok(())
    }

    fn update(&self, command: &Command<Self::RecordType, CqsContext>) -> Result<(), Self::Error> {
        let user = command.record();
        let user_query = all_users.find(diesel::Identifiable::id(user));
        if user_query.get_result::<UserDb>(self.connection).is_err() {
            return Err(SrvError::persistence_error(format!(
                "Original object with ID '{}' could not be found",
                diesel::Identifiable::id(user)
            )));
        }

        diesel::update(user_query)
            .set(user)
            .execute(self.connection)?;

        Ok(())
    }

    fn delete(&self, command: &Command<Self::RecordType, CqsContext>) -> Result<(), Self::Error> {
        diesel::delete(all_users.find(&command.record().username)).execute(self.connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use cqrs::prelude::{Command, CommandExecutor, Count};

    use crate::domain::user::UserDb;
    use crate::test_helpers::*;
    use crate::ConnectionType;

    use super::*;

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

            CommandExecutor::perform(
                &UserCommandExecutor::new_with_connection(&conn),
                &Command::add(new_user, ()),
            )
            .unwrap();

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
                &UserCommandExecutor::new_with_connection(&conn),
                &Command::update(
                    UserDb {
                        username: "saul-panther-918".to_string(), // Same username
                        first_name: "Paul".to_string(),           // New name
                        last_name: "Panther".to_string(),
                        password_hash: "123456".to_string(),
                    },
                    (),
                ),
            )
            .unwrap();

            assert_eq!(count_all_users(&conn), 2);
        })
    }
    #[test]
    fn test_upsert_add() {
        run_database_test(|conn| {
            clear_database(&conn);

            let new_user = UserDb {
                username: "superhacker-918".to_string(),
                first_name: "Super".to_string(),
                last_name: "Hacker".to_string(),
                password_hash: "123456".to_string(),
            };

            CommandExecutor::perform(
                &UserCommandExecutor::new_with_connection(&conn),
                &Command::upsert(new_user, ()),
            )
            .unwrap();

            assert_eq!(count_all_users(&conn), 1);
        })
    }
    #[test]
    fn test_upsert_update() {
        run_database_test(|conn| {
            clear_database(&conn);
            insert_test_user(&conn, "saul-panther-918", "Saul", "Panther");
            insert_test_user(&conn, "roger-mulliger-8", "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            CommandExecutor::perform(
                &UserCommandExecutor::new_with_connection(&conn),
                &Command::upsert(
                    UserDb {
                        username: "saul-panther-918".to_string(), // Same username
                        first_name: "Paul".to_string(),           // New name
                        last_name: "Panther".to_string(),
                        password_hash: "123456".to_string(),
                    },
                    (),
                ),
            )
            .unwrap();

            assert_eq!(count_all_users(&conn), 2);
        })
    }

    #[test]
    fn test_delete() {
        run_database_test(|conn| {
            clear_database(&conn);

            let username = "saul-panther-918";
            insert_test_user(&conn, username, "Saul", "Panther");
            insert_test_user(&conn, "roger-mulliger-8", "Roger", "Mulliger");
            assert_eq!(count_all_users(&conn), 2);

            let user_to_delete = UserDb {
                username: username.to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                password_hash: "".to_string(),
            };

            CommandExecutor::perform(
                &UserCommandExecutor::new_with_connection(&conn),
                &Command::delete(user_to_delete, ()),
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
