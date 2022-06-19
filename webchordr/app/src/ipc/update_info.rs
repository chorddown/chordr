use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateInfo {
    pub r#type: String,
    pub version: String,
}
