use crate::helpers::Class;
use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

#[derive(Properties, PartialEq)]
pub struct SongListItemProps<S: SongData> {
    #[props(required)]
    pub song: S,

    pub class: Class,
}

#[allow(dead_code)]
pub struct Item<S: SongData + PartialEq + 'static> {
    /// State from the parent
    props: SongListItemProps<S>,
    /// Utility object
    link: ComponentLink<Self>,
}

impl<S: SongData + PartialEq + 'static> Component for Item<S> {
    type Message = ();
    type Properties = SongListItemProps<S>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> VNode {
        let title = &self.props.song.title();
        let href = format!("#/song/{}", self.props.song.id());
        let class = self.props.class.or("song-item");

        html! { <a role="button" class=class href=href>{ title }</a> }
    }
}
