use self::bs::BrowserStorageBackend;
use crate::browser_storage::BrowserStorage;
use crate::fetch_helper::fetch;
use crate::WebError;
use libchordr::prelude::Catalog;
use webchordr_common::tri::Tri;

pub struct CatalogWebRepository {
    backend: BrowserStorageBackend,
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
                let _ = self.backend.store(&c);

                return Tri::Some(c);
            }
            Tri::None => {}
            Tri::Err(e) => log::error!("{}", e),
        }

        self.backend.load()
    }
}

mod bs {
    use crate::browser_storage::*;
    use crate::errors::PersistenceError;
    use crate::errors::WebError;
    use crate::storage_key_utility::build_combined_key;
    use libchordr::prelude::Catalog;
    use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
    use webchordr_common::constants::{STORAGE_KEY_CATALOG, STORAGE_NAMESPACE};
    use webchordr_common::tri::Tri;

    /// A simplified version of the general BrowserStorage based backend
    pub struct BrowserStorageBackend {
        browser_storage: RwLock<BrowserStorage>,
    }

    impl BrowserStorageBackend {
        pub fn new(browser_storage: BrowserStorage) -> Self {
            Self {
                browser_storage: RwLock::new(browser_storage),
            }
        }

        /// Acquire a lock for reading
        fn lock_for_reading(&self) -> Result<RwLockReadGuard<BrowserStorage>, WebError> {
            match self.browser_storage.read() {
                Ok(l) => Ok(l),
                Err(_) => Err(PersistenceError::general_error(
                    "Could not acquire lock for reading",
                )
                .into()),
            }
        }

        /// Acquire a lock for reading
        fn lock_for_writing(&self) -> Result<RwLockWriteGuard<BrowserStorage>, WebError> {
            match self.browser_storage.write() {
                Ok(l) => Ok(l),
                Err(_) => Err(PersistenceError::general_error(
                    "Could not acquire lock for writing",
                )
                .into()),
            }
        }

        pub(super) fn store(&self, value: &Catalog) -> Result<(), WebError> {
            match serde_json::to_string(&value) {
                Ok(serialized) => self.lock_for_writing()?.set_item(
                    build_combined_key(&STORAGE_NAMESPACE, &STORAGE_KEY_CATALOG),
                    serialized,
                ),
                Err(e) => Err(PersistenceError::serialization_error(e.to_string()).into()),
            }
        }

        pub(super) fn load(&self) -> Tri<Catalog, WebError> {
            let lock_guard = match self.lock_for_reading() {
                Ok(l) => l,
                Err(e) => return Tri::Err(e),
            };

            match lock_guard.get_item(build_combined_key(&STORAGE_NAMESPACE, &STORAGE_KEY_CATALOG))
            {
                Some(v) => match serde_json::from_str(v.as_str()) {
                    Ok(serialized) => Tri::from_option(serialized),
                    Err(e) => Tri::Err(PersistenceError::deserialization_error(e, Some(v)).into()),
                },
                None => Tri::None,
            }
        }
    }
}
