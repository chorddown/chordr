#![recursion_limit = "256"]

use crate::routing::Main;
pub use fetch_helper::*;
use webchordr_common::config;
use webchordr_common::connection;
use webchordr_common::data_exchange;
use webchordr_common::errors;
use webchordr_common::fetch_helper;
use webchordr_common::helpers;
use webchordr_common::session;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod handler;
mod handler_traits;
mod ipc;
mod routing;
mod service;
mod state;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
