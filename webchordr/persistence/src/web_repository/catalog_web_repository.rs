use crate::backend::{BackendTrait, BrowserStorageBackend};
use crate::browser_storage::BrowserStorage;
use crate::constants::{STORAGE_KEY_CATALOG, STORAGE_NAMESPACE};
use crate::fetch_helper::fetch;
use crate::WebError;
use libchordr::prelude::Catalog;
use webchordr_common::tri::Tri;

pub struct CatalogWebRepository {
    backend: BrowserStorageBackend<BrowserStorage>,
}

impl CatalogWebRepository {
    pub fn new(browser_storage: BrowserStorage) -> Self {
        Self {
            backend: BrowserStorageBackend::new(browser_storage),
        }
    }

    async fn fetch_catalog(&self, append_timestamp: bool) -> Tri<Catalog, WebError> {
        let base_uri = "/catalog.json";
        let uri = if append_timestamp {
            format!("{}?{}", base_uri, chrono::Local::now().timestamp())
        } else {
            base_uri.to_string()
        };

        match fetch::<Catalog>(&uri).await {
            Ok(catalog) => Tri::Some(catalog),
            Err(error) => Tri::Err(error),
        }
    }

    pub async fn load(&mut self) -> Tri<Catalog, WebError> {
        match self.fetch_catalog(true).await {
            Tri::Some(c) => {
                // Store/cache the loaded Catalog
                let _ = self
                    .backend
                    .store(STORAGE_NAMESPACE, STORAGE_KEY_CATALOG, &c)
                    .await;

                return Tri::Some(c);
            }
            Tri::None => {}
            Tri::Err(e) => log::error!("{}", e),
        }

        self.backend
            .load(STORAGE_NAMESPACE, STORAGE_KEY_CATALOG)
            .await
    }
}
