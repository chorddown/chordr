use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

use libchordr::prelude::*;

use webchordr_common::helpers::Class;
use webchordr_common::route::route;

#[derive(Properties, PartialEq, Clone)]
pub struct SongListItemProps<S: SongData + Clone> {
    pub song: S,
    pub data_key: String,

    #[prop_or_default]
    pub sortable: bool,

    #[prop_or_default]
    pub draggable: bool,

    #[prop_or_default]
    pub highlight: bool,

    #[prop_or_default]
    pub class: Class,
}

#[allow(dead_code)]
pub struct Item<S: SongData + PartialEq + 'static + Clone> {
    /// State from the parent
    props: SongListItemProps<S>,
    /// Utility object
    link: ComponentLink<Self>,
}

impl<S: SongData + PartialEq + 'static + Clone> Item<S> {
    fn get_class(&self) -> Class {
        let base_class = self.props.class.or("song-item");
        let class = if self.props.highlight {
            base_class.add("-highlight")
        } else {
            base_class
        };

        let class = if self.props.sortable {
            class.add("-sortable")
        } else {
            class
        };

        if self.props.draggable {
            class.add("-draggable")
        } else {
            class
        }
    }
}

impl<S: SongData + PartialEq + 'static + Clone> Component for Item<S> {
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
        let key = &self.props.data_key;
        let id = self.props.song.id().to_string();
        let href = route(&format!("song/{}", id));
        let class = self.get_class();
        let draggable = if self.props.draggable {
            "true"
        } else {
            "false"
        };
        let link =
            html! { <a role="button" class="discreet" data-key=key.clone() href=href>{title}</a> };

        (if self.props.sortable {
            html! { <div class=class data-song-id=id draggable=draggable>{link}<span class="sortable-handle">{"::"}</span></div> }
        } else {
            html! { <div class=class data-song-id=id draggable=draggable>{link}</div> }
        }) as Html
    }
}
