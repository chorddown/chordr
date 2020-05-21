#![recursion_limit = "128000"]
// extern crate stdweb;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod data_exchange;
mod errors;
mod events;
mod helpers;
mod route;
mod sortable_service;
#[cfg(test)]
mod test_helpers;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
