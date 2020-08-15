use crate::errors::WebError;
use crate::helpers::window;
use libchordr::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys;
use web_sys::{Request as WebRequest, RequestInit, RequestMode, Response as WebResponse};
use yew::Callback;

type FetchResult<OUT> = Result<OUT, WebError>;

/// Fetch a URI
///
/// # Example
/// ```rust,no_run
/// # use webchordr::fetch;
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
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    fetch_with_options(uri, &options).await
}

/// Fetch a URI and invoke the given callback when finished
///
/// # Example
/// ```rust,no_run
/// # use webchordr::fetch_with_callback;
/// # use yew::Callback;
///
/// # type TargetType = String;
///
/// # let uri = "some_uri";
///
/// # fn cb(_:Result<TargetType,webchordr::WebError>) {}
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
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    let result: Result<OUT, _> = fetch_with_options(uri, &options).await;
    callback.emit(result.clone());

    result
}

pub async fn fetch_with_options<OUT>(uri: &str, options: &RequestInit) -> FetchResult<OUT>
where
    OUT: for<'a> Deserialize<'a>,
{
    let request = WebRequest::new_with_str_and_init(uri, options).unwrap();
    let request_promise = window().fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: WebResponse = resp.dyn_into().expect("response not working...");
    let json = resp.json()?;
    let json = JsFuture::from(json).await?;

    Ok(json.into_serde::<OUT>()?)
}
