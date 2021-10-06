use super::index::Index;
use webchordr_class::Class;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SongBrowserIndexItemProps {
    pub index: Index,
    pub class: Class,
}

#[allow(dead_code)]
pub struct IndexItem {
    /// State from the parent
    props: SongBrowserIndexItemProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for IndexItem {
    type Message = ();
    type Properties = SongBrowserIndexItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // info!("Change the Song Browser Item props: {:?}", props);
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        let index = &self.props.index;
        let href = format!("#/song-browser/{}", index.chars);
        let class = self.props.class.or("song-browser-index-item");

        html! {
            <a class=class role="button" href=href>
                <span class="index-item-chars">{ &index.chars }</span>
                <span class="index-item-count">{ format!("({})", index.count) }</span>
            </a>
        }
    }
}
