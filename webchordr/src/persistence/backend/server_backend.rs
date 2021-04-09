use crate::errors::WebError;
use crate::persistence::backend::BackendTrait;
use crate::{fetch_with_additional_headers, fetch_with_options_and_additional_headers};
use async_trait::async_trait;
use libchordr::prelude::{Credentials, RecordTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::{RequestInit, RequestMode};

pub struct ServerBackend {
    host: String,
    credentials: Option<Credentials>,
}

impl ServerBackend {
    pub fn new<S: Into<String>>(host: S, credentials: Option<Credentials>) -> Self {
        Self {
            host: host.into(),
            credentials,
        }
    }

    fn build_request_uri<N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: &N,
        key: &K,
        suffix: Option<&str>,
    ) -> String {
        match suffix {
            Some(s) => format!("{}/{}", self.build_base_request_uri(namespace, key), s),
            None => self.build_base_request_uri(namespace, key),
        }
    }

    fn build_base_request_uri<N: AsRef<str>, K: AsRef<str>>(
        &self,
        _namespace: &N,
        key: &K,
    ) -> String {
        match &self.credentials {
            None => format!("{}/{}", self.host, key.as_ref()),
            Some(c) => format!("{}/{}/{}", self.host, key.as_ref(), c.username()),
        }

        // if namespace.as_ref().is_empty() {
        //     key.as_ref().to_string()
        // } else {
        //     format!("{}/{}", self.host, key.as_ref())
        //     // format!("{}/{}/{}", self.host, namespace.as_ref(), key.as_ref())
        // }
    }

    fn build_request_headers(&self) -> HashMap<&str, String> {
        let mut headers = HashMap::new();
        if let Some(credentials) = &self.credentials {
            let hash = base64::encode(format!(
                "{}:{}",
                credentials.username(),
                credentials.password()
            ));
            headers.insert("Authorization", format!("Basic {}", hash));
        }

        headers
    }
}

#[async_trait(? Send)]
impl BackendTrait for ServerBackend {
    async fn store<T: Serialize + RecordTrait, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
        value: &T,
    ) -> Result<(), WebError> {
        let mut headers = self.build_request_headers();
        headers.insert("Content-Type", "application/json".to_string());

        // TODO: Append the Record ID to the URL?
        // let uri = self.build_request_uri(&namespace, &key, &RecordIdTrait::id(value).to_string());
        let uri = self.build_request_uri(&namespace, &key, None);

        let serialized_json_string = serde_json::to_string(value)?;
        let js_value = JsValue::from_str(&serialized_json_string);

        let mut options = RequestInit::new();
        options.method("POST");
        options.mode(RequestMode::Cors);
        options.body(Some(&js_value));

        let result = fetch_with_options_and_additional_headers::<
            HashMap<String, serde_json::Value>,
            &str,
        >(&uri, &options, Some(headers))
        .await;
        result.map(|_| ())
    }

    async fn load<T, N: AsRef<str>, K: AsRef<str>>(
        &self,
        namespace: N,
        key: K,
    ) -> Result<Option<T>, WebError>
    where
        T: for<'a> Deserialize<'a>,
    {
        let headers = self.build_request_headers();
        let uri = self.build_request_uri(&namespace, &key, Some("latest"));

        fetch_with_additional_headers(&uri, headers).await
    }
}
