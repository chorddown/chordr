#![recursion_limit = "128000"]
extern crate stdweb;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod components;
mod helpers;
mod events;
mod errors;
mod route;
mod setlist_serializer_service;
mod sortable_service;

use crate::components::nav::Nav;
use crate::components::reload_section::ReloadSection;
use crate::components::song_browser::SongBrowser;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::route::{AppRoute, SetlistRoute};
use libchordr::models::setlist::{Setlist, SetlistEntry};
use libchordr::models::song_id::SongIdTrait;
use libchordr::models::song_settings::SongSettings;
use libchordr::prelude::*;
use log::{debug, error, info, warn};
use percent_encoding::percent_decode_str;
use std::rc::Rc;
use stdweb::js;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use crate::events::{Event, SortingChange, SetlistEvent};
use crate::components::song_search::SongSearch;
use crate::components::setlist::SetlistLoad;

const STORAGE_KEY_SETLIST: &'static str = "net.cundd.chordr.setlist";
const STORAGE_KEY_SETTINGS: &'static str = "net.cundd.chordr.settings";

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

    expand: bool,
    fetching: bool,
    catalog: Option<Catalog>,
    current_song: Option<Song>,
    setlist: Setlist<SetlistEntry>,
    settings: SongSettingsMap,
}

pub enum Msg {
    Event(Event),
    OpenSongInMainView(SongId),
    FetchCatalogReady(Result<Catalog, ::anyhow::Error>),
    FetchCatalog(bool),
    SongSettingsChange(SongId, SongSettings),
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
            Some(AppRoute::SongSearch) => self.view_song_search(true),
            Some(AppRoute::Setlist(route)) => self.view_setlist_route(route),
            Some(AppRoute::Index) => self.view_index(),
            None => self.view_index(),
        }) as Html
    }

    fn view_index(&self) -> Html {
        html! {
            <>
                <StartScreen />
                {self.view_song_browser("")}
                <ReloadSection />
                {self.view_song_search(false)}
            </>
        }
    }

    fn view_song(&self, song_id: SongId) -> Html {
        if self.catalog.is_none() {
            return html! {};
        }

        let catalog = self.catalog.as_ref().unwrap();
        if let Some(song) = catalog.get(song_id.clone()) {
            let add = self.link.callback(|s| Msg::Event(SetlistEvent::Add(s).into()));
            let remove = self.link.callback(|s| Msg::Event(SetlistEvent::Remove(s).into()));
            let change = self
                .link
                .callback(|s: (SongId, SongSettings)| Msg::SongSettingsChange(s.0, s.1));
            let is_on_setlist = self.setlist.contains(song);

            let song_settings = match self.settings.get(&song_id) {
                Some(s) => {
                    info!("Found settings for song {}: {:?}", song_id, s);
                    s.clone()
                }
                None => {
                    info!("No settings for song {} found in setlist", song_id);
                    SongSettings::new(0, Formatting::default())
                }
            };

            info!("Song {} is on list? {}", song.id(), is_on_setlist);

            return html! {
                <SongView
                    song=song
                    song_settings=song_settings
                    enable_setlists=true
                    on_setlist_add=add
                    on_setlist_remove=remove
                    on_settings_change=change
                    is_on_setlist=is_on_setlist
                />
            };
        }

        (match percent_decode_str(song_id.as_str()).decode_utf8() {
            Ok(decoded) => {
                let decoded = decoded.to_string();
                info!("Decoded song ID '{}' to '{}'", song_id, decoded);
                if &decoded != song_id.as_str() {
                    self.view_song(SongId::new(decoded))
                } else {
                    html! {}
                }
            }
            Err(e) => {
                error!("Could not decode the song ID {}", e);
                (html! {}) as Html
            }
        }) as Html
    }

    fn view_song_browser<S: Into<String>>(&self, chars: S) -> Html {
        let chars_as_string = chars.into();
        let chars = match percent_decode_str(&chars_as_string).decode_utf8() {
            Ok(decoded) => decoded.to_string(),
            Err(_) => chars_as_string,
        };

        (match &self.catalog {
            Some(catalog) => {
                info!("New chars from router: {}", chars);
                html! {<SongBrowser chars=chars catalog=catalog/>}
            }
            None => html! {},
        }) as Html
    }

    fn view_song_search(&self, show_back_button: bool) -> Html {
        (match &self.catalog {
            Some(catalog) => {
                html! {<SongSearch catalog=catalog show_back_button=show_back_button />}
            }
            None => html! {},
        }) as Html
    }

    fn view_setlist_route(&self, route: SetlistRoute) -> Html {
        (match route {
            SetlistRoute::Load { serialized_setlist } => {
                match &self.catalog {
                    None => html! {},
                    Some(catalog) => {
                        let replace = self.link.callback(|e| Msg::Event(e));
                        let catalog = Rc::new(catalog.clone());

                        html! {<SetlistLoad catalog=catalog serialized_setlist=serialized_setlist on_load=replace />}
                    }
                }
            }
        }) as Html
    }

    fn fetch_catalog(&mut self, no_cache: bool) {
        use stdweb::web::Date;

        let callback =
            self.link
                .callback(move |response: Response<Json<Result<Catalog, anyhow::Error>>>| {
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
        match self.fetch_service.fetch(request, callback) {
            Ok(task) => self.ft = Some(task),
            Err(e) => error!("Fetch Task count not be built: {:?}", e),
        }
    }

    fn handle_setlist_event(&mut self, event: SetlistEvent) {
        match event {
            SetlistEvent::SortingChange(v) => self.setlist_sorting_changed(v),
            SetlistEvent::Add(v) => self.setlist_add(v),
            SetlistEvent::Remove(v) => self.setlist_remove(v),
            SetlistEvent::Replace(v) => self.setlist_replace(v),
        }
    }

    fn setlist_add(&mut self, song: SetlistEntry) {
        let song_id = song.id();
        match self.setlist.add(song) {
            Ok(_) => debug!("Did add song to setlist {}", song_id),
            Err(e) => error!("Could not add song to setlist: {:?}", e),
        }
        self.storage_service
            .store(STORAGE_KEY_SETLIST, Json(&self.setlist));
    }

    fn setlist_remove(&mut self, song_id: SongId) {
        match self.setlist.remove_by_id(&song_id) {
            Ok(_) => info!("Removed song {} from setlist", song_id),
            Err(_) => warn!("Could not remove song {} from setlist", song_id),
        }
        self.storage_service
            .store(STORAGE_KEY_SETLIST, Json(&self.setlist));
    }

    fn setlist_replace(&mut self, setlist: Setlist<SetlistEntry>) {
        info!("Replace setlist {:?} with {:?}", self.setlist, setlist);
        self.setlist = setlist;
        self.storage_service
            .store(STORAGE_KEY_SETLIST, Json(&self.setlist));
    }

    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange) {
        match self.setlist.move_entry(sorting_change.old_index(), sorting_change.new_index()) {
            Ok(_) => {
                self.storage_service.store(STORAGE_KEY_SETLIST, Json(&self.setlist));
            }
            Err(e) => error!("{}", e)
        }
    }

    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) {
        self.settings.store(song_id, settings);
        self.storage_service
            .store(STORAGE_KEY_SETTINGS, Json(&self.settings));
    }

    fn get_setlist(storage_service: &StorageService) -> Setlist<SetlistEntry> {
        if let Json(Ok(restored_model)) = storage_service.restore(STORAGE_KEY_SETLIST) {
            restored_model
        } else {
            Setlist::new()
        }
    }
    fn get_settings(storage_service: &StorageService) -> SongSettingsMap {
        if let Json(Ok(restored_model)) = storage_service.restore(STORAGE_KEY_SETTINGS) {
            restored_model
        } else {
            SongSettingsMap::new()
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = Route::from(route_service.get_route());
        route_service.register_callback(link.callback(Msg::RouteChanged));

        let storage_service = StorageService::new(Area::Local).unwrap();
        let setlist = App::get_setlist(&storage_service);
        let settings = App::get_settings(&storage_service);

        Self {
            fetch_service: FetchService::new(),
            storage_service,
            link,
            fetching: false,
            expand: true,
            current_song: None,
            catalog: None,
            ft: None,
            settings,
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
            Msg::SongSettingsChange(song_id, settings) => {
                self.song_settings_change(song_id, settings)
            }
            Msg::Ignore => return false,
            Msg::ToggleMenu => {
                self.expand = !self.expand;
            }
            Msg::Reload => {
                js! {
                    top.frames.location.reload()
                }
            }
            Msg::Event(e) => {
                match e {
                    Event::SetlistEvent(se) => self.handle_setlist_event(se),
                    _ => debug!("New event {:?}", e)
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        let main_classes = if self.expand {
            "-menu-visible"
        } else {
            "-menu-hidden"
        };

        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);
        let on_setlist_change = self.link.callback(|e| Msg::Event(e));
        let songs = Rc::new(self.setlist.clone());

        html! {
            <main class=main_classes>
                <Nav
                    expand=self.expand
                    songs=songs
                    on_toggle=toggle_menu
                    on_setlist_change=on_setlist_change
                />
                <div class="content">
                    { self.route() }
                </div>
            </main>
        }
    }
}
