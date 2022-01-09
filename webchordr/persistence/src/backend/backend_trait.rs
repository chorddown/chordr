use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use cqrs::prelude::{Command, CommandType, Query};
use libchordr::prelude::RecordTrait;
use webchordr_common::tri::Tri;

use crate::errors::WebError;
use crate::persistence_manager::CommandContext;

/// Trait for a persistent data Backend.
///
/// It will take care of storing and loading data from it's Storage (e.g. Browser Storage,
/// Server API)
#[async_trait(? Send)]
pub trait BackendTrait {
    /// Store `value` with the given `key` in the `namespace`
    ///
    /// `value` will be serialized before it is stored
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError>;

    /// Load the stored value with the given `key` in the `namespace`
    async fn load<T, N: AsRef<str>, K: AsRef<str>>(&self, namespace: N, key: K) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>;
}

#[async_trait(? Send)]
pub trait CommandBackendTrait {
    async fn perform<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        match command.command_type() {
            CommandType::Add => self.add(command).await,
            CommandType::Update => self.update(command).await,
            CommandType::Delete => self.delete(command).await,
        }
    }

    async fn add<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError>;

    async fn update<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError>;

    async fn delete<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError>;
}

#[async_trait(? Send)]
pub trait QueryBackendTrait {
    async fn find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>;

    async fn find_by_id<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>;
}

pub trait CommandQueryBackendTrait: BackendTrait + CommandBackendTrait + QueryBackendTrait {}
