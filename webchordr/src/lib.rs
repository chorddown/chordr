#![recursion_limit = "256"]
extern crate stdweb;

mod components;

use stdweb::js;
use log::info;
use log::error;

use failure::Error;
use serde_derive::Deserialize;
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{StorageService, Area};

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use libchordr::prelude::*;

use crate::components::song_list::Item;
use crate::components::song_view::SongView;

pub enum Format {
    Json,
    Chorddown,
}

#[allow(dead_code)]
pub struct Model {
    fetch_service: FetchService,
    storage_service: StorageService,
    //    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    fetching: bool,
    song_list: Option<SongList>,
    //    data: Option<u32>,
    song_meta: Option<SongMeta>,
    song_data: Option<String>,
    catalog: Option<Catalog>,
    current_song: Option<Song>,
    show_menu: bool,
    ft: Option<FetchTask>,
//    ws: Option<WebSocketTask>,
}

pub enum Msg {
    OpenSongInMainView(SongId),
    FetchCatalogReady(Result<Catalog, Error>),
    ToggleMenu,
    Reload,
    Ignore,
}


/// This type is used to parse data from `./static/songs.json` file and
/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
pub struct SongListResponse {
    song_list: Vec<SongMeta>
}

impl Model {
    fn view_data(&self) -> Html {
        if let Some(song) = &self.current_song {
            match song.file_type() {
                FileType::Jpeg => self.view_image(song),
                FileType::Chorddown => self.view_chorddown(song)
            }
        } else {
            html! {
                <p>{ "Select a song" }</p>
            }
        }
    }


    fn view_image(&self, song: &Song) -> Html {
        let image_uri = format!("/songs/{}.{}", song.id(), song.file_type());

        html! {<img src=image_uri class="song-image" />}
    }

    fn view_chorddown(&self, song: &Song) -> Html {
        html! {<SongView song=song />}
    }

    fn view_song_list(&self) -> Html {
        let render = |song: &Song| {
            let song = song.clone();
            let onclick = self.link.callback(|song_id: SongId| Msg::OpenSongInMainView(song_id));
            html! { <Item song=song onclick=onclick/> }
        };

        return (match &self.catalog {
            Some(c) => {
                html! {
                    <div class="song-list">
                        { for c.iter().map(render) }
                     </div>
                }
            }
            None => html! {}
        }) as Html;
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);
        let reload_songs = self.link.callback(|_| Msg::Reload);

        (if self.show_menu {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "→" }</button>
                    <button class="reload-songs" onclick=reload_songs>{ "⟲ Reload2 Songs" }</button>
                </footer>
            }
        } else {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "︎←" }</button>
                </footer>
            }
        }) as Html
    }

    fn fetch_catalog(&mut self, no_cache: bool) {
        use stdweb::web::Date;

        let callback = self.link.callback(
            move |response: Response<Json<Result<Catalog, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchCatalogReady(data)
                } else {
                    error!("Could not fetch catalog: {:?}", meta);
                    Msg::Ignore
                }
            },
        );

        info!("Fetch catalog");
        let uri_base = "/catalog.json".to_owned();
        let uri = if no_cache || true {
            uri_base + &format!("?{}", Date::now())
        } else {
            uri_base
        };
        let request = Request::get(uri).body(Nothing).unwrap();
        self.ft = Some(self.fetch_service.fetch(request, callback));
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_service: FetchService::new(),
            storage_service: StorageService::new(Area::Local),
            link,
            fetching: false,
            show_menu: true,
            song_list: None,
            song_meta: None,
            song_data: None,
            current_song: None,
            catalog: None,
            ft: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_catalog(false);

        false
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenSongInMainView(song_id) => {
                info!("Change song to {}", song_id);
                self.current_song = match &self.catalog {
                    Some(c) => c.get(song_id).cloned(),
                    None => {
                        error!("Catalog not loaded yet");
                        None
                    }
                }
            }
            Msg::FetchCatalogReady(response) => {
                self.fetching = false;
                self.catalog = response.ok();
            }
            Msg::Ignore => {
                return false;
            }
            Msg::ToggleMenu => {
                self.show_menu = !self.show_menu;
            }
            Msg::Reload => {
                js! {
                    top.frames.location.reload()
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        let mut menu_classes = vec!["menu"];
        let _ = if self.show_menu {
            menu_classes.push("-visible");
        } else {
            menu_classes.push("-hidden");
        };

        let main_classes = if self.show_menu {
            "-menu-visible"
        } else {
            "-menu-hidden"
        };

        let song_list = if self.show_menu {
            self.view_song_list()
        } else {
            html! {}
        };

        html! {
            <main class=main_classes>
                <nav class=menu_classes>
                    { song_list }
                    { self.view_nav_footer() }
                </nav>
                <div class="content">
                    { self.view_data() }
                </div>
            </main>
        }
    }
}
