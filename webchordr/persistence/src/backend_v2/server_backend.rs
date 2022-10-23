use crate::errors::WebError;
use crate::fetch_helper::{
    fetch_with_additional_headers, fetch_with_options_and_additional_headers,
};
use crate::persistence_manager::CommandContext;
use crate::shared::missing_record_id_error;
use async_trait::async_trait;
use cqrs::nonblocking::{CommandExecutor, QueryExecutor};
use cqrs::prelude::{Command, Query};
use libchordr::prelude::{Credentials, RecordTrait};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;
use web_sys::{RequestInit, RequestMode};
use webchordr_common::tri::Tri;

pub struct ServerBackend<R: RecordTrait + Serialize + DeserializeOwned> {
    host: String,
    credentials: Option<Credentials>,
    _data_type: PhantomData<R>,
}

impl<R: RecordTrait + Serialize + DeserializeOwned> ServerBackend<R> {
    pub fn new<S: Into<String>>(host: S, credentials: Option<Credentials>) -> Self {
        Self {
            host: host.into(),
            credentials,
            _data_type: PhantomData,
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

    fn build_request_uri_from_context(
        &self,
        context: &CommandContext,
        suffix: Option<&str>,
    ) -> String {
        self.build_request_uri(&context.namespace, &context.key, suffix)
    }

    fn build_base_request_uri<N: AsRef<str>, K: AsRef<str>>(
        &self,
        _namespace: &N,
        key: &K,
    ) -> String {
        if key.as_ref() == webchordr_common::constants::STORAGE_V2_KEY_SETLIST {
            return self.build_base_request_uri(_namespace, &"setlist");
        }

        match &self.credentials {
            None => format!("{}/{}", self.host, key.as_ref()),
            Some(c) => format!("{}/{}/{}", self.host, key.as_ref(), c.username()),
        }
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

    async fn send_post<T: Serialize + RecordTrait>(
        &self,
        context: &CommandContext,
        value: &T,
    ) -> Result<(), WebError> {
        let mut headers = self.build_request_headers();
        headers.insert("Content-Type", "application/json".to_string());

        // TODO: Append the Record ID to the URL?
        // let uri = self.build_request_uri(&namespace, &key, &RecordIdTrait::id(value).to_string());
        let uri = self.build_request_uri(&context.namespace, &context.key, None);

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
}

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> CommandExecutor for ServerBackend<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn upsert(
        &self,
        command: Command<Self::RecordType, Self::Context>,
    ) -> Result<(), WebError> {
        self.send_post(command.context(), command.record()).await
    }

    async fn add(&self, command: Command<Self::RecordType, Self::Context>) -> Result<(), WebError> {
        self.send_post(command.context(), command.record()).await
    }

    async fn update(
        &self,
        command: Command<Self::RecordType, Self::Context>,
    ) -> Result<(), WebError> {
        self.send_post(command.context(), command.record()).await
    }

    async fn delete(
        &self,
        _command: Command<Self::RecordType, Self::Context>,
    ) -> Result<(), WebError> {
        todo!("delete() is not implemented")
    }
}

#[async_trait(? Send)]
impl<R: RecordTrait + Serialize + DeserializeOwned> QueryExecutor for ServerBackend<R> {
    type RecordType = R;
    type Error = WebError;
    type Context = CommandContext;

    async fn find_all(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Result<Vec<Self::RecordType>, Self::Error> {
        let headers = self.build_request_headers();
        let uri = self.build_request_uri_from_context(query.context(), None);

        fetch_with_additional_headers::<Vec<Self::RecordType>, &str>(uri.as_str(), headers).await
    }

    async fn find_by_id(
        &self,
        query: Query<Self::RecordType, Self::Context>,
    ) -> Tri<Self::RecordType, Self::Error> {
        let headers = self.build_request_headers();
        let id = match query.id() {
            None => return Tri::Err(missing_record_id_error()),
            Some(id) => id,
        };
        let uri =
            self.build_request_uri_from_context(query.context(), Some(id.to_string().as_ref()));

        fetch_with_additional_headers::<Self::RecordType, &str>(uri.as_str(), headers)
            .await
            .into()
    }
}
