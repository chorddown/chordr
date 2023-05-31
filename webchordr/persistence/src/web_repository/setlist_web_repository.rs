use async_trait::async_trait;
use cqrs::nonblocking::{CommandExecutor, QueryExecutor, RepositoryTrait};

use cqrs::prelude::{Command, Count, Query};
use libchordr::prelude::{RecordTrait, Setlist};
use webchordr_common::errors::PersistenceError;
use webchordr_common::tri::Tri;

use crate::backend_v2::context_provider::ContextProvider;
use crate::backend_v2::persistence_manager::PersistenceManagerV2;
use crate::WebError;

pub struct SetlistWebRepository {
    persistence_manager: PersistenceManagerV2<Setlist>,
}

impl SetlistWebRepository {
    pub fn new(persistence_manager: PersistenceManagerV2<Setlist>) -> Self {
        Self {
            persistence_manager,
        }
    }
}

#[async_trait(? Send)]
impl ContextProvider for SetlistWebRepository {
    fn namespace() -> &'static str {
        crate::constants::STORAGE_NAMESPACE
    }

    fn key() -> &'static str {
        crate::constants::STORAGE_V2_KEY_SETLIST
    }
}

#[async_trait(? Send)]
impl RepositoryTrait for SetlistWebRepository {
    type ManagedType = Setlist;
    type Error = WebError;

    async fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error> {
        self.persistence_manager
            .find_all(&Query::all(Self::build_context()))
            .await
    }

    async fn count_all(&self) -> Result<Count, Self::Error> {
        let result = self
            .persistence_manager
            .find_all(&Query::all(Self::build_context()))
            .await?;

        let count = Count::try_from(result.len());

        count.map_err(|_| PersistenceError::general_error("Count is out of bounds").into())
    }

    async fn find_by_id(
        &self,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error> {
        self.persistence_manager
            .find_by_id(&Query::by_id(id, Self::build_context()))
            .await
    }

    async fn save(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .upsert(&Command::upsert(instance, Self::build_context()))
            .await
    }

    async fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .add(&Command::add(instance, Self::build_context()))
            .await
    }

    async fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .update(&Command::update(instance, Self::build_context()))
            .await
    }

    async fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .delete(&Command::delete(instance, Self::build_context()))
            .await
    }
}
