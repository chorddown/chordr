use async_trait::async_trait;

use cqrs::prelude::Count;
use libchordr::prelude::{RecordTrait, Setlist};
use webchordr_common::tri::Tri;

use crate::backend::BackendTrait;
use crate::persistence_manager::{CommandContext, PersistenceManagerTrait};
use crate::WebError;

use super::WebRepositoryTrait;

pub struct SetlistWebRepository<'a, P: PersistenceManagerTrait> {
    persistence_manager: &'a P,
}

impl<'a, P> SetlistWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    pub fn new(persistence_manager: &'a P) -> Self {
        Self {
            persistence_manager,
        }
    }
}

#[async_trait(? Send)]
impl<'a, P> WebRepositoryTrait for SetlistWebRepository<'a, P>
where
    P: PersistenceManagerTrait + BackendTrait,
{
    type ManagedType = Setlist;

    fn namespace() -> &'static str {
        crate::constants::STORAGE_NAMESPACE
    }

    fn key() -> &'static str {
        crate::constants::STORAGE_KEY_SETLIST
    }

    fn build_context() -> CommandContext {
        CommandContext::new(
            crate::constants::STORAGE_NAMESPACE,
            crate::constants::STORAGE_V2_KEY_SETLIST,
        )
    }

    async fn store(&mut self, value: &Self::ManagedType) -> Result<(), WebError> {
        self.persistence_manager
            .store(Self::namespace(), Self::key(), value)
            .await
    }

    async fn load(&mut self) -> Tri<Self::ManagedType, WebError> {
        self.persistence_manager
            .load(Self::namespace(), Self::key())
            .await
    }
}

#[async_trait(? Send)]
impl<'a, P> cqrs::prelude::AsyncRepositoryTrait for SetlistWebRepository<'a, P>
where
    P: PersistenceManagerTrait + BackendTrait,
{
    type ManagedType = Setlist;
    type Error = WebError;

    async fn find_all(&self) -> Result<Vec<Self::ManagedType>, Self::Error> {
        self.persistence_manager
            .find_all::<Self::ManagedType>(Self::build_context())
            .await
    }

    async fn count_all(&self) -> Result<Count, Self::Error> {
        self.persistence_manager
            .count_all(Self::build_context())
            .await
    }

    async fn find_by_id(
        &self,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error> {
        self.persistence_manager
            .find_by_id::<Self::ManagedType>(Self::build_context(), id)
            .await
    }

    async fn save(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .save::<Self::ManagedType>(Self::build_context(), &instance)
            .await
    }

    async fn add(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .add::<Self::ManagedType>(Self::build_context(), &instance)
            .await
    }

    async fn update(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .update::<Self::ManagedType>(Self::build_context(), &instance)
            .await
    }

    async fn delete(&self, instance: Self::ManagedType) -> Result<(), Self::Error> {
        self.persistence_manager
            .delete::<Self::ManagedType>(Self::build_context(), &instance)
            .await
    }
}
