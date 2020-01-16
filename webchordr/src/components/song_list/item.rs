use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::helpers::Class;

#[derive(Properties, PartialEq)]
pub struct SongListItemProps {
    #[props(required)]
    pub song: Song,

    pub class: Class,
}

#[allow(dead_code)]
pub struct Item {
    /// State from the parent
    props: SongListItemProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for Item {
    type Message = ();
    type Properties = SongListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> VNode {
        let title = &self.props.song.title();
        let href = format!("#/song/{}", self.props.song.id());
        let class = self.props.class.or("song-item");
        //("song-item".into());

        html! { <a role="button" class=class href=href>{ title }</a> }
    }
}
