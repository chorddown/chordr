use std::rc::Rc;

use gloo_timers::callback::Timeout;
use log::{debug, info};
use yew::prelude::*;
use yew::{Component, ComponentLink, ShouldRender};

use libchordr::models::catalog::*;
use libchordr::models::list::ListEntryTrait;
use libchordr::prelude::{SearchIndex, Song, SongData, SongSorting};
use webchordr_song_list::Item as SongItem;

use self::link::SongSearchLink;

mod link;

pub struct SongSearch {
    search: String,
    props: SongSearchProps,
    /// Utility object
    link: ComponentLink<Self>,
    search_index: Option<SearchIndex>,
    timeout: Option<Timeout>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SongSearchProps {
    pub catalog: Rc<Catalog>,
    pub show_back_button: bool,
}

impl SongSearch {
    /// Return the [Song]s from the [Catalog] filtered by [self.search]
    fn get_filtered_songs(&self) -> Vec<&Song> {
        if self.search.is_empty() {
            self.props
                .catalog
                .iter()
                .collect::<Vec<&Song>>()
                .sort_by_title()
        } else {
            self.search_index
                .as_ref()
                .expect("Search index not built yet")
                .search_by_term(&self.search)
                .sort_by_title()
        }
    }

    fn needs_to_build_search_index(&self) -> bool {
        !self.search.is_empty() && !self.search.trim().is_empty() && self.search_index.is_none()
    }

    fn get_back_link(&self) -> Html {
        (if self.props.show_back_button {
            let href = "#/";

            html! { <a class="song-search-back back-link -inline" href=href><i class="im im-angle-left"></i>{ "Back" }</a> }
        } else {
            html! {}
        }) as Html
    }

    fn render_filter(&self) -> Html {
        html! {
            <>
                <h1><SongSearchLink />{"Search Songs"}</h1>
                <input type="search"
                       value=self.search.clone()
                       oninput=self.link.callback(|e: InputData| Msg::SearchChange(e.value))
                       placeholder="Search"/>
            </>
        }
    }
}

pub enum Msg {
    SearchChange(String),
    Debounce(String),
    BuildSearchIndex,
}

impl Component for SongSearch {
    type Message = Msg;
    type Properties = SongSearchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SongSearch {
            props,
            link,
            search: String::new(),
            search_index: None,
            timeout: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SearchChange(new_search) => {
                // Clear the previous timer if one exists
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                }

                // Debounce the handling of user input
                let debounced = self.link.callback(|new_search| Msg::Debounce(new_search));
                self.timeout = Some(Timeout::new(100, move || {
                    debounced.emit(new_search);
                }));

                false
            }

            Msg::Debounce(new_search) => {
                info!("New search {}", new_search);
                self.search = new_search;

                true
            }

            Msg::BuildSearchIndex => {
                self.search_index = Some(build_search_index_from_props(&self.props));

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            let catalog_changed = self.props.catalog.revision() != props.catalog.revision();
            if catalog_changed {
                self.search_index = None;
            }
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let render_song_item = |song: &Song| {
            let data_key = song.title();
            let song_id = song.id();
            let key = song_id.as_str();

            html! {
                <SongItem<Song> class="song-item button"
                    key=key
                    data_key=data_key
                    song=song.clone()/>
            }
        };

        if self.needs_to_build_search_index() {
            self.link.send_message(Msg::BuildSearchIndex);

            return html! {};
        }

        let songs = self.get_filtered_songs();

        let inner = if !songs.is_empty() {
            html! { <div class="song-search-results">{ for songs.into_iter().map(render_song_item) }</div>}
        } else {
            html! { <div class="song-search-results -no-results">{"No matching songs found"}</div>}
        };
        html! {
            <div class="song-search-song-list song-list">
                {self.render_filter()}
                {inner}
                {self.get_back_link()}
            </div>
        }
    }
}

fn build_search_index_from_props(props: &SongSearchProps) -> SearchIndex {
    debug!("Build search index");
    let search_index = SearchIndex::build_for_catalog(props.catalog.clone());
    debug!("Did build search index");

    search_index
}
