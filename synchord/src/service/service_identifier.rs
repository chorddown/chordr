use crate::error::Error;
use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ServiceIdentifier {
    WebDAV,
    Dropbox,
}

impl TryFrom<&str> for ServiceIdentifier {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dropbox" => Ok(ServiceIdentifier::Dropbox),
            "webdav" => Ok(ServiceIdentifier::WebDAV),
            _ => Err(Error::unknown_service_error(format!(
                "Service {} is not implemented",
                value
            ))),
        }
    }
}

impl Display for ServiceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                ServiceIdentifier::WebDAV => "WebDAV",
                ServiceIdentifier::Dropbox => "Dropbox",
            }
        )
    }
}
