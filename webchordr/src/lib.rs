#![recursion_limit = "128000"]
extern crate stdweb;

mod components;
mod helpers;
mod route;

use crate::components::song_browser::SongBrowser;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::route::AppRoute;
use failure::Error;
use libchordr::prelude::*;
use log::error;
use log::info;
use stdweb::js;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use crate::components::nav::Nav;
use std::rc::Rc;
use crate::components::reload_section::ReloadSection;

const STORAGE_KEY_SET_LIST: &'static str = "net.cundd.chordr.set-list";

pub enum Format {
    Json,
    Chorddown,
}

#[allow(dead_code)]
pub struct App {
    fetch_service: FetchService,
    storage_service: StorageService,
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<App>,
    ft: Option<FetchTask>,

    show_menu: bool,
    fetching: bool,
    catalog: Option<Catalog>,
    current_song: Option<Song>,
    setlist: Vec<Song>,
}

pub enum Msg {
    OpenSongInMainView(SongId),
    FetchCatalogReady(Result<Catalog, Error>),
    FetchCatalog(bool),
    SetlistAdd(Song),
    SetlistRemove(Song),
    ToggleMenu,
    Reload,
    Ignore,
    RouteChanged(Route<()>),
}

impl App {
    fn route(&self) -> Html {
        (match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Song(id)) => self.view_song(id),
            Some(AppRoute::SongBrowser(chars)) => self.view_song_browser(chars),
            Some(AppRoute::Index) => {
                html! {<><StartScreen/>{self.view_song_browser("")}<ReloadSection /></>}
            }
            None => html! {<><StartScreen/>{self.view_song_browser("")}<ReloadSection /></>},
        }) as Html
    }

    fn view_song(&self, song_id: SongId) -> Html {
        if self.catalog.is_none() {
            return html! {};
        }

        let catalog = self.catalog.as_ref().unwrap();
        (match catalog.get(song_id) {
            Some(song) => {
                let add = self.link.callback(|s| Msg::SetlistAdd(s));
                let remove = self.link.callback(|s| Msg::SetlistRemove(s));
                let is_on_setlist = self.setlist.contains(song);

                info!("Song {} is on list? {}", song.id(), is_on_setlist);

                html! {
                    <SongView
                        song=song
                        enable_setlists=true
                        on_setlist_add=add
                        on_setlist_remove=remove
                        is_on_setlist=is_on_setlist
                    />
                }
            }
            None => html! {},
        }) as Html
    }

    fn view_song_browser<S: Into<String>>(&self, chars: S) -> Html {
        let chars = chars.into();
        (match &self.catalog {
            Some(catalog) => {
                info!("New chars from router: {}", chars);
                html! {<SongBrowser chars=chars catalog=catalog/>}
            }
            None => html! {},
        }) as Html
    }

    fn fetch_catalog(&mut self, no_cache: bool) {
        use stdweb::web::Date;

        let callback =
            self.link
                .callback(move |response: Response<Json<Result<Catalog, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchCatalogReady(data)
                    } else if no_cache {
                        info!("Could not fetch catalog without cache. Try again");
                        Msg::FetchCatalog(false)
                    } else {
                        error!("Could not fetch catalog: {:?}", meta);
                        Msg::Ignore
                    }
                });

        let uri_base = "/catalog.json".to_owned();
        let uri = if no_cache {
            uri_base + &format!("?{}", Date::now())
        } else {
            uri_base
        };
        let request = Request::get(uri)
            .body(Nothing)
            .expect("Request could not be built");
        self.ft = Some(self.fetch_service.fetch(request, callback));
    }

    fn setlist_add(&mut self, song: Song) {
        self.setlist.push(song);
        self.storage_service
            .store(STORAGE_KEY_SET_LIST, Json(&self.setlist));
    }

    fn setlist_remove(&mut self, song: Song) {
        match self.setlist.iter().position(|x| *x == song) {
            Some(pos) => {
                info!("Remove song {} from set-list", song.id());
                self.setlist.remove(pos);
            }
            None => {
                info!("Could not find song {} in set-list", song.id());
            }
        }
        self.storage_service
            .store(STORAGE_KEY_SET_LIST, Json(&self.setlist));
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let route = Route::from(route);
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);

        let storage_service = StorageService::new(Area::Local);
        let setlist =
            if let Json(Ok(restored_model)) = storage_service.restore(STORAGE_KEY_SET_LIST) {
                restored_model
            } else {
                Vec::new()
            };

        Self {
            fetch_service: FetchService::new(),
            storage_service,
            link,
            fetching: false,
            show_menu: true,
            current_song: None,
            catalog: None,
            ft: None,
            setlist,
            route_service,
            route,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_catalog(true);

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
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
            Msg::FetchCatalog(no_cache) => self.fetch_catalog(no_cache),
            Msg::SetlistAdd(song) => self.setlist_add(song),
            Msg::SetlistRemove(song) => self.setlist_remove(song),
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
        let main_classes = if self.show_menu {
            "-menu-visible"
        } else {
            "-menu-hidden"
        };

        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);
        let songs = Rc::new(self.setlist.clone());

        html! {
            <main class=main_classes>
                <Nav
                    show_menu=self.show_menu
                    songs=songs
                    on_toggle=toggle_menu
                />
                <div class="content">
                    { self.route() }
                </div>
            </main>
        }
    }
}
