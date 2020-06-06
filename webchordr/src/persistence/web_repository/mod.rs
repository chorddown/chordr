mod catalog_web_repository;
mod setlist_web_repository;
mod settings_web_repository;
mod web_repository_trait;

pub use self::catalog_web_repository::CatalogWebRepository;
pub use self::setlist_web_repository::SetlistWebRepository;
pub use self::settings_web_repository::SettingsWebRepository;
pub use self::web_repository_trait::WebRepositoryTrait;
