use crate::components::nav::Nav;
use crate::components::reload_section::ReloadSection;
use crate::components::setlist::SetlistLoad;
use crate::components::song_browser::SongBrowser;
use crate::components::song_search::SongSearch;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::errors::WebError;
use crate::events::{Event, SetlistEvent, SortingChange};
use crate::fetch;
use crate::handler_traits::setlist_handler::SetlistHandler;
use crate::handler_traits::settings_handler::SettingsHandler;
use crate::route::{AppRoute, SetlistRoute};
use js_sys::Date;
use libchordr::models::setlist::{Setlist, SetlistEntry};
use libchordr::models::song_id::SongIdTrait;
use libchordr::models::song_settings::SongSettings;
use libchordr::prelude::*;
use log::{debug, error, info, warn};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys;
use web_sys::window;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

const STORAGE_KEY_SETLIST: &'static str = "net.cundd.chordr.setlist";
const STORAGE_KEY_SETTINGS: &'static str = "net.cundd.chordr.settings";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppRouteState {}

impl Default for AppRouteState {
    fn default() -> Self {
        Self {}
    }
}

pub struct App {
    storage_service: StorageService,
    /// Keep a reference to the RouteService so that it doesn't get dropped
    _route_service: RouteService<AppRouteState>,
    route: Route<AppRouteState>,
    link: ComponentLink<App>,

    expand: bool,
    fetching: bool,
    catalog: Option<Catalog>,
    setlist: Setlist<SetlistEntry>,
    settings: SongSettingsMap,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Msg {
    Event(Event),
    FetchCatalogReady(Result<Catalog, WebError>),
    FetchCatalog(bool),
    SongSettingsChange(SongId, SongSettings),
    ToggleMenu,
    Reload,
    Ignore,
    RouteChanged(Route<AppRouteState>),
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
        self.compose(
            html! {
                <>
                    <StartScreen />
                    {self.render_song_browser("")}
                    <ReloadSection />
                    {self.render_song_search(false)}
                </>
            },
            self.view_nav(None),
        )
    }

    fn view_song(&self, song_id: SongId) -> Html {
        if self.catalog.is_none() {
            return html! {};
        }

        let catalog = self.catalog.as_ref().unwrap();
        if let Some(song) = catalog.get(song_id.clone()) {
            let add = self
                .link
                .callback(|s| Msg::Event(SetlistEvent::Add(s).into()));
            let remove = self
                .link
                .callback(|s| Msg::Event(SetlistEvent::Remove(s).into()));
            let change = self
                .link
                .callback(|s: (SongId, SongSettings)| Msg::SongSettingsChange(s.0, s.1));
            let is_on_setlist = self.setlist.contains(song);

            let song_settings = match self.settings.get(&song_id) {
                Some(s) => {
                    debug!("Found settings for song {}: {:?}", song_id, s);
                    s.clone()
                }
                None => {
                    debug!("No settings for song {} found in setlist", song_id);
                    SongSettings::new(0, Formatting::default())
                }
            };

            debug!("Song {} is on list? {}", song.id(), is_on_setlist);

            return self.compose(
                html! {
                    <SongView
                        song=song
                        song_settings=song_settings
                        enable_setlists=true
                        on_setlist_add=add
                        on_setlist_remove=remove
                        on_settings_change=change
                        is_on_setlist=is_on_setlist
                    />
                },
                self.view_nav(Some(song_id)),
            );
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
                html! {}
            }
        }) as Html
    }

    fn view_song_browser<S: Into<String>>(&self, chars: S) -> Html {
        self.compose(self.render_song_browser(chars), self.view_nav(None))
    }

    fn render_song_browser<S: Into<String>>(&self, chars: S) -> Html {
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
        self.compose(
            self.render_song_search(show_back_button),
            self.view_nav(None),
        )
    }

    fn render_song_search(&self, show_back_button: bool) -> Html {
        (match &self.catalog {
            Some(catalog) => {
                html! {<SongSearch catalog=catalog show_back_button=show_back_button />}
            }
            None => html! {},
        }) as Html
    }

    fn view_setlist_route(&self, route: SetlistRoute) -> Html {
        match route {
            SetlistRoute::Load { serialized_setlist } => match &self.catalog {
                None => html! {},
                Some(catalog) => {
                    let replace = self.link.callback(|e| Msg::Event(e));
                    let catalog = Rc::new(catalog.clone());
                    let setlist = Rc::new(self.setlist.clone());

                    self.compose(
                        html! {
                            <SetlistLoad
                                catalog=catalog
                                serialized_setlist=serialized_setlist
                                on_load=replace
                                current_setlist=setlist
                            />
                        },
                        self.view_nav(None),
                    )
                }
            },
        }
    }

    fn view_nav(&self, current_song_id: Option<SongId>) -> Html {
        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);
        let on_setlist_change = self.link.callback(|e| Msg::Event(e));
        let songs = Rc::new(self.setlist.clone());

        html! {
            <Nav
                expand=self.expand
                songs=songs
                current_song_id=current_song_id
                on_toggle=toggle_menu
                on_setlist_change=on_setlist_change
            />
        }
    }

    /// Wrap `content` and `navigation` blocks into the required HTML structure
    fn compose(&self, content: Html, nav: Html) -> Html {
        return html! {
            <>
                { nav }
                <div class="content">{ content }</div>
            </>
        };
    }

    fn fetch_catalog(&mut self, no_cache: bool) {
        let callback = self.link.callback(move |result| {
            info!("Catalog response {:?}", result);
            Msg::FetchCatalogReady(result)
        });

        let uri_base = "/catalog.json".to_owned();
        let uri = if no_cache {
            uri_base + &format!("?{}", Date::now())
        } else {
            uri_base
        };

        spawn_local(async move {
            let result = fetch::<Catalog>(&uri).await;
            callback.emit(result);
        });
    }
}

impl SetlistHandler for App {
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
        match self
            .setlist
            .move_entry(sorting_change.old_index(), sorting_change.new_index())
        {
            Ok(_) => {
                self.storage_service
                    .store(STORAGE_KEY_SETLIST, Json(&self.setlist));
            }
            Err(e) => error!("{}", e),
        }
    }

    fn get_setlist(storage_service: &StorageService) -> Setlist<SetlistEntry> {
        if let Json(Ok(restored_model)) = storage_service.restore(STORAGE_KEY_SETLIST) {
            restored_model
        } else {
            Setlist::new()
        }
    }
}

impl SettingsHandler for App {
    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) {
        self.settings.store(song_id, settings);
        self.storage_service
            .store(STORAGE_KEY_SETTINGS, Json(&self.settings));
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
        let mut route_service: RouteService<AppRouteState> = RouteService::new();
        let route = Route::from(route_service.get_route());
        route_service.register_callback(link.callback(Msg::RouteChanged));

        let storage_service = StorageService::new(Area::Local).unwrap();
        let setlist = App::get_setlist(&storage_service);
        let settings = App::get_settings(&storage_service);

        Self {
            storage_service,
            link,
            fetching: false,
            expand: true,
            catalog: None,
            settings,
            setlist,
            _route_service: route_service,
            route,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
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
                window()
                    .expect("Could not detect the JS window object")
                    .top()
                    .unwrap()
                    .unwrap()
                    .location()
                    .reload()
                    .expect("Could not reload the top-frame");
            }
            Msg::Event(e) => match e {
                Event::SetlistEvent(se) => self.handle_setlist_event(se),
                _ => debug!("New event {:?}", e),
            },
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let main_classes = if self.expand {
            "-menu-visible"
        } else {
            "-menu-hidden"
        };

        debug!("Redraw App");

        (html! {<main class=main_classes>{ self.route() }</main>}) as Html
    }

    fn rendered(&mut self, first_render: bool) -> () {
        if first_render {
            self.fetch_catalog(true);
        }
    }
}
