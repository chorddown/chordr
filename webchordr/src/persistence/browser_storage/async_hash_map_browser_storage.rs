use super::BrowserStorageTrait;
use crate::helpers::window;
use crate::persistence::browser_storage::HashMapBrowserStorage;
use crate::WebError;
use js_sys::Function;
use js_sys::Promise;
use std::{thread, time};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;

pub struct AsyncHashMapBrowserStorage {
    hash_map_browser_storage: HashMapBrowserStorage,
}

impl AsyncHashMapBrowserStorage {
    pub fn new() -> Self {
        Self {
            hash_map_browser_storage: HashMapBrowserStorage::new(),
        }
    }
    pub async fn get_item<S: AsRef<str>>(&self, key_name: S) -> Option<String> {
        self.hash_map_browser_storage.get_item(key_name)

        // let promise = js_sys::Promise::resolve(&JsValue::from(self.hash_map_browser_storage.get_item(key_name)));
        // // console_log!("XX");
        // // Convert that promise into a future and make the test wait on it.
        //  JsFuture::from(promise).await.unwrap().into()

        // // Promise::new(cb: &mut dyn FnMut(Function, Function))
        // Promise::new(|resolve: Function, reject| {
        //     resolve.call0(Some("resolved".to_owned()))
        //     // let cb = Closure::wrap(Box::new(|| {
        //     //     web_sys::console::log_1(&"inverval elapsed!".into());
        //     //
        //     //     Some("resolved".to_owned())
        //     // }) as Box<dyn FnMut() -> Option<String>>);
        //     //
        //     // let window = window();
        //     // let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
        //     //     // Note this method call, which uses `as_ref()` to get a `JsValue`
        //     //     // from our `Closure` which is then converted to a `&Function`
        //     //     // using the `JsCast::unchecked_ref` function.
        //     //     cb.as_ref().unchecked_ref(),
        //     //     1_000,
        //     // ).unwrap();
        // })

        // JsFuture::from(promise)
        // window().set_timeout_with_callback_and_timeout_and_arguments_0(
        //     closure.as_ref().unchecked_ref(),
        //     3000,
        // ).unwrap();
        // closure.forget();

        // window()
        //     .set_timeout_with_callback_and_timeout_and_arguments_0();
        // let ten_millis = time::Duration::from_millis(10);
        // let now = time::Instant::now();

        // thread::sleep(ten_millis);
        // let thread_one = thread::spawn(|| download("https://www.foo.com"));

        // unimplemented!()
    }

    pub async fn set_item<S: Into<String>, V: Into<String>>(
        &mut self,
        key_name: S,
        key_value: V,
    ) -> Result<(), WebError> {
        self.hash_map_browser_storage.set_item(key_name, key_value)
    }

    pub async fn remove_item<S: AsRef<str>>(&mut self, key_name: S) -> Result<(), WebError> {
        self.hash_map_browser_storage.remove_item(key_name)
    }

    pub async fn clear(&mut self) -> Result<(), WebError> {
        unimplemented!()
    }

    pub async fn len(&self) -> usize {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::browser_storage::HashMapBrowserStorage;
    use crate::test_helpers::entry;
    use log::info;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::*;
    use web_sys::console::info;

    // use wasm_bindgen_test::{wasm_bindgen_test as test};
    use crate::persistence::PersistenceManager;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn set_and_get_test() {
        let mut storage = AsyncHashMapBrowserStorage::new();
        assert!(storage.set_item("abc", "a string content").await.is_ok());
        assert!(storage.get_item("abc").await.is_some());
        assert_eq!("a string content", storage.get_item("abc").await.unwrap());
    }

    #[wasm_bindgen_test]
    async fn get_not_existing_test() {
        let storage = AsyncHashMapBrowserStorage::new();
        assert!(storage.get_item("not existing").await.is_none());
    }
}
