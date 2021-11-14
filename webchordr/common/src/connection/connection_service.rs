use super::ConnectionStatus;
use crate::config::Config;
use crate::fetch_helper::{fetch, FetchResult};
use crate::helpers::window;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct ConnectionService {
    config: Config,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConnectionInfo {
    version: String,
    running: bool,
}

impl ConnectionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn get_connection_status(&self) -> ConnectionStatus {
        let navigator = window().navigator();
        if !navigator.on_line() {
            return ConnectionStatus::Offline;
        }

        let uri = format!("{}/status/", self.config.api_url());
        let connection_info: FetchResult<ConnectionInfo> = fetch(&uri).await;

        match connection_info {
            Err(_) => {
                if navigator.on_line() {
                    ConnectionStatus::ServerNotReachable
                } else {
                    ConnectionStatus::Offline
                }
            }
            Ok(_) => ConnectionStatus::OnLine,
        }
    }
}
