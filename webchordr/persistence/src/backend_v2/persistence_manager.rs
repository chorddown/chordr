//! Persistence Manager v2
//!
//! The second iteration of the Persistence Manager manages a specific Record Type.
//! It keeps two sets of backends: a collection of `CommandExecutor` backends and a list of
//! `QueryExecutor` backends.
//!
//! `Command`s will be sent to all registered `CommandExecutor`s and the Persistence Manager will
//! wait for them to complete (although the result is currently discarded, unless every backend
//! failed).
//!
//! `Query`s will also be sent to all `QueryExecutor` backends. The first successful (non-error)
//! result will be returned.
use crate::command_context::CommandContext;
use async_trait::async_trait;
use cqrs::nonblocking::{BackendTrait, CommandExecutor, QueryExecutor};
use cqrs::prelude::{Command, Query, RecordTrait};
use log::{debug, warn};
use serde::de::DeserializeOwned;
use serde::Serialize;
use webchordr_common::errors::{PersistenceError, WebError};
use webchordr_common::tri::Tri;

pub trait RecordType: RecordTrait + Serialize + DeserializeOwned {}

pub(crate) type CE<R> =
    dyn CommandExecutor<RecordType = R, Error = WebError, Context = CommandContext>;
pub(crate) type QE<R> =
    dyn QueryExecutor<RecordType = R, Error = WebError, Context = CommandContext>;

pub struct PersistenceManagerV2<R: RecordTrait + Serialize + DeserializeOwned> {
    command_backends: Vec<Box<CE<R>>>,
    query_backends: Vec<Box<QE<R>>>,
}

impl<R: RecordTrait + Serialize + DeserializeOwned> PersistenceManagerV2<R> {
    pub fn with_backends(
        command_backends: Vec<
            Box<dyn CommandExecutor<RecordType = R, Error = WebError, Context = CommandContext>>,
        >,
        query_backends: Vec<
            Box<dyn QueryExecutor<RecordType = R, Error = WebError, Context = CommandContext>>,
        >,
    ) -> Self {
        Self {
            command_backends,
            query_backends,
        }
    }

    async fn forward_command(&self, command: &Command<R, CommandContext>) -> Result<(), WebError> {
        let mut errors: Vec<WebError> = vec![];
        for command_backend in &self.command_backends {
            let r = command_backend.perform(command).await;
            match r {
                Ok(_) => {}
                Err(e) => {
                    warn!("{}", e);
                    errors.push(e);
                }
            }
        }

        if errors.len() < self.command_backends.len() {
            Ok(())
        } else {
            Err(PersistenceError::backend_error(
                format!(
                    "No backend could execute the {} command successfully",
                    command.command_type()
                ),
                errors,
            )
            .into())
        }
    }
}

impl<R: RecordTrait + Serialize + DeserializeOwned> BackendTrait<R, WebError, CommandContext>
    for PersistenceManagerV2<R>
{
}

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> QueryExecutor for PersistenceManagerV2<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn find_all(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error> {
        let mut errors: Vec<WebError> = vec![];
        for query_backend in &self.query_backends {
            let r = query_backend.find_all(query).await;
            match r {
                Ok(v) => return Ok(v),
                Err(e) => {
                    debug!("{}", e);
                    errors.push(e)
                }
            }
        }
        Err(
            PersistenceError::backend_error("No backend could run the query successfully", errors)
                .into(),
        )
    }

    async fn find_by_id(
        &self,
        query: &Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error> {
        let mut errors = vec![];
        for query_backend in &self.query_backends {
            let r = query_backend.find_by_id(query).await;
            match r {
                Tri::Some(s) => return Tri::Some(s),
                Tri::None => return Tri::None,
                Tri::Err(e) => {
                    debug!("{}", e);
                    errors.push(e)
                }
            }
        }

        Tri::Err(
            PersistenceError::backend_error("No backend could run the query successfully", errors)
                .into(),
        )
    }
}

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> CommandExecutor for PersistenceManagerV2<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn upsert(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.forward_command(command).await
    }

    async fn add(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.forward_command(command).await
    }

    async fn update(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.forward_command(command).await
    }

    async fn delete(
        &self,
        command: &Command<Self::RecordType, Self::Context>,
    ) -> Result<(), Self::Error> {
        self.forward_command(command).await
    }
}
