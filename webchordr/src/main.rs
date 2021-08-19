#![recursion_limit = "256"]

pub use errors::WebError;
pub use fetch_helper::*;
use libchordr::prelude::Setlist;

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
mod search;
mod session;
mod sortable_service;
mod state;
#[cfg(test)]
mod test_helpers;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<handler::Handler>();
}
