use crate::handler::Handler;
use log::debug;
use webchordr_common::route::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: &AppRoute) -> Html {
    debug!("Switch Routes {:?}", route);
    let route = route.clone();
    html! {<Handler {route}/>}
}

#[function_component(Main)]
pub fn entry() -> Html {
    (html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }) as Html
}
