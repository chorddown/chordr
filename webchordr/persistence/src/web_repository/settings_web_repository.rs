use super::WebRepositoryTrait;
use crate::persistence_manager::PersistenceManagerTrait;
use crate::WebError;
use async_trait::async_trait;
use libchordr::prelude::SongSettingsMap;
use webchordr_common::tri::Tri;

pub struct SettingsWebRepository<'a, P: PersistenceManagerTrait> {
    persistence_manager: &'a P,
}

impl<'a, P> SettingsWebRepository<'a, P>
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
impl<'a, P> WebRepositoryTrait for SettingsWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    type ManagedType = SongSettingsMap;

    fn namespace() -> &'static str {
        crate::constants::STORAGE_NAMESPACE
    }

    fn key() -> &'static str {
        crate::constants::STORAGE_KEY_SETTINGS
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
