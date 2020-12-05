#![recursion_limit = "256"]

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod config;
mod connection;
pub mod constants;
mod data_exchange;
mod errors;
mod events;
mod fetch_helper;
mod handler;
mod handler_traits;
mod helpers;
mod lock;
mod persistence;
mod route;
mod session;
mod sortable_service;
mod state;
#[cfg(test)]
mod test_helpers;

pub use errors::WebError;
pub use fetch_helper::*;
use libchordr::prelude::Setlist;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// This is the entry point for the web app
#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<handler::Handler>();
    Ok(())
}
