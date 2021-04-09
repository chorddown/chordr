use super::WebRepositoryTrait;
use crate::errors::PersistenceError;
use crate::persistence::persistence_manager::PersistenceManagerTrait;
use crate::{fetch, WebError};
use async_trait::async_trait;
use libchordr::prelude::Catalog;
use wasm_bindgen::__rt::core::marker::PhantomData;

pub struct CatalogWebRepository<'a, P: PersistenceManagerTrait> {
    _phantom: ::std::marker::PhantomData<&'a P>,
}

impl<'a, P> CatalogWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    pub fn new(_persistence_manager: &'a P) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

#[async_trait(? Send)]
impl<'a, P> WebRepositoryTrait for CatalogWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    type ManagedType = Catalog;

    fn namespace() -> &'static str {
        ""
    }

    fn key() -> &'static str {
        ""
    }

    async fn store(&mut self, _value: &Self::ManagedType) -> Result<(), WebError> {
        Err(PersistenceError::general_error("Changing the Catalog is not implement").into())
    }

    async fn load(&mut self) -> Result<Option<Self::ManagedType>, WebError> {
        let catalog = fetch::<Catalog>("/catalog.json").await?;
        Ok(Some(catalog))
    }
}
