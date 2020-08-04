use super::WebRepositoryTrait;
use crate::errors::PersistenceError;
use crate::persistence::persistence_manager::PersistenceManagerTrait;
use crate::{fetch, WebError};
use async_trait::async_trait;
use js_sys::Date;
use libchordr::prelude::Catalog;
use wasm_bindgen::__rt::core::marker::PhantomData;

pub struct CatalogWebRepository<'a, P: PersistenceManagerTrait> {
    _phantom: ::std::marker::PhantomData<&'a P>,
}

impl<'a, P> CatalogWebRepository<'a, P>
where
    P: PersistenceManagerTrait,
{
    pub fn new(_persistence_manager: &'a mut P) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    async fn load_catalog(no_cache: bool) -> Result<Option<Catalog>, WebError> {
        let uri_base = "/catalog.json".to_owned();
        let uri = if no_cache {
            format!("{}?{}", uri_base, Date::now())
        } else {
            uri_base
        };

        let catalog = fetch::<Catalog>(&uri).await?;
        Ok(Some(catalog))
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
        match <CatalogWebRepository<'a, P>>::load_catalog(true).await {
            Ok(r) => Ok(r),
            Err(_) => <CatalogWebRepository<'a, P>>::load_catalog(false).await,
        }
    }
}
