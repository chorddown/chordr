use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[derive(Debug, Clone)]
pub struct SortableWrapper {}

impl SortableWrapper {
    pub fn new(_s: &HtmlElement, _handler: &Function, _options: &JsValue) -> SortableWrapper {
        SortableWrapper {}
    }

    pub fn destroy(&self) -> SortableWrapper {
        self.clone()
    }
}
