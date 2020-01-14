#![recursion_limit = "128000"]
extern crate stdweb;

mod components;
mod route;
mod helpers;

use log::error;
use log::info;
use stdweb::js;
use failure::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use libchordr::prelude::*;
use crate::components::song_list::Item;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::components::song_browser::{SongBrowser, SongBrowserProps};
use crate::route::AppRoute;

pub enum Format {
    Json,
    Chorddown,
}

#[allow(dead_code)]
pub struct Model {
    fetch_service: FetchService,
    storage_service: StorageService,
    link: ComponentLink<Model>,
    fetching: bool,
    song_list: Option<SongList>,
    song_meta: Option<SongMeta>,
    song_data: Option<String>,
    catalog: Option<Catalog>,
    current_song: Option<Song>,
    show_menu: bool,
    ft: Option<FetchTask>,

    route_service: RouteService<()>,
    route: Route<()>,
}

pub enum Msg {
    OpenSongInMainView(SongId),
    FetchCatalogReady(Result<Catalog, Error>),
    FetchCatalog(bool),
    ToggleMenu,
    Reload,
    Ignore,
    RouteChanged(Route<()>),
}

impl Model {
    fn route(&self) -> Html {
        match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Song(id)) => self.view_song(id),
            Some(AppRoute::SongBrowser(chars)) => self.view_song_browser(chars),
            Some(AppRoute::Index) => html! {<><StartScreen/>{self.view_song_browser("".to_owned())}</>},
            None => html! {<><StartScreen/>{self.view_song_browser("".to_owned())}</>},
        }
    }

    fn view_song(&self, song_id: SongId) -> Html {
        match &self.catalog {
            Some(catalog) => {
                match catalog.get(song_id) {
                    Some(song) => html! {<SongView song=song/>},
                    None => html! {},
                }
            }
            None => html! {}
        }
    }

    fn view_song_browser(&self, chars: String) -> Html {
        (match &self.catalog {
//            Some(catalog) => SongBrowser::create(SongBrowserProps{}).view()
            Some(catalog) => {
                info!("New chars from router: {}", chars);
                html! {<SongBrowser chars=chars catalog=catalog/>}
            }
            None => html! {},
        }) as Html
    }

    fn view_song_list(&self) -> Html {
        let render = |song: &Song| {
            let song = song.clone();
            html! { <Item song=song/> }
        };

        return (match &self.catalog {
            Some(c) => {
                html! {
                    <div class="song-list">
                        { for c.iter().map(render) }
                     </div>
                }
            }
            None => html! {},
        }) as Html;
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);

        (if self.show_menu {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "→" }</button>
                    <a role="button" class="home" href="/" title="Reload the song catalog and go to home screen">
                        <i class="im im-home"></i>
                        <span>{ "Home" }</span>
                    </a>
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
        let request = Request::get(uri).body(Nothing).expect("Request could not be built");
        self.ft = Some(self.fetch_service.fetch(request, callback));
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let route = Route::from(route);
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);

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
                    { self.route() }
                </div>
            </main>
        }
    }
}
