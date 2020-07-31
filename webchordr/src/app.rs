use crate::components::nav::Nav;
use crate::components::reload_section::ReloadSection;
use crate::components::setlist::SetlistLoad;
use crate::components::song_browser::SongBrowser;
use crate::components::song_search::SongSearch;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::components::user::Info as UserInfo;
use crate::errors::WebError;
use crate::events::{Event, SetlistEvent, SettingsEvent, SortingChange};
use crate::handler_traits::catalog_handler::CatalogHandler;
use crate::handler_traits::setlist_handler::SetlistHandler;
use crate::handler_traits::settings_handler::SettingsHandler;
use crate::helpers::window;
use crate::persistence::prelude::*;
use crate::persistence::web_repository::{CatalogWebRepository, SettingsWebRepository};
use crate::route::{AppRoute, SetlistRoute, UserRoute};
use chrono::Utc;
use libchordr::prelude::*;
use log::{debug, error, info, warn};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppRouteState {}

impl Default for AppRouteState {
    fn default() -> Self {
        Self {}
    }
}

pub struct App {
    persistence_manager: PersistenceManager<BrowserStorage>,
    /// Keep a reference to the RouteService so that it doesn't get dropped
    _route_service: RouteService<AppRouteState>,
    route: Route<AppRouteState>,
    link: ComponentLink<App>,
    user: User,
    expand: bool,
    fetching: bool,
    catalog: Option<Catalog>,
    current_setlist: Option<Setlist>,
    _setlist_collection: SetlistCollection,
    settings: SongSettingsMap,
}

//#[allow(dead_code)]

#[derive(Debug)]
pub enum Msg {
    Event(Event),
    FetchCatalogReady(Result<Catalog, WebError>),
    ToggleMenu,
    #[allow(dead_code)]
    Reload,
    #[allow(dead_code)]
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

            Some(AppRoute::User(route)) => self.view_user_route(route),
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
            let change = self.link.callback(|s: (SongId, SongSettings)| {
                Msg::Event(Event::Pair(
                    Box::new(SettingsEvent::Change(s.0.clone(), s.1.clone()).into()),
                    Box::new(SetlistEvent::SettingsChange(s.0, s.1).into()),
                ))
            });
            let is_on_setlist = if let Some(ref setlist) = self.current_setlist {
                setlist.contains_id(song_id.clone())
            } else {
                false
            };

            let song_settings = self.get_settings_for_song(&song_id);

            debug!("Song {} is on list? {}", song_id, is_on_setlist);

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

    fn get_settings_for_song(&self, song_id: &SongId) -> SongSettings {
        // Look if there are settings for the `SongId` in the `Setlist`
        if let Some(settings) = self.get_settings_from_setlist(song_id) {
            debug!(
                "Found settings for song in Setlist {}: {:?}",
                song_id, settings
            );

            return settings;
        }

        match self.settings.get(song_id) {
            Some(s) => {
                debug!("Found settings for song {}: {:?}", song_id, s);
                s.clone()
            }
            None => {
                debug!("No settings for song {} found in setlist", song_id);
                SongSettings::default()
            }
        }
    }

    fn get_settings_from_setlist(&self, song_id: &SongId) -> Option<SongSettings> {
        match &self.current_setlist {
            None => None,
            Some(setlist) => setlist
                .get(song_id.clone())
                .and_then(|entry| entry.settings()),
        }
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
                    let setlist = match self.current_setlist {
                        Some(ref s) => Some(Rc::new(s.clone())),
                        None => None,
                    };
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

    fn view_user_route(&self, route: UserRoute) -> Html {
        match route {
            UserRoute::Info => html! { <UserInfo user=self.user.clone() /> },
        }
    }

    fn view_nav(&self, current_song_id: Option<SongId>) -> Html {
        let toggle_menu = self.link.callback(|_| Msg::ToggleMenu);
        let on_setlist_change = self.link.callback(|e| Msg::Event(e));
        let songs = match self.current_setlist {
            Some(ref s) => Some(Rc::new(s.clone())),
            None => None,
        };

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

    fn handle_event(&mut self, e: Event) {
        match e {
            Event::SetlistEvent(se) => self.handle_setlist_event(se),
            Event::SettingsEvent(se) => self.handle_settings_event(se),
            Event::Pair(a, b) => {
                self.handle_event(*a);
                self.handle_event(*b)
            }
            _ => debug!("New event {:?}", e),
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
}

impl CatalogHandler for App {
    fn fetch_catalog(&mut self) {
        let mut pm = self.persistence_manager.clone();
        let callback = self.link.callback(move |result| {
            debug!("Catalog response {:?}", result);
            Msg::FetchCatalogReady(result)
        });

        spawn_local(async move {
            type Repository<'a> = CatalogWebRepository<'a, PersistenceManager<BrowserStorage>>;
            let result = Repository::new(&mut pm).load().await;

            match result {
                Ok(Some(catalog)) => callback.emit(Ok(catalog)),
                Ok(None) => { /* noop */ }
                Err(e) => callback.emit(Err(e)),
            }
        });
    }

    fn commit_changes(&mut self) {
        let mut pm = self.persistence_manager.clone();
        let catalog = self.catalog.clone();
        spawn_local(async move {
            type Repository<'a> = CatalogWebRepository<'a, PersistenceManager<BrowserStorage>>;
            let result = Repository::new(&mut pm).store(&catalog.unwrap()).await;

            if let Err(e) = result {
                error!("Could not commit Catalog changes: {}", e.to_string())
            }
        });
    }
}

impl SetlistHandler for App {
    fn handle_setlist_event(&mut self, event: SetlistEvent) {
        match event {
            SetlistEvent::SortingChange(v) => self.setlist_sorting_changed(v),
            SetlistEvent::Add(v) => self.setlist_add(v),
            SetlistEvent::Remove(v) => self.setlist_remove(v),
            SetlistEvent::SettingsChange(id, settings) => {
                self.setlist_settings_changed(id, settings)
            }
            SetlistEvent::Replace(v) => self.setlist_replace(v),
        }
    }

    fn setlist_add(&mut self, song: SetlistEntry) {
        let song_id = song.id();
        match self
            .current_setlist
            .as_mut()
            .expect("No current setlist defined")
            .add(song)
        {
            Ok(_) => debug!("Did add song to setlist {}", song_id),
            Err(e) => error!("Could not add song to setlist: {:?}", e),
        }
        <App as SetlistHandler>::commit_changes(self);
    }

    fn setlist_remove(&mut self, song_id: SongId) {
        match self
            .current_setlist
            .as_mut()
            .expect("No current setlist defined")
            .remove_by_id(song_id.clone())
        {
            Ok(_) => info!("Removed song {} from setlist", song_id),
            Err(_) => warn!("Could not remove song {} from setlist", song_id),
        }
        <App as SetlistHandler>::commit_changes(self);
    }

    fn setlist_settings_changed(&mut self, song_id: SongId, song_settings: SongSettings) {
        info!(
            "Settings changed song {} {:#?}",
            song_id,
            self.current_setlist.as_ref()
        );
        let setlist = self
            .current_setlist
            .as_mut()
            .expect("No current setlist defined");
        let current_entry = match setlist.get(song_id.clone()) {
            None => {
                warn!("Could not find song {} in setlist", song_id);
                return;
            }
            Some(c) => c,
        };
        let new_entry = current_entry.with_settings(song_settings);

        if setlist.replace(new_entry).is_err() {
            warn!("Could not replace song {} in setlist", song_id);
            return;
        }

        <App as SetlistHandler>::commit_changes(self);
    }

    fn setlist_replace(&mut self, setlist: Setlist) {
        info!(
            "Replace setlist {:?} with {:?}",
            self.current_setlist, setlist
        );
        self.current_setlist = Some(setlist);
        <App as SetlistHandler>::commit_changes(self);
    }

    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange) {
        let move_result = self
            .current_setlist
            .as_mut()
            .unwrap()
            .move_entry(sorting_change.old_index(), sorting_change.new_index());

        match move_result {
            Ok(_) => <App as SetlistHandler>::commit_changes(self),
            Err(e) => error!("{}", e),
        }
    }

    fn fetch_setlist(&mut self) {
        let mut pm = self.persistence_manager.clone();
        let callback = self
            .link
            .callback(move |setlist| Msg::Event(SetlistEvent::Replace(setlist).into()));

        spawn_local(async move {
            type Repository<'a> = SetlistWebRepository<'a, PersistenceManager<BrowserStorage>>;
            let result = Repository::new(&mut pm).load().await;

            match result {
                Ok(Some(setlist)) => callback.emit(setlist),
                Ok(None) => { /* noop */ }
                Err(e) => error!("{}", e),
            }
        });
    }

    fn commit_changes(&mut self) {
        let mut pm = self.persistence_manager.clone();
        match self.current_setlist.clone() {
            Some(s) => spawn_local(async move {
                type Repository<'a> = SetlistWebRepository<'a, PersistenceManager<BrowserStorage>>;
                let result = Repository::new(&mut pm).store(&s).await;

                if let Err(e) = result {
                    error!("Could not commit setlist changes: {}", e.to_string())
                }
            }),
            None => info!("Currently there is no setlist to commit"),
        }
    }
}

impl SettingsHandler for App {
    fn handle_settings_event(&mut self, event: SettingsEvent) {
        match event {
            SettingsEvent::Change(song_id, settings) => {
                self.song_settings_change(song_id, settings)
            }
            SettingsEvent::Replace(v) => self.song_settings_replace(v),
        }
    }

    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) {
        self.settings.store(song_id, settings);
        <App as SettingsHandler>::commit_changes(self);
    }

    fn song_settings_replace(&mut self, settings: SongSettingsMap) {
        info!(
            "Replace Map of Song Settings {:?} with {:?}",
            self.settings, settings
        );
        self.settings = settings;
        <App as SettingsHandler>::commit_changes(self);
    }

    fn fetch_song_settings(&mut self) {
        let mut pm = self.persistence_manager.clone();
        let callback = self
            .link
            .callback(move |settings_map| Msg::Event(SettingsEvent::Replace(settings_map).into()));

        spawn_local(async move {
            type Repository<'a> = SettingsWebRepository<'a, PersistenceManager<BrowserStorage>>;
            let result = Repository::new(&mut pm).load().await;

            match result {
                Ok(Some(settings)) => callback.emit(settings),
                Ok(None) => { /* noop */ }
                Err(e) => error!("{}", e),
            }
        });
    }

    fn commit_changes(&mut self) {
        let mut pm = self.persistence_manager.clone();
        let settings = self.settings.clone();
        spawn_local(async move {
            type Repository<'a> = SettingsWebRepository<'a, PersistenceManager<BrowserStorage>>;
            let result = Repository::new(&mut pm).store(&settings).await;

            if let Err(e) = result {
                error!("Could not commit setting changes: {}", e.to_string())
            }
        });
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<AppRouteState> = RouteService::new();
        let route = Route::from(route_service.get_route());
        route_service.register_callback(link.callback(Msg::RouteChanged));

        let persistence_manager = PersistenceManager::new(BrowserStorage::new().unwrap());

        let user = User::unknown();
        let now = Utc::now();
        let setlist = Setlist::new("", 0, user.clone(), None, Some(now), now, now, vec![]);
        let settings = SongSettingsMap::new();

        Self {
            persistence_manager,
            link,
            fetching: false,
            expand: true,
            catalog: None,
            settings,
            current_setlist: Some(setlist),
            _setlist_collection: SetlistCollection::new(),
            _route_service: route_service,
            route,
            user: user,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::FetchCatalogReady(response) => {
                self.fetching = false;
                self.catalog = response.ok();
            }
            Msg::Ignore => return false,
            Msg::ToggleMenu => {
                self.expand = !self.expand;
            }
            Msg::Reload => {
                window()
                    .top()
                    .unwrap()
                    .unwrap()
                    .location()
                    .reload()
                    .expect("Could not reload the top-frame");
            }
            Msg::Event(e) => self.handle_event(e),
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
            self.fetch_catalog();
            self.fetch_setlist();
            self.fetch_song_settings();
        }
    }
}
