use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/src/dropzone.js")]
extern "C" {
    #[derive(Debug)]
    pub type DropzoneWrapper;

    #[wasm_bindgen(constructor)]
    pub fn new(
        container: &HtmlElement,
        item_selectors: String,
        on_drop: &Function,
    ) -> DropzoneWrapper;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &DropzoneWrapper) -> DropzoneWrapper;
}
