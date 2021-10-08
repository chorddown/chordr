use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[cfg(not(feature = "trunk_build"))]
#[wasm_bindgen(module = "/dist/sortable.js")]
extern "C" {
    #[derive(Debug)]
    pub type SortableWrapper;

    #[wasm_bindgen(constructor)]
    pub fn new(s: &HtmlElement, handler: &Function, options: &JsValue) -> SortableWrapper;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &SortableWrapper) -> SortableWrapper;
}

#[cfg(feature = "trunk_build")]
#[wasm_bindgen(module = "/src-typescript/SortableWrapper-trunk.js")]
extern "C" {
    #[derive(Debug)]
    pub type SortableWrapper;

    #[wasm_bindgen(constructor)]
    pub fn new(s: &HtmlElement, handler: &Function, options: &JsValue) -> SortableWrapper;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &SortableWrapper) -> SortableWrapper;
}
