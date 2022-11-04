use super::persistence_manager_trait::PersistenceManagerTrait;
use crate::backend::BackendTrait;
use crate::backend::CommandQueryBackendTrait;
use crate::errors::WebError;
use crate::lock::Stupex;
use crate::persistence_manager::command_context::CommandContext;
use crate::persistence_manager::server_backend_type::ServerBackendType;
use async_trait::async_trait;
use cqrs::prelude::{Command, Count, Query};
use libchordr::prelude::RecordTrait;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use webchordr_common::session::Session;
use webchordr_common::tri::Tri;

type Lock<I> = Stupex<I>;

#[deprecated(note = "Use V2 Backend")]
pub struct PersistenceManager<CB, SB, TB> {
    session: Session,
    client_backend: Arc<Lock<CB>>,
    server_backend: Arc<Lock<SB>>,
    transient_backend: Arc<Lock<TB>>,
}

// async fn backend_call<'r, F, R: 'r, Fu, B: BackendTrait>(
//     hint: &'static str,
//     backend: &'r Arc<Lock<B>>,
//     callback: F,
// ) -> R
// where
//     Fu: Future<Output = R>,
//     F: FnOnce(&B) -> Fu,
// {
//     let locked_backend = backend
//         .lock()
//         .await
//         .expect(&format!("Could not acquire lock for {}", hint));
//
//     callback(&*locked_backend).await
// }

impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    PersistenceManager<CB, SB, TB>
{
    pub fn new(
        session: Session,
        client_backend: CB,
        server_backend: SB,
        transient_backend: TB,
    ) -> Self {
        Self {
            session,
            client_backend: Arc::new(Lock::new(client_backend)),
            server_backend: Arc::new(Lock::new(server_backend)),
            transient_backend: Arc::new(Lock::new(transient_backend)),
        }
    }

    async fn perform_command<T: Serialize + RecordTrait>(
        &self,
        command: Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        if let Err(e) = self.client_command(&command).await {
            if self.session.is_unauthenticated() {
                return Err(e);
            } else {
                warn!("Client command failed: {}", e)
            }
        }

        if self.session.is_authenticated() {
            self.server_command(&command).await
        } else {
            Ok(())
        }
    }

    async fn server_store<T: Serialize + RecordTrait>(
        &self,
        namespace: &str,
        key: &str,
        value: &T,
    ) -> Result<(), WebError> {
        if self.session.is_unauthenticated() {
            return Ok(());
        }

        match ServerBackendType::from_context(&CommandContext::new(namespace, key)) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .store(namespace, key, value)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .store(namespace, key, value)
                    .await
            }
        }
    }

    async fn server_load<T>(&self, namespace: &str, key: &str) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        match ServerBackendType::from_context(&CommandContext::new(namespace, key)) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .load(namespace, key)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .load(namespace, key)
                    .await
            }
        }
    }

    async fn server_command<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        if self.session.is_unauthenticated() {
            return Ok(());
        }

        match ServerBackendType::from_context(command.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .perform(command)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .perform(command)
                    .await
            }
        }
    }

    async fn server_find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.session.is_unauthenticated() {
            return Ok(vec![]);
        }

        match ServerBackendType::from_context(query.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (server backend)")
                    .find_all(query)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for writing (transient server backend)")
                    .find_all(query)
                    .await
            }
        }
    }

    async fn server_find_by_id<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        assert!(self.session.is_authenticated());

        match ServerBackendType::from_context(query.context()) {
            ServerBackendType::Server => {
                self.server_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (server backend)")
                    .find_by_id(query)
                    .await
            }
            ServerBackendType::Transient => {
                self.transient_backend
                    .lock()
                    .await
                    .expect("Could not acquire lock for reading (transient server backend)")
                    .find_by_id(query)
                    .await
            }
        }
    }

    async fn client_store<T: Serialize + RecordTrait>(
        &self,
        namespace: &str,
        key: &str,
        value: &T,
    ) -> Result<(), WebError> {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .store(namespace, key, value)
            .await
    }

    async fn client_command<T: Serialize + RecordTrait>(
        &self,
        command: &Command<T, CommandContext>,
    ) -> Result<(), WebError> {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .perform(command)
            .await
    }

    async fn client_find_all<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .find_all(query)
            .await
    }

    async fn client_find_by_id<T: RecordTrait>(
        &self,
        query: &Query<T, CommandContext>,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for writing (client backend)")
            .find_by_id(query)
            .await
    }
}

#[async_trait(? Send)]
impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    BackendTrait for PersistenceManager<CB, SB, TB>
{
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        // TODO: use join!
        if let Err(e) = self
            .client_store(namespace.as_ref(), key.as_ref(), value)
            .await
        {
            error!("{}", e)
        }

        self.server_store(namespace.as_ref(), key.as_ref(), value)
            .await
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(&self, namespace: N, key: K) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        if self.session.is_authenticated() {
            let server_result = self.server_load(namespace.as_ref(), key.as_ref()).await;
            if let Tri::Err(e) = server_result {
                warn!("{}", e)
            } else {
                return server_result;
            }
        }
        self.client_backend
            .lock()
            .await
            .expect("Could not acquire lock for reading (client backend)")
            .load(namespace.as_ref(), key.as_ref())
            .await
    }
}

#[async_trait(? Send)]
impl<CB: CommandQueryBackendTrait, SB: CommandQueryBackendTrait, TB: CommandQueryBackendTrait>
    PersistenceManagerTrait for PersistenceManager<CB, SB, TB>
{
    async fn find_all<T>(&self, context: CommandContext) -> Result<Vec<T>, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait,
    {
        let query = Query::all(context);

        if self.session.is_authenticated() {
            let server_result = self.server_find_all(&query).await;

            if let Err(e) = server_result {
                warn!("{}", e);
            } else {
                return server_result;
            }
        }

        self.client_find_all(&query).await
    }

    async fn count_all(&self, context: CommandContext) -> Result<Count, WebError> {
        // No real type is needed to perform the counting. All data the backend needs is provided in
        // the Context
        #[derive(Serialize, Deserialize)]
        struct EmptyType {}
        impl RecordTrait for EmptyType {
            type Id = u8;

            fn id(&self) -> Self::Id {
                8
            }
        }

        self.find_all::<EmptyType>(context)
            .await
            .map(|v| v.len() as Count)
    }

    async fn find_by_id<T>(
        &self,
        context: CommandContext,
        id: <T as cqrs::record_trait::RecordTrait>::Id,
    ) -> Tri<T, WebError>
    where
        T: for<'a> Deserialize<'a> + RecordTrait,
    {
        let command = Query::by_id(id, context);
        if self.session.is_authenticated() {
            let server_result = self.server_find_by_id(&command).await;
            if let Tri::Err(e) = server_result {
                // Log the warning and fall through to querying the client backend
                warn!("{}", e);
            } else {
                return server_result;
            }
        }

        self.client_find_by_id(&command).await
    }

    async fn save<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::upsert(instance, context))
            .await
    }

    async fn add<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::add(instance, context)).await
    }

    async fn update<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::update(instance, context))
            .await
    }

    async fn delete<'a, T: Serialize + RecordTrait>(
        &self,
        context: CommandContext,
        instance: &'a T,
    ) -> Result<(), WebError>
    where
        &'a T: RecordTrait,
    {
        self.perform_command(Command::delete(instance, context))
            .await
    }
}

impl<CB: Debug, SB: Debug, TB: Debug> Debug for PersistenceManager<CB, SB, TB> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn try_lock_or_print_info<T: Debug>(lock: &Arc<Lock<T>>) -> String {
            lock.try_lock()
                .map_or("Can't acquire lock".to_owned(), |i| format!("{:?}", i))
        }
        write!(
            f,
            "PersistenceManager {{
session: {:?},
client_backend: {},
server_backend: {},
transient_backend: {}
}}",
            self.session,
            try_lock_or_print_info(&self.client_backend),
            try_lock_or_print_info(&self.server_backend),
            try_lock_or_print_info(&self.transient_backend),
        )
    }
}
