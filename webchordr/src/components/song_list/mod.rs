use log::info;
use web_sys::HtmlElement;
use yew::prelude::*;

use libchordr::models::song_list::SongList as SongListLibModel;
use libchordr::prelude::*;

use crate::events::setlist_events::SetlistEvent;
use crate::events::{Event, SortingChange};
use crate::sortable_service::{SortableHandle, SortableOptions, SortableService};

pub use self::item::Item;

mod item;

type SongListModel = SongListLibModel<SetlistEntry>;

#[derive(Properties, Clone)]
pub struct SongListProps {
    pub songs: SongListModel,
    pub sortable: bool,
    #[prop_or_default]
    pub highlighted_song_id: Option<SongId>,

    pub on_setlist_change: Callback<Event>,
}

impl PartialEq for SongListProps {
    fn eq(&self, other: &Self) -> bool {
        self.songs == other.songs
            && self.sortable == other.sortable
            && self.highlighted_song_id == other.highlighted_song_id
    }
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistChangeSorting(e) => {
                let sorting_change = self.patch_sorting_change(e);
                if sorting_change.new_index() != sorting_change.old_index() {
                    self.props.on_setlist_change.emit(Event::SetlistEvent(
                        SetlistEvent::SortingChange(sorting_change),
                    ));
                    self.props.songs = SongListModel::new();
                    true
                } else {
                    false
                }
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
        let sortable = self.props.sortable;
        let highlighted_song_id = &self.props.highlighted_song_id;
        let render = |song: SetlistEntry| {
            let key = song.title();

            let highlight = if let Some(highlighted_song_id) = highlighted_song_id {
                &song.id() == highlighted_song_id
            } else {
                false
            };

            html! { <Item<SetlistEntry> data_key=key song=song sortable=sortable highlight=highlight /> }
        };

        let entries = songs.clone().into_iter().collect::<Vec<SetlistEntry>>();

        info!("Redraw song list {:?}", songs);

        (html! {
            <div class="song-list" ref=self.node_ref.clone()>
                {for entries.into_iter().map(render)}
            </div>
        }) as Html
    }

    fn rendered(&mut self, _: bool) -> () {
        if self.props.sortable {
            self.make_sortable();
        }
    }
}

impl SongList {
    fn make_sortable(&mut self) {
        match self.sortable_handle {
            None => {
                if let Some(element) = self.node_ref.cast::<HtmlElement>() {
                    let mut options = SortableOptions::default();
                    options.handle = Some(".sortable-handle".into());
                    options.force_fallback = true;
                    self.sortable_handle = self
                        .sortable_service
                        .make_sortable(
                            element,
                            self.link.callback(|e| Msg::SetlistChangeSorting(e)),
                            options,
                        )
                        .ok();
                }
            }
            Some(_) => { /* Element is already sortable */ }
        }
    }

    fn make_unsortable(&mut self) {
        if let Some(mut handle) = self.sortable_handle.take() {
            handle.destroy()
        };
    }

    /// Patch the Sorting Change value
    ///
    /// The JS library may report that the element was moved to index `self.props.songs.len()`. If
    /// that's the case, patch the value
    fn patch_sorting_change(&self, e: SortingChange) -> SortingChange {
        let song_count = self.props.songs.len();
        if e.new_index() == song_count {
            let last_index = song_count - 1;
            info!(
                "Handle Setlist sorting change: Move {} to {} (patched: {})",
                e.old_index(),
                last_index,
                e.new_index()
            );
            SortingChange::new(e.old_index(), last_index)
        } else {
            info!(
                "Handle Setlist sorting change: Move {} to {}",
                e.old_index(),
                e.new_index()
            );
            e
        }
    }
}
