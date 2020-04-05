mod item;

pub use self::item::Item;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::*;
use log::info;
use log::error;
use std::rc::Rc;
use yew::prelude::*;
use stdweb::web::HtmlElement;
use crate::events::{SortingChange, Event};
use crate::sortable_service::{SortableService, SortableHandle};
use crate::events::setlist_events::SetlistEvent;

#[derive(Properties, PartialEq, Clone)]
pub struct SongListProps {
    pub songs: Rc<Setlist<SetlistEntry>>,
    pub sortable: bool,

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
    sortable_handle: Option<SortableHandle>,
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
            sortable_handle: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if self.props.sortable {
            self.make_sortable();

            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistChangeSorting(e) => {
                info!("Handle Setlist sorting change: Move {} to {}", e.old_index(), e.new_index());
                self.props.on_setlist_change.emit(Event::SetlistEvent(SetlistEvent::SortingChange(e)));
                self.props.songs = Rc::new(Setlist::new());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;

            if self.props.sortable {
                self.make_sortable();
            } else {
                self.make_unsortable();
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let songs = &self.props.songs;
        let render = |song: &SetlistEntry| {
            let key = song.title();

            html! { <Item<SetlistEntry> key=key song=song.clone() /> }
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

impl SongList {
    fn make_sortable(&mut self) {
        match self.sortable_handle {
            None =>
                if let Some(element) = self.node_ref.cast::<HtmlElement>() {
                    self.sortable_handle = self.sortable_service
                        .make_sortable(element, self.link.callback(|e| Msg::SetlistChangeSorting(e)))
                        .ok();
                },
            Some(_) => { /* Element is already sortable */ }
        }
    }

    fn make_unsortable(&mut self) {
        if let Some(mut handle) = self.sortable_handle.take() {
            if let Err(e) = handle.destroy() {
                error!("{}", e);
            }
        };
    }
}
