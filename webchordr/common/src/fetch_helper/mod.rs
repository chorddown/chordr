use crate::errors::WebError;
use crate::helpers::window;
use libchordr::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys;
use web_sys::{Request as WebRequest, RequestInit, RequestMode, Response as WebResponse};
use yew::Callback;

pub type FetchResult<OUT> = Result<OUT, WebError>;

/// Fetch a URI
///
/// # Example
/// ```rust,no_run
/// # use webchordr_common::fetch_helper::fetch;
/// # type TargetType = String;
/// # let uri = "some_uri";
///
/// use wasm_bindgen_futures::spawn_local;
/// spawn_local(async move {
///     let result = fetch::<TargetType>(&uri).await;
/// });
/// ```
pub async fn fetch<OUT>(uri: &str) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a>,
{
    fetch_with_options(uri, &get_default_options()).await
}

/// Fetch a URI and invoke the given callback when finished
///
/// # Example
/// ```rust,no_run
/// # use webchordr_common::fetch_helper::fetch_with_callback;
/// # use yew::Callback;
///
/// # type TargetType = String;
///
/// # let uri = "some_uri";
///
/// # fn cb(_:Result<TargetType, webchordr_common::errors::WebError>) {}
/// # let callback = Callback::from(cb);
/// use wasm_bindgen_futures::spawn_local;
/// spawn_local(async move {
///     let _ = fetch_with_callback::<TargetType>(&uri, callback).await.unwrap();
/// });
/// ```
pub async fn fetch_with_callback<OUT>(
    uri: &str,
    callback: Callback<FetchResult<OUT>>,
) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a> + Clone,
{
    let result: Result<OUT, _> = fetch_with_options(uri, &get_default_options()).await;
    callback.emit(result.clone());

    result
}

pub async fn fetch_with_options<OUT>(uri: &str, options: &RequestInit) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a>,
{
    fetch_with_options_and_additional_headers::<OUT, String>(uri, options, None).await
}

pub async fn fetch_with_additional_headers<OUT, AHKEY>(
    uri: &str,
    additional_headers: HashMap<AHKEY, String>,
) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a>,
    AHKEY: AsRef<str>,
{
    fetch_with_options_and_additional_headers(uri, &get_default_options(), Some(additional_headers))
        .await
}

/// Fetch a URI with additional settings
///
/// # Example: POST request
/// ```rust,no_run
/// # use webchordr_common::fetch_helper::fetch_with_options_and_additional_headers;
/// # let uri = "some_uri";
///
/// use wasm_bindgen_futures::spawn_local;
/// use web_sys::{RequestInit, RequestMode};
/// use std::collections::HashMap;
/// use wasm_bindgen::JsValue;
/// type TargetType = HashMap<String, serde_json::Value>;
/// spawn_local(async move {
///     let mut options = RequestInit::new();
///     options.method("POST");
///     options.mode(RequestMode::Cors);
///
///     // Configure the request body/post data
///     let serialized_json_string = serde_json::to_string("some value").expect("Could not serialize value");
///     let js_value = JsValue::from_str(&serialized_json_string);
///     options.body(Some(&js_value));
///
///     let result = fetch_with_options_and_additional_headers::<TargetType, &str>(&uri, &options, None).await;
/// });
/// ```
pub async fn fetch_with_options_and_additional_headers<OUT, AHKEY>(
    uri: &str,
    options: &RequestInit,
    additional_headers: Option<HashMap<AHKEY, String>>,
) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a>,
    AHKEY: AsRef<str>,
{
    let request = WebRequest::new_with_str_and_init(uri, options).unwrap();
    if let Some(headers) = additional_headers {
        for (key, value) in headers {
            request.headers().set(key.as_ref(), &value)?;
        }
    }
    let request_promise = window().fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: WebResponse = resp.dyn_into().expect("response not working...");
    if resp.ok() {
        let json = resp.json()?;
        let json = JsFuture::from(json).await?;

        Ok(serde_wasm_bindgen::from_value::<OUT>(json)?)
    } else {
        // TODO: If `resp.headers().get("Content-Type")` contains JSON parse the error message
        Err(WebError::response_error(uri, resp))
    }
}

fn get_default_options() -> RequestInit {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);
    options
}
