use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Promise;
use log::error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::errors::WebError;
use crate::helpers::window;

pub struct ClosureHandle(Rc<RefCell<Option<Closure<dyn Fn()>>>>);

pub async fn request_tick_after_timeout(
    milliseconds: i32,
) -> (ClosureHandle, Result<JsValue, JsValue>) {
    let closure_handle = Rc::new(RefCell::new(None));
    let promise = Promise::new(&mut |resolve, _reject| {
        let closure = Closure::wrap(Box::new(move || {
            // if let Some(debug_message) = debug_message {
            //     web_sys::console::log_1(&debug_message.into());
            // }
            match resolve.call0(&JsValue::NULL) {
                Ok(_) => {}
                Err(e) => error!("{}", WebError::from(e)),
            }
        }) as Box<dyn Fn()>);

        *closure_handle.borrow_mut() = Some(closure);

        match set_timeout(
            &closure_handle.clone().borrow().as_ref().unwrap(),
            milliseconds,
        ) {
            Ok(_) => {}
            Err(e) => error!("{}", WebError::from(e)),
        }
    });

    let js_future = JsFuture::from(promise);

    (ClosureHandle(closure_handle), js_future.await)
}

fn set_timeout(callback: &Closure<dyn Fn()>, milliseconds: i32) -> Result<(), WebError> {
    match window().set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        milliseconds,
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod test {
    use js_sys::Date;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn request_tick_after_timeout_run_in_order() {
        let wait_time = 1000i32;
        let before = Date::new_0().get_time();
        let (_callback_handle, result) = request_tick_after_timeout(wait_time).await;
        assert!(result.is_ok());
        let after = Date::new_0().get_time();

        // Assert that at least `wait_time` did elapse
        assert!(after - before > wait_time as f64)
    }
}
