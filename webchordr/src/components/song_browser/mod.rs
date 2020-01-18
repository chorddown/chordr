mod index;
mod index_item;
mod link;

use self::index::*;
use self::index_item::IndexItem;
use self::link::SongBrowserLink;
use crate::components::song_list::Item as SongItem;
use libchordr::models::catalog::Catalog;
use libchordr::models::song_data::SongData;
use libchordr::prelude::Song;
use log::info;
use yew::prelude::*;
use yew::{Component, ComponentLink, ShouldRender};

pub struct SongBrowser {
    props: SongBrowserProps,
}

const SONG_BROWSER_PLACEHOLDER: &'static str = "_";

#[derive(Properties, PartialEq)]
pub struct SongBrowserProps {
    #[props(required)]
    pub chars: String,

    #[props(required)]
    pub catalog: Catalog,
}

impl SongBrowser {
    /// Return the [Song]s from the [Catalog] filtered by [props.chars]
    fn get_filtered_songs(&self) -> Vec<&Song> {
        if self.has_chars() {
            let chars = &self.props.chars;
            self.props
                .catalog
                .iter()
                .filter(|s| str::starts_with(&s.title().to_lowercase(), chars))
                .collect()
        } else {
            self.props.catalog.iter().collect()
        }
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
            let sub_string = sub_string(chars, chars.len() - 1);
            let parameter = if sub_string.is_empty() {
                SONG_BROWSER_PLACEHOLDER
            } else {
                sub_string.as_str()
            };

            let href = format!("#/song-browser/{}", parameter);

            html! { <a class="song-browser-back -inline" href=href><i class="im im-angle-left"></i>{ "Back" }</a> }
        } else {
            html! {}
        }) as Html
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
        info!("change: props.chars: {}", props.chars);

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let chars = &self.props.chars;

        info!("props.chars: {}", chars);

        let render_index_item = |index| {
            html! {
                <div class="col-3">
                    <IndexItem class="song-browser-index-item grid-button"
                        index=index/>
                </div>
            }
        };
        let render_song_item = |song| {
            html! {
                <div class="col-3">
                    <SongItem class="song-item grid-button"
                        song=song/>
                </div>
            }
        };

        let songs = self.get_filtered_songs();

        (if songs.len() > 24 || !self.has_chars() {
            html! {
                <div class="song-browser-index-list">
                    {self.render_header()}
                    <div class="row grid">
                        { for self.get_indexes_for_filtered_songs().into_iter().map(render_index_item) }
                    </div>
                    {self.get_back_link()}
                </div>
            }
        } else {
            html! {
                <div class="song-browser-song-list song-list">
                    {self.render_header()}
                    <div class="row grid">
                        { for songs.into_iter().map(render_song_item) }
                    </div>
                    {self.get_back_link()}
                </div>
            }
        }) as Html
    }
}

fn sub_string(input: &str, length: usize) -> String {
    input.chars().take(length).collect()
}
