use diesel::{self, prelude::*};

use cqrs::prelude::{CommandExecutor, Count};
use libchordr::prelude::RecordTrait;
use tri::Tri;

use crate::diesel::QueryDsl;
use crate::domain::user::command::UserCommandExecutor;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;
use crate::ConnectionType;

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
        UserCommandExecutor::new_with_connection(connection)
    }
}

impl cqrs::prelude::RepositoryTrait for UserRepository {
    type ManagedType = UserDb;
    type Error = SrvError;
    type Context = ConnectionType;

    fn find_all(&self, context: &ConnectionType) -> Result<Vec<Self::ManagedType>, Self::Error> {
        Ok(all_users.order(user::username.desc()).load(context)?)
    }

    fn count_all(&self, context: &ConnectionType) -> Result<Count, Self::Error> {
        Ok(all_users.count().get_result(context)?)
    }

    fn find_by_id(
        &self,
        context: &ConnectionType,
        id: <UserDb as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error> {
        match all_users.find(id).get_result::<UserDb>(context) {
            Ok(o) => Tri::Some(o),
            Err(e) => Tri::Err(e.into()),
        }
    }

    fn add(
        &self,
        context: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(context)
            .perform(cqrs::prelude::Command::add(instance, ()))
    }

    fn update(
        &self,
        context: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(context)
            .perform(cqrs::prelude::Command::update(instance, ()))
    }

    fn delete(
        &self,
        context: &ConnectionType,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.get_command_executor(context)
            .perform(cqrs::prelude::Command::delete(
                RecordTrait::id(&instance),
                (),
            ))
    }
}
