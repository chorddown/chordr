use crate::config::Config;
use crate::fetch_helper::*;
use crate::persistence::browser_storage::BrowserStorageTrait;
use crate::persistence::prelude::BrowserStorage;
use crate::session::session_main_data::SessionMainData;
use crate::session::Session;
use crate::WebError;
use libchordr::models::user::MainData;
use libchordr::prelude::{Credentials, Password, User, Username};
use std::collections::HashMap;

const STORAGE_KEY_USERNAME: &str = "username";
const STORAGE_KEY_PASSWORD: &str = "password";

pub struct SessionService {
    config: Config,
    session_storage: BrowserStorage,
}

impl SessionService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            session_storage: BrowserStorage::session_storage().expect("Session storage"),
        }
    }

    pub async fn try_login(&self, credentials: &Credentials) -> Result<Session, WebError> {
        self.perform_login(credentials).await
    }

    pub async fn try_from_browser_storage(&self) -> Result<Session, WebError> {
        let credentials = self.get_credentials_from_session_storage()?;

        self.perform_login(&credentials).await
    }

    #[allow(unused)]
    pub fn has_credentials_in_session_storage(&self) -> bool {
        self.get_credentials_from_session_storage().is_ok()
    }

    pub fn get_credentials_from_session_storage(&self) -> Result<Credentials, WebError> {
        let username = self.get_username_from_session_storage()?;
        let password = self.get_password_from_session_storage()?;

        Ok(Credentials::new(username, password))
    }

    pub fn set_credentials_in_session_storage(
        &mut self,
        credentials: &Credentials,
    ) -> Result<(), WebError> {
        self.session_storage
            .set_item(STORAGE_KEY_PASSWORD, credentials.password().to_string())?;
        self.session_storage
            .set_item(STORAGE_KEY_USERNAME, credentials.username().to_string())?;

        Ok(())
    }

    pub async fn get_main_data(
        &self,
        credentials: &Credentials,
    ) -> Result<SessionMainData, WebError> {
        let headers = self.build_basic_auth_headers(credentials);
        let uri = format!("{}/user/", self.config.api_url());

        let main_data: MainData = fetch_with_additional_headers(&uri, headers).await?;
        let password = credentials.password().clone();
        let user = main_data.user.clone().with_password(password);
        log::info!("{:?}", user);
        Ok(SessionMainData {
            session: Session::with_user(user.clone()),
            main_data: main_data.with_user(user),
        })
    }

    async fn perform_login(&self, credentials: &Credentials) -> Result<Session, WebError> {
        let headers = self.build_basic_auth_headers(credentials);
        let uri = format!("{}/user/", self.config.api_url());

        let user: User = fetch_with_additional_headers(&uri, headers).await?;

        let user = user.with_password(credentials.password().clone());
        log::info!("{:?}", user);
        Ok(Session::with_user(user))
    }

    fn get_password_from_session_storage(&self) -> Result<Password, WebError> {
        match self.session_storage.get_item(STORAGE_KEY_PASSWORD) {
            None => Err(WebError::credentials_error("No password set")),
            Some(password) => Password::new(password).map_err(WebError::credentials_error),
        }
    }

    fn get_username_from_session_storage(&self) -> Result<Username, WebError> {
        match self.session_storage.get_item(STORAGE_KEY_USERNAME) {
            None => Err(WebError::credentials_error("No username set")),
            Some(username) => Username::new(username).map_err(WebError::credentials_error),
        }
    }

    fn build_basic_auth_headers(&self, credentials: &Credentials) -> HashMap<&str, String> {
        let mut headers = HashMap::new();
        let hash = base64::encode(format!(
            "{}:{}",
            credentials.username(),
            credentials.password()
        ));
        headers.insert("Authorization", format!("Basic {}", hash));

        headers
    }
}
