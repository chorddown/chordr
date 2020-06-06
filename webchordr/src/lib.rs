// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
pub mod constants;
mod data_exchange;
mod errors;
mod events;
mod fetch_helper;
mod handler_traits;
mod helpers;
mod persistence;
mod route;
mod sortable_service;
#[cfg(test)]
mod test_helpers;

pub use errors::WebError;
pub use fetch_helper::*;
use libchordr::prelude::{Setlist as LibchordrSetlist, SetlistEntry};

pub type Setlist = LibchordrSetlist<SetlistEntry>;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// This is the entry point for the web app
#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
