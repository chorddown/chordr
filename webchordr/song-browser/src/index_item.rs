use super::index::Index;
use webchordr_common::components::link::Link;
use webchordr_common::helpers::Class;
use webchordr_common::route::AppRoute;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Component;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SongBrowserIndexItemProps {
    pub index: Index,
    pub class: Class,
}

#[allow(dead_code)]
pub struct IndexItem {}

impl Component for IndexItem {
    type Message = ();
    type Properties = SongBrowserIndexItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let index = &ctx.props().index;
        let class = ctx.props().class.or("song-browser-index-item").to_classes();

        let to = AppRoute::SongBrowser {
            chars: index.chars.to_string(),
        };

        html! {
            <Link class={class} role="button" {to}>
                <span class="index-item-chars">{ &index.chars }</span>
                <span class="index-item-count">{ format!("({})", index.count) }</span>
            </Link>
        }
    }
}
