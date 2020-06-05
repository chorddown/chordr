use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum WebError {
    SortableError(String),
    JsError(String),
    SetlistDeserializeError(String),
    PersistenceError(String),
}

#[allow(unused)]
impl WebError {
    pub fn sortable_error<S: Into<String>>(s: S) -> Self {
        WebError::SortableError(s.into())
    }

    pub fn js_error<S: Into<String>>(s: S) -> Self {
        WebError::JsError(s.into())
    }

    pub fn persistence_error<S: Into<String>>(s: S) -> Self {
        WebError::PersistenceError(s.into())
    }

    pub fn setlist_deserialize_error<S: Into<String>>(s: S) -> Self {
        WebError::SetlistDeserializeError(s.into())
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                WebError::SortableError(s) => s,
                WebError::JsError(s) => s,
                WebError::SetlistDeserializeError(s) => s,
                WebError::PersistenceError(s) => s,
            }
        )
    }
}

impl Error for WebError {}

impl ::std::convert::From<wasm_bindgen::JsValue> for WebError {
    fn from(v: JsValue) -> Self {
        WebError::JsError(format!("{:?}", v))
    }
}

impl ::std::convert::From<serde_json::error::Error> for WebError {
    fn from(e: serde_json::error::Error) -> Self {
        WebError::JsError(format!("{:?}", e))
    }
}
