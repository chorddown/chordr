#![recursion_limit = "256"]

use log::info;
use web_sys::Document;
// pub use errors::WebError;
use crate::handler::Handler;
pub use fetch_helper::*;
use webchordr_common::config;
use webchordr_common::connection;
use webchordr_common::data_exchange;
use webchordr_common::errors;
use webchordr_common::fetch_helper;
use webchordr_common::helpers;
use webchordr_common::helpers::window;
use webchordr_common::session;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod handler;
mod handler_traits;
mod ipc;
mod route;
mod service;
mod state;

use webchordr_common::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: &AppRoute) -> Html {
    info!("Switch Routes {:?}", route);
    let route = route.clone();
    html! {<Handler {route}/>}
}

#[function_component(Main)]
fn entry() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    // let element = window()
    //     .document()
    //     .query_selector("#root")
    //     .expect("can't get #root node for rendering")
    //     .expect("can't unwrap #root node");
    // yew::App::<handler::Handler>::new().mount(element);
    // yew::run_loop();
}
