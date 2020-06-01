use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/src-typescript/SortableWrapper.ts")]
extern "C" {
    #[derive(Debug)]
    pub type SortableWrapper;

    #[wasm_bindgen(constructor)]
    pub fn new(s: &HtmlElement, handler: &Function, options: &JsValue) -> SortableWrapper;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &SortableWrapper) -> SortableWrapper;
}
