use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Callback, Component, ComponentLink};

#[derive(Properties, PartialEq)]
pub struct SongListItemProps {
    #[props(required)]
    pub song: Song,

    #[props(required)]
    pub onclick: Callback<SongId>,
}

pub enum Msg {
    Clicked,
}

#[allow(dead_code)]
pub struct Item {
    /// State from the parent
    props: SongListItemProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for Item {
    type Message = Msg;
    type Properties = SongListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(self.props.song.id());
            }
        }
        false
    }

    fn view(&self) -> VNode {
        let title = &self.props.song.title();
        let c = self.link.callback(|_| Msg::Clicked);

        html! {
            <button onclick=c>{ title }</button>
        }
    }
}
