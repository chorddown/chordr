use super::WebRepositoryTrait;
use crate::persistence_manager::PersistenceManagerTrait;
use crate::WebError;
use async_trait::async_trait;
use libchordr::prelude::Setlist;

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
    P: PersistenceManagerTrait,
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

    async fn load(&mut self) -> Result<Option<Self::ManagedType>, WebError> {
        self.persistence_manager
            .load(Self::namespace(), Self::key())
            .await
    }
}
