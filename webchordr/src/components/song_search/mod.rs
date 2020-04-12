mod link;

use crate::components::song_list::Item as SongItem;
use libchordr::models::catalog::Catalog;
use libchordr::models::song_data::SongData;
use libchordr::prelude::Song;
use log::info;
use yew::prelude::*;
use yew::{Component, ComponentLink, ShouldRender};
use self::link::SongSearchLink;

pub struct SongSearch {
    search: String,
    props: SongSearchProps,
    /// Utility object
    link: ComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SongSearchProps {
    pub catalog: Catalog,
    pub show_back_button: bool,
}

impl SongSearch {
    /// Return the [Song]s from the [Catalog] filtered by [self.search]
    fn get_filtered_songs(&self) -> Vec<&Song> {
        let mut songs: Vec<&Song> = if self.search.is_empty() {
            self.props.catalog.iter().collect()
        } else {
            let search_normalized = self.search.to_lowercase();
            self.props
                .catalog
                .iter()
                .filter(|s| str::contains(&s.title().to_lowercase(), &search_normalized))
                .collect()
        };

        // Todo: Do not re-sort on each render
        songs.sort_by(|a, b| a.title().partial_cmp(&b.title()).unwrap());
        songs
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
                       value=&self.search
                       oninput=self.link.callback(|e: InputData| Msg::SearchChange(e.value))
                       placeholder="Search"/>
            </>
        }
    }
}

pub enum Msg {
    SearchChange(String),
}

impl Component for SongSearch {
    type Message = Msg;
    type Properties = SongSearchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SongSearch {
            props,
            link,
            search: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SearchChange(new_search) => {
                info!("New search {}", new_search);
                self.search = new_search;
            }
        }
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
        let render_song_item = |song: &Song| {
            let key = song.title();
            html! {
                <SongItem<Song> class="song-item button"
                    key=key
                    song=song/>
            }
        };

        let songs = self.get_filtered_songs();

        let inner = if songs.len() > 0 {
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
