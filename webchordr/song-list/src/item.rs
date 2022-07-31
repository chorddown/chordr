use libchordr::prelude::*;
use std::marker::PhantomData;
use webchordr_common::components::link::Link;
use webchordr_common::helpers::Class;
use webchordr_common::route::{AppRoute, SongIdParam};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Component;

#[derive(Properties, PartialEq, Clone)]
pub struct SongListItemProps<S: SongData + Clone + PartialEq> {
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

fn get_class<'a, S: SongData + Clone + PartialEq>(props: &SongListItemProps<S>) -> Class {
    let base_class = props.class.or("song-item");
    let class = if props.highlight {
        base_class.add("-highlight")
    } else {
        base_class
    };

    let class = if props.sortable {
        class.add("-sortable")
    } else {
        class
    };

    if props.draggable {
        class.add("-draggable")
    } else {
        class
    }
}

#[function_component(Item)]
pub fn render_item<S: SongData + Clone + PartialEq>(props: &SongListItemProps<S>) -> Html {
    let title = &props.song.title();
    let key = &props.data_key;
    let id = props.song.id().to_string();
    let class = get_class(props);
    let draggable = if props.draggable { Some("true") } else { None };

    let to = AppRoute::Song {
        id: SongIdParam::from_song_id(&props.song.id()),
    };

    let link =
        html! { <Link role="button" class="discreet" data_key={key.clone()} {to}>{title}</Link> };

    let handle = if props.sortable {
        html! { <span class="sortable-handle">{"::"}</span> }
    } else {
        html! {}
    };

    html! { <div class={class} data-song-id={id} draggable={draggable}>{link}{handle}</div> }
}
