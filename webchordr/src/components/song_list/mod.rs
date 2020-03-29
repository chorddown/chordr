mod item;

pub use self::item::Item;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::*;
use log::info;
use std::rc::Rc;
use yew::prelude::*;
use stdweb::web::HtmlElement;
use crate::events::{SortingChange, Event};
use crate::sortable_service::SortableService;
use crate::events::setlist_events::SetlistEvent;

#[derive(Properties, PartialEq)]
pub struct SongListProps {
    #[props(required)]
    pub songs: Rc<Setlist<SetlistEntry>>,

    #[props(required)]
    pub on_setlist_change: Callback<Event>,
}

pub enum Msg {
    SetlistChangeSorting(SortingChange),
}

pub struct SongList {
    props: SongListProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    sortable_service: SortableService,
}

impl Component for SongList {
    type Message = Msg;
    type Properties = SongListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            sortable_service: SortableService::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(element) = self.node_ref.try_into::<HtmlElement>() {
            self.sortable_service.make_sortable(element, self.link.callback(|e| Msg::SetlistChangeSorting(e)));
        }

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistChangeSorting(e) => {
                info!("Handle Setlist sorting change: Move {} to {}", e.old_index(), e.new_index());
                self.props.on_setlist_change.emit(Event::SetlistEvent(SetlistEvent::SortingChange(e)));

                false
            }
        }
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
        let render = |song: &SetlistEntry| {
            html! { <Item<SetlistEntry> song=song.clone() /> }
        };

        info!(
            "Redraw song list {:?}",
            songs.iter().map(|s| s.id()).collect::<Vec<SongId>>()
        );

        (html! {
            <div ref=self.node_ref.clone()>
                {for songs.iter().map(render)}
            </div>
        }) as Html
    }
}
