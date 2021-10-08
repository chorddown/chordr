use std::rc::Rc;

use yew::prelude::*;
use yew::{Component, ComponentLink, ShouldRender};

use libchordr::models::catalog::*;
use libchordr::models::song_data::SongData;
use libchordr::prelude::SongSorting;
use libchordr::prelude::{ListEntryTrait, Song};

use webchordr_song_list::Item as SongItem;

use self::index::*;
use self::index_item::IndexItem;
use self::link::SongBrowserLink;

mod index;
mod index_item;
mod link;

pub struct SongBrowser {
    props: SongBrowserProps,
}

const SONG_BROWSER_PLACEHOLDER: &'static str = "_";

#[derive(Properties, PartialEq, Clone)]
pub struct SongBrowserProps {
    pub chars: String,
    pub catalog: Rc<Catalog>,
}

impl SongBrowser {
    /// Return the [Song]s from the [Catalog] filtered by [props.chars]
    fn get_filtered_songs(&self) -> Vec<&Song> {
        let songs: Vec<&Song> = if self.has_chars() {
            let chars = &self.props.chars;
            self.props
                .catalog
                .iter()
                .filter(|s| str::starts_with(&s.title().to_lowercase(), chars))
                .collect()
        } else {
            self.props.catalog.iter().collect()
        };

        songs.sort_by_title()
    }

    /// Return the indexes for the filtered [Song]s
    fn get_indexes_for_filtered_songs(&self) -> Vec<Index> {
        let root_chars = if self.has_chars() {
            &self.props.chars
        } else {
            ""
        };
        build_indexes(self.get_filtered_songs(), root_chars)
    }

    fn has_chars(&self) -> bool {
        let chars = &self.props.chars;

        !chars.is_empty() && chars != SONG_BROWSER_PLACEHOLDER
    }

    fn get_back_link(&self) -> Html {
        (if self.has_chars() {
            let chars = &self.props.chars;
            let parameter = self.get_back_link_parameter(chars);

            let href = format!("#/song-browser/{}", parameter);

            html! { <a class="song-browser-back back-link -inline" href=href><i class="im im-angle-left"></i>{ "Back" }</a> }
        } else {
            html! {}
        }) as Html
    }

    fn get_back_link_parameter(&self, chars: &str) -> String {
        if !self.has_chars() {
            return SONG_BROWSER_PLACEHOLDER.to_owned();
        }

        let count = char_count(chars);
        if count < 1 {
            return SONG_BROWSER_PLACEHOLDER.to_owned();
        }
        let sub_string = sub_string(chars, count - 1);
        if sub_string.is_empty() {
            SONG_BROWSER_PLACEHOLDER.to_owned()
        } else {
            sub_string
        }
    }

    fn render_header(&self) -> Html {
        let header_suffix = if self.has_chars() {
            html! { <>{":"} <span class="song-browser-header-suffix">{&self.props.chars}</span></> }
        } else {
            html! {}
        };

        html! { <h1><SongBrowserLink />{"Browse Songs"}{header_suffix}</h1> }
    }
}

impl Component for SongBrowser {
    type Message = ();
    type Properties = SongBrowserProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SongBrowser { props }
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
        let songs = self.get_filtered_songs();

        if songs.len() > 24 || !self.has_chars() {
            self.render_index()
        } else {
            self.render_songs(songs)
        }
    }
}

impl SongBrowser {
    fn render_index(&self) -> Html {
        let render_index_item = |index: Index| {
            let key = index.chars.clone();

            html! {
                <div class="col-xs-12 col-sm-6 col-3">
                    <IndexItem class="song-browser-index-item grid-button"
                        key=key
                        index=index/>
                </div>
            }
        };

        let indexes_for_filtered_songs = self.get_indexes_for_filtered_songs();

        html! {
            <div class="song-browser-index-list">
                {self.render_header()}
                <div class="row grid">
                    { for indexes_for_filtered_songs.into_iter().map(render_index_item) }
                </div>
                {self.get_back_link()}
            </div>
        }
    }

    fn render_songs(&self, songs: Vec<&Song>) -> Html {
        let render_song_item = |song: &Song| {
            let data_key = song.title();
            let song_id = song.id();
            let key = song_id.as_str();

            html! {
                <div class="col-xs-12 col-6">
                    <SongItem<Song> class="song-item grid-button"
                        key=key
                        data_key=data_key
                        song=song.clone()/>
                </div>
            }
        };

        html! {
            <div class="song-browser-song-list song-list">
                {self.render_header()}
                <div class="row grid">
                    { for songs.into_iter().map(render_song_item) }
                </div>
                {self.get_back_link()}
            </div>
        }
    }
}
