use std::rc::Rc;

use log::info;
use yew::prelude::*;
use yew::{Component, ComponentLink, ShouldRender};

use libchordr::models::catalog::*;
use libchordr::models::list::ListEntryTrait;
use libchordr::prelude::{Song, SongData, SongSorting};
use webchordr_song_list::Item as SongItem;

use crate::search::SearchUtility;

use self::link::SongSearchLink;

mod link;

pub struct SongSearch {
    search: String,
    props: SongSearchProps,
    /// Utility object
    link: ComponentLink<Self>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SongSearchProps {
    pub catalog: Rc<Catalog>,
    pub show_back_button: bool,
}

impl SongSearch {
    /// Return the [Song]s from the [Catalog] filtered by [self.search]
    fn get_filtered_songs(&self) -> Vec<&Song> {
        SearchUtility::search_catalog_by_term(&self.props.catalog, &self.search).sort_by_title()
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
