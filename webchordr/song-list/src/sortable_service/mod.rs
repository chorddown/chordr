#[cfg(not(test))]
mod sortable_wasm_binding;
#[cfg(test)]
mod sortable_wasm_fixture;

#[cfg(not(test))]
use self::sortable_wasm_binding::SortableWrapper;
#[cfg(test)]
use self::sortable_wasm_fixture::SortableWrapper;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use webchordr_events::sorting_change::Sorting;
use webchordr_events::SortingChange;
use yew::Callback;

type SortingChangeFn = dyn Fn(i32, i32);

/// Service to make a HtmlElement sortable using [Shopify/draggable](https://github.com/Shopify/draggable)
pub struct SortableService {}

#[must_use]
pub struct SortableHandle {
    sortable: SortableWrapper,
    _closure: Closure<SortingChangeFn>,
}

impl SortableHandle {
    pub fn destroy(&mut self) {
        self.sortable.destroy();
    }
}

impl Drop for SortableHandle {
    fn drop(&mut self) {
        self.sortable.destroy();
    }
}

impl SortableService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_sortable(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>,
        options: SortableOptions,
    ) -> Result<SortableHandle, ()> {
        let handler = Box::new(move |old_index: i32, new_index: i32| {
            callback.emit(SortingChange::new(
                old_index as Sorting,
                new_index as Sorting,
            ));
        });

        let closure = Closure::wrap(handler as Box<SortingChangeFn>);
        let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
        let wrapper = SortableWrapper::new(&element, closure.as_ref().unchecked_ref(), &options_js);

        Ok(SortableHandle {
            sortable: wrapper,
            _closure: closure,
        })
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SortableOptions {
    pub delay: i32,
    pub handle: Option<String>,
    pub force_fallback: bool,
}
