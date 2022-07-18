use log::info;
use web_sys::HtmlElement;
use yew::prelude::*;

use libchordr::models::song_list::SongList as SongListLibModel;
use libchordr::prelude::*;
use webchordr_events::setlist_events::SetlistEvent;
use webchordr_events::{Event, SortingChange};

use crate::sortable_service::{SortableHandle, SortableOptions, SortableService};

pub use self::item::Item;

mod item;
mod sortable_service;

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
    clear: bool,
    node_ref: NodeRef,
    sortable_service: SortableService,
    sortable_handle: Option<SortableHandle>,
}

impl Component for SongList {
    type Message = Msg;
    type Properties = SongListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            clear: false,
            node_ref: NodeRef::default(),
            sortable_service: SortableService::new(),
            sortable_handle: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetlistChangeSorting(e) => {
                let sorting_change = self.patch_sorting_change(ctx, e);
                if sorting_change.new_index() != sorting_change.old_index() {
                    ctx.props().on_setlist_change.emit(Event::SetlistEvent(
                        SetlistEvent::SortingChange(sorting_change),
                    ));
                    // Tell the view to clear the list
                    self.clear = true;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().sortable {
            self.make_sortable(ctx.link().callback(Msg::SetlistChangeSorting));
        } else {
            self.make_unsortable();
        }
        // Set clear to false to render the real Songs
        self.clear = false;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let empty_list = SongListModel::new();
        let songs = if self.clear {
            &empty_list
        } else {
            &props.songs
        };
        let sortable = props.sortable;
        let highlighted_song_id = &props.highlighted_song_id;
        let render = |song: SetlistEntry| {
            let data_key = song.title();
            let song_id = song.id();
            let key = song_id.as_str();

            let highlight = if let Some(highlighted_song_id) = highlighted_song_id {
                &song.id() == highlighted_song_id
            } else {
                false
            };

            html! { <Item<SetlistEntry> key={key} data_key={data_key} song={song} sortable={sortable} highlight={highlight} /> }
        };

        let entries = songs.clone().into_iter();

        info!("Redraw song list {:?}", songs);

        (html! {
            <div class="song-list" ref={self.node_ref.clone()}>
                {for entries.map(render)}
            </div>
        }) as Html
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if ctx.props().sortable {
            self.make_sortable(ctx.link().callback(Msg::SetlistChangeSorting));
        }
    }
}

impl SongList {
    fn make_sortable(&mut self, callback: Callback<SortingChange>) {
        match self.sortable_handle {
            None => {
                if let Some(element) = self.node_ref.cast::<HtmlElement>() {
                    let options = SortableOptions {
                        handle: Some(".sortable-handle".into()),
                        force_fallback: true,
                        ..Default::default()
                    };
                    self.sortable_handle = self
                        .sortable_service
                        .make_sortable(element, callback, options)
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
    /// The JS library may report that the element was moved to index `ctx.props().songs.len()`. If
    /// that's the case, patch the value
    fn patch_sorting_change(&self, ctx: &Context<Self>, e: SortingChange) -> SortingChange {
        let song_count = ctx.props().songs.len();
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
