use crate::diesel::QueryDsl;
use crate::domain::user::command::UserCommandExecutor;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;
use crate::traits::*;
use crate::ConnectionType;
use diesel::{self, prelude::*};
use libchordr::prelude::RecordTrait;
use cqrs::prelude::CommandExecutor;

pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find_by_name<S: AsRef<str>>(
        &self,
        connection: &ConnectionType,
        username: S,
    ) -> Result<UserDb, SrvError> {
        Ok(all_users
            .filter(crate::schema::user::username.eq(username.as_ref()))
            .first(connection)?)
    }

    fn get_command_executor<'a>(&self, connection: &'a ConnectionType) -> UserCommandExecutor<'a> {
        UserCommandExecutor::with_connection(connection)
    }
}

impl RepositoryTrait for UserRepository {
    type ManagedType = UserDb;
    type Error = SrvError;

    fn find_all(&self, connection: &ConnectionType) -> Result<Vec<Self::ManagedType>, Self::Error> {
        Ok(all_users.order(user::username.desc()).load(connection)?)
    }

    fn count_all(&self, connection: &ConnectionType) -> Result<Count, Self::Error> {
        Ok(all_users.count().get_result(connection)?)
    }

    fn find_by_id(
        &self,
        connection: &ConnectionType,
        id: <UserDb as RecordTrait>::Id,
    ) -> Result<Self::ManagedType, Self::Error> {
        Ok(all_users.find(id).get_result::<UserDb>(connection)?)
    }

    fn add(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(connection)
            .perform(cqrs::prelude::Command::add(instance))
    }

    fn update(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(connection)
            .perform(cqrs::prelude::Command::update(instance))
    }

    fn delete(
        &self,
        connection: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(connection)
            .perform(cqrs::prelude::Command::delete(instance.id()))
    }
}
