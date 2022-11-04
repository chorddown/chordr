use diesel::{self, prelude::*};

use cqrs::prelude::{Command, CommandExecutor, Count};
use libchordr::prelude::RecordTrait;
use tri::Tri;

use crate::diesel::QueryDsl;
use crate::domain::user::command::UserCommandExecutor;
use crate::domain::user::UserDb;
use crate::error::SrvError;
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;
use crate::ConnectionType;

pub struct UserRepository<'a> {
    connection: &'a ConnectionType,
}

impl<'a> UserRepository<'a> {
    pub fn new(connection: &'a ConnectionType) -> Self {
        Self { connection }
    }

    pub fn find_by_name<S: AsRef<str>>(&self, username: S) -> Result<UserDb, SrvError> {
        Ok(all_users
            .filter(crate::schema::user::username.eq(username.as_ref()))
            .first(self.connection)?)
    }

    fn get_command_executor(&self, connection: &'a ConnectionType) -> UserCommandExecutor<'a> {
        UserCommandExecutor::new_with_connection(connection)
    }
}

impl<'a> cqrs::prelude::RepositoryTrait for UserRepository<'a> {
    type ManagedType = UserDb;
    type Error = SrvError;

    fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error> {
        Ok(all_users
            .order(user::username.desc())
            .load(self.connection)?)
    }

    fn count_all(&self) -> Result<Count, Self::Error> {
        Ok(all_users.count().get_result(self.connection)?)
    }

    fn find_by_id(&self, id: <UserDb as RecordTrait>::Id) -> Tri<Self::ManagedType, Self::Error> {
        match all_users.find(id).get_result::<UserDb>(self.connection) {
            Ok(o) => Tri::Some(o),
            Err(e) => Tri::Err(e.into()),
        }
    }

    fn save(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(&Command::upsert(instance, ()))
    }

    fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(&Command::add(instance, ()))
    }

    fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(&Command::update(instance, ()))
    }

    fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.get_command_executor(self.connection)
            .perform(&Command::delete(instance, ()))
    }
}
