use crate::errors::WebError;
use libchordr::prelude::*;
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
/// use wasm_bindgen_futures::spawn_local;
/// spawn_local(async move {
///     let _ = fetch::fetch::<TargetType>(&uri, callback).await.unwrap();
/// });
/// ```
pub async fn fetch<OUT>(uri: &str, callback: Callback<FetchResult<OUT>>) -> FetchResult<OUT>
    where
        OUT: for<'a> serde::de::Deserialize<'a> + Clone,
{
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    fetch_with_options(uri, &options, callback).await
}

pub async fn fetch_with_options<OUT>(
    uri: &str,
    options: &RequestInit,
    callback: Callback<FetchResult<OUT>>,
) -> FetchResult<OUT>
    where
        OUT: for<'a> serde::de::Deserialize<'a> + Clone,
{
    let request = WebRequest::new_with_str_and_init(uri, options).unwrap();
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: WebResponse = resp.dyn_into().expect("response not working...");
    let json = resp.json()?;
    let json = JsFuture::from(json).await?;

    match json.into_serde::<OUT>() {
        Ok(rv) => {
            callback.emit(Ok(rv.clone()));

            Ok(rv)
        }
        Err(e) => Err(e.into()),
    }
}
