use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use libchordr::data_exchange::{DeserializerError, SerializerError};
use wasm_bindgen::JsValue;
use web_sys::Response;

#[derive(Debug, Clone, PartialEq)]
pub enum WebError {
    SortableError(String),
    JsError(String),
    CustomError(String),
    SharingError(SharingError),
    PersistenceError(PersistenceError),
    ResponseError(String, Response),
    CredentialsError(String),
}

#[allow(unused)]
impl WebError {
    pub fn sortable_error<S: Into<String>>(s: S) -> Self {
        WebError::SortableError(s.into())
    }

    pub fn js_error<S: Into<String>>(s: S) -> Self {
        WebError::JsError(s.into())
    }

    pub fn custom_error<S: Into<String>>(s: S) -> Self {
        WebError::CustomError(s.into())
    }

    pub fn persistence_error(s: PersistenceError) -> Self {
        WebError::PersistenceError(s)
    }

    pub fn sharing_error(e: SharingError) -> Self {
        WebError::SharingError(e)
    }

    pub fn response_error<S: Into<String>>(url: S, response: Response) -> Self {
        WebError::ResponseError(url.into(), response)
    }

    pub fn credentials_error<S: Display>(s: S) -> Self {
        WebError::CredentialsError(s.to_string())
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            WebError::SortableError(s) => f.write_str(s),
            WebError::JsError(s) => f.write_str(s),
            WebError::CustomError(s) => f.write_str(s),
            WebError::SharingError(s) => f.write_str(s.to_string().as_str()),
            WebError::PersistenceError(s) => f.write_str(s.to_string().as_str()),
            WebError::ResponseError(u, r) => write!(
                f,
                "Error fetching URL {}: Status {} {}",
                u,
                r.status(),
                r.status_text()
            ),
            WebError::CredentialsError(s) => f.write_str(s.to_string().as_str()),
        }
    }
}

impl Error for WebError {}

impl From<wasm_bindgen::JsValue> for WebError {
    fn from(v: JsValue) -> Self {
        WebError::JsError(format!("{:?}", v))
    }
}

impl From<serde_wasm_bindgen::Error> for WebError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        WebError::JsError(format!("{:?}", e))
    }
}

impl From<serde_json::error::Error> for WebError {
    fn from(e: serde_json::error::Error) -> Self {
        WebError::JsError(format!("{:?}", e))
    }
}

impl From<PersistenceError> for WebError {
    fn from(e: PersistenceError) -> Self {
        WebError::PersistenceError(e)
    }
}

impl From<SharingError> for WebError {
    fn from(e: SharingError) -> Self {
        WebError::SharingError(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceError {
    SerializationError(String),
    DeserializationError(String, Option<String>),
    StorageUnavailable(String),
    RecordNotFoundError(String),
    RecordExistsError(String),
    MissingRecordIdError(String),
    BackendError(String, Vec<WebError>),
    GeneralError(String),
}

impl PersistenceError {
    pub fn serialization_error<S: Display>(s: S) -> Self {
        Self::SerializationError(s.to_string())
    }

    pub fn deserialization_error<S: Display>(s: S, content: Option<String>) -> Self {
        Self::DeserializationError(s.to_string(), content)
    }

    pub fn storage_unavailable<S: Display>(s: S) -> Self {
        Self::StorageUnavailable(s.to_string())
    }

    pub fn general_error<S: Display>(s: S) -> Self {
        Self::GeneralError(s.to_string())
    }

    pub fn record_not_found_error<S: Display>(s: S) -> Self {
        Self::RecordNotFoundError(s.to_string())
    }

    pub fn record_exists_error<S: Display>(s: S) -> Self {
        Self::RecordExistsError(s.to_string())
    }

    pub fn missing_record_id_error<S: Display>(s: S) -> Self {
        Self::MissingRecordIdError(s.to_string())
    }

    pub fn backend_error<S: Display>(s: S, errors: Vec<WebError>) -> Self {
        Self::BackendError(s.to_string(), errors)
    }
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PersistenceError::SerializationError(s) => f.write_str(s),
            PersistenceError::DeserializationError(s, _) => f.write_str(s),
            PersistenceError::StorageUnavailable(s) => f.write_str(s),
            PersistenceError::RecordNotFoundError(s) => f.write_str(s),
            PersistenceError::RecordExistsError(s) => f.write_str(s),
            PersistenceError::MissingRecordIdError(s) => f.write_str(s),
            PersistenceError::BackendError(s, _) => f.write_str(s),
            PersistenceError::GeneralError(s) => f.write_str(s),
        }
    }
}

impl Error for PersistenceError {}

#[derive(Debug, Clone, PartialEq)]
pub enum SharingError {
    Serialization(String),
    SongNotFound(String),
    Deserialization(String),
}

impl SharingError {}

impl From<DeserializerError> for SharingError {
    fn from(e: DeserializerError) -> Self {
        SharingError::Deserialization(e.to_string())
    }
}
impl From<SerializerError> for SharingError {
    fn from(e: SerializerError) -> Self {
        SharingError::Serialization(e.to_string())
    }
}

impl Display for SharingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SharingError::Serialization(e) => write!(f, "Serialization error: {}", e),
            SharingError::Deserialization(e) => write!(f, "Deserialization error: {}", e),
            SharingError::SongNotFound(e) => write!(f, "Song Not Found: {}", e),
        }
    }
}

impl Error for SharingError {}
