mod item;

pub use self::item::Item;
use libchordr::prelude::*;
use log::info;
use yew::prelude::*;
use std::rc::Rc;
use crate::setlist::Setlist;

#[derive(Properties, PartialEq)]
pub struct SongListProps {
    #[props(required)]
    pub songs: Rc<Setlist>,
}

pub struct SongList {
    props: SongListProps,
}

impl Component for SongList {
    type Message = ();
    type Properties = SongListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let songs = &self.props.songs;
        let render = |song: &Song| {
            html! { <Item song=song/> }
        };

        info!(
            "Redraw song list {:?}",
            songs.iter().map(|s| s.id()).collect::<Vec<String>>()
        );

        (html! { for songs.iter().map(render) }) as Html
    }
}
