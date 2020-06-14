use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum WebError {
    SortableError(String),
    JsError(String),
    SetlistDeserializeError(String),
    PersistenceError(PersistenceError),
}

#[allow(unused)]
impl WebError {
    pub fn sortable_error<S: Into<String>>(s: S) -> Self {
        WebError::SortableError(s.into())
    }

    pub fn js_error<S: Into<String>>(s: S) -> Self {
        WebError::JsError(s.into())
    }

    pub fn persistence_error(s: PersistenceError) -> Self {
        WebError::PersistenceError(s)
    }

    pub fn setlist_deserialize_error<S: Into<String>>(s: S) -> Self {
        WebError::SetlistDeserializeError(s.into())
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            WebError::SortableError(s) => f.write_str(s),
            WebError::JsError(s) => f.write_str(s),
            WebError::SetlistDeserializeError(s) => f.write_str(s),
            WebError::PersistenceError(s) => f.write_str(s.to_string().as_str()),
        }
    }
}

impl Error for WebError {}

impl From<wasm_bindgen::JsValue> for WebError {
    fn from(v: JsValue) -> Self {
        WebError::JsError(format!("{:?}", v))
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

#[derive(Debug, Clone)]
pub enum PersistenceError {
    SerializationError(String),
    DeserializationError(String, Option<String>),
    StorageUnavailable(String),
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
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PersistenceError::SerializationError(s) => f.write_str(s),
            PersistenceError::DeserializationError(s, _) => f.write_str(s),
            PersistenceError::GeneralError(s) => f.write_str(s),
            PersistenceError::StorageUnavailable(s) => f.write_str(s),
        }
    }
}

impl Error for PersistenceError {}
