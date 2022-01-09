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
impl<'a, 'b, P> cqrs::prelude::AsyncRepositoryTrait for &'b SetlistWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    type ManagedType = Setlist;
    type Error = WebError;
    type Context = CommandContext;

    async fn find_all(
        &self,
        context: &Self::Context,
    ) -> Result<Vec<Self::ManagedType>, Self::Error> {
        self.persistence_manager
            .find_all::<Self::ManagedType>(context.clone())
            .await
    }

    async fn count_all(&self, context: &Self::Context) -> Result<Count, Self::Error> {
        self.persistence_manager.count_all(context.clone()).await
    }

    async fn find_by_id(
        &self,
        context: &Self::Context,
        id: <Self::ManagedType as RecordTrait>::Id,
    ) -> Tri<Self::ManagedType, Self::Error> {
        self.persistence_manager
            .find_by_id::<Self::ManagedType>(context.clone(), id)
            .await
    }

    async fn add(
        &self,
        context: &Self::Context,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.persistence_manager
            .add::<Self::ManagedType>(context.clone(), &instance)
            .await
    }

    async fn update(
        &self,
        context: &Self::Context,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.persistence_manager
            .update::<Self::ManagedType>(context.clone(), &instance)
            .await
    }

    async fn delete(
        &self,
        context: &Self::Context,
        instance: Self::ManagedType,
    ) -> Result<(), Self::Error> {
        self.persistence_manager
            .delete::<Self::ManagedType>(context.clone(), &instance)
            .await
    }
}
