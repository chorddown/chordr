use crate::command::{Command, CommandExecutor};
use crate::diesel::QueryDsl;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;
use crate::traits::*;
use crate::ConnectionType;
use diesel::{self, prelude::*};
use libchordr::prelude::RecordIdTrait;

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
        id: <UserDb as RecordIdTrait>::Id,
    ) -> Result<Self::ManagedType, Self::Error> {
        Ok(all_users.find(id).get_result::<UserDb>(connection)?)
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
