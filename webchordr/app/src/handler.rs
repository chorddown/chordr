use cqrs::prelude::AsyncRepositoryTrait;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use libchordr::models::user::MainData;
use libchordr::prelude::*;
use log::{debug, error, info, trace, warn};
use std::rc::Rc;
use std::sync::Arc;
use tri::Tri;
use wasm_bindgen_futures::spawn_local;
use webchordr_common::route::AppRoute;
use webchordr_events::{Event, SetlistEvent, SettingsEvent, SortingChange};
use webchordr_persistence::persistence_manager::PMType;
use webchordr_persistence::persistence_manager::PersistenceManagerFactory;
use webchordr_persistence::prelude::*;
use webchordr_persistence::session::SessionService;
use webchordr_persistence::web_repository::{CatalogWebRepository, SettingsWebRepository};
use yew::prelude::*;

use crate::app::App;
use crate::config::Config;
use crate::connection::{ConnectionService, ConnectionStatus};
use crate::control::navigate::SongNavigator;
use crate::control::{Control, KeyboardControl};
use crate::errors::WebError;
use crate::handler_traits::catalog_handler::CatalogHandler;
use crate::handler_traits::setlist_handler::SetlistHandler;
use crate::handler_traits::settings_handler::SettingsHandler;
use crate::helpers::window;
use crate::ipc::update_info::UpdateInfo;
use crate::ipc::{register_ipc_handler, IpcMessage};
use crate::session::{Session, SessionMainData};
use crate::state::State;

type InitialDataResult = Result<Box<SessionMainData>, Option<WebError>>;

const TICK_INTERVAL: u32 = 300;

#[allow(unused)]
pub struct Handler {
    persistence_manager: Arc<PMType>,
    /// Keep a reference to the IntervalTask so that it doesn't get dropped
    _clock_handle: gloo_timers::callback::Interval,
    message_listener: Option<EventListener>,
    keyboard_control: KeyboardControl,
    #[allow(unused)]
    fetching: bool,
    config: Config,
    session_service: Rc<SessionService>,
    connection_service: ConnectionService,
    state: Rc<State>,
}

#[derive(Debug)]
pub enum Msg {
    Tick,
    Event(Box<Event>),
    FetchCatalogReady(Result<Catalog, WebError>),
    #[allow(dead_code)]
    Reload,
    Ignore,
    SessionChanged(Session),
    #[cfg(feature = "server_sync")]
    ConnectionStatusChanged(ConnectionStatus),
    StateChanged(State),
    InitialDataLoaded(InitialDataResult),
    UpdateInfo(UpdateInfo),
    Control(Control),
}

impl Handler {
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

    fn build_persistence_manager(config: &Config, session: Session) -> Arc<PMType> {
        let persistence_manager_factory = PersistenceManagerFactory::new();

        Arc::new(persistence_manager_factory.build(config, session))
    }

    fn load_initial_data(&mut self, ctx: &Context<Self>) {
        let session_service = self.session_service.clone();
        match session_service.get_credentials_from_session_storage() {
            Ok(credentials) => {
                let on_load = ctx.link().callback(Msg::InitialDataLoaded);
                debug!("Try to login with credentials from Session Storage");
                spawn_local(async move {
                    match session_service.get_main_data(&credentials).await {
                        Ok(r) => {
                            debug!("Successful login with credentials from Session Storage");
                            on_load.emit(Ok(Box::new(r)))
                        }
                        Err(e) => {
                            debug!("Failed login with credentials from Session Storage");
                            on_load.emit(Err(Some(e)))
                        }
                    }
                });
            }
            Err(e) => ctx
                .link()
                .send_message(Msg::InitialDataLoaded(Err(Some(e)))),
        }
    }

    fn handle_initial_data(&mut self, ctx: &Context<Self>, r: InitialDataResult) {
        match r {
            Ok(initial_data) => {
                self.update_session(ctx, initial_data.session.clone(), false);
                let MainData {
                    song_settings,
                    latest_setlist,
                    user: _,
                } = initial_data.main_data;

                let did_fetch_setlist = latest_setlist.is_some();
                let did_fetch_song_settings = song_settings.is_some();

                self.set_state(
                    None,
                    State::new(
                        self.state.catalog().map(|c| (*c).clone()),
                        latest_setlist,
                        None,
                        song_settings.unwrap_or_else(SongSettingsMap::new),
                        ConnectionStatus::OnLine,
                        initial_data.session,
                        None,
                        None,
                    ),
                    true,
                );
                if !did_fetch_setlist {
                    self.fetch_setlist(ctx);
                }
                if !did_fetch_song_settings {
                    self.fetch_song_settings(ctx);
                }
            }
            Err(_) => {
                self.update_session(ctx, Session::default(), false);
                self.fetch_setlist(ctx);
                self.fetch_song_settings(ctx);
                #[cfg(feature = "server_sync")]
                self.check_connection_status(ctx);
            }
        }
    }

    #[cfg(feature = "server_sync")]
    fn check_connection_status(&mut self, ctx: &Context<Self>) {
        let connection_service = self.connection_service.clone();
        let state_changed = ctx.link().callback(Msg::ConnectionStatusChanged);
        spawn_local(async move {
            let connection_status = connection_service.get_connection_status().await;
            state_changed.emit(connection_status)
        });
    }

    #[allow(unused_variables)]
    fn run_scheduled_tasks(&mut self, ctx: &Context<Self>) {
        debug!("Run scheduled tasks");

        #[cfg(feature = "server_sync")]
        self.check_connection_status(ctx);
    }

    fn update_session(&mut self, ctx: &Context<Self>, session: Session, reload_data: bool) -> bool {
        let session_changed = &*self.state.session() != &session;
        if session_changed {
            self.set_state(None, self.state.with_session(session), true);
            self.persistence_manager =
                Self::build_persistence_manager(&self.config, (*self.state.session()).clone());
        }

        if reload_data {
            // Fetch/reload the Setlist and Song Settings
            self.fetch_setlist(ctx);
            self.fetch_song_settings(ctx);

            return true;
        }

        session_changed
    }

    fn set_state(&mut self, ctx: Option<&Context<Self>>, state: State, sync: bool) {
        debug!("Change state ({})", if sync { "sync" } else { "async" });
        trace!(
            "State diff: {}",
            crate::state::debug::diff(&self.state, &state)
        );

        if sync {
            self.state = Rc::new(state)
        } else {
            ctx.expect("Expected ctx to be a context")
                .link()
                .send_message(Msg::StateChanged(state))
        }
    }

    fn update_state_with_route(state: &State, ctx: &Context<Self>) -> State {
        if let AppRoute::Song { id } = &ctx.props().route {
            state.with_current_song_id(id.as_song_id())
        } else {
            state.without_current_song_id()
        }
    }
}

impl CatalogHandler for Handler {
    fn fetch_catalog(&mut self, ctx: &Context<Self>) {
        let pm = self.persistence_manager.clone();
        let callback = ctx.link().callback(Msg::FetchCatalogReady);

        spawn_local(async move {
            type Repository<'a> = CatalogWebRepository<'a, PMType>;
            let browser_storage = match BrowserStorage::local_storage() {
                Ok(s) => s,
                Err(e) => {
                    callback.emit(Err(e));

                    return;
                }
            };
            let result = Repository::new(&pm, browser_storage).load().await;

            match result {
                Tri::Some(catalog) => callback.emit(Ok(catalog)),
                Tri::None => { /* noop */ }
                Tri::Err(e) => callback.emit(Err(e)),
            }
        });
    }

    fn commit_changes(&mut self) {
        let pm = self.persistence_manager.clone();
        let catalog = self.state.catalog();
        spawn_local(async move {
            type Repository<'a> = CatalogWebRepository<'a, PMType>;
            let browser_storage = match BrowserStorage::local_storage() {
                Ok(s) => s,
                Err(e) => {
                    error!("Could not commit Catalog changes: {}", e.to_string());
                    return;
                }
            };
            let result = Repository::new(&pm, browser_storage)
                .store(&catalog.unwrap())
                .await;

            if let Err(e) = result {
                error!("Could not commit Catalog changes: {}", e.to_string())
            }
        });
    }
}

impl SetlistHandler for Handler {
    fn handle_setlist_event(&mut self, event: SetlistEvent) {
        match event {
            SetlistEvent::SortingChange(v) => self.setlist_sorting_changed(v),
            SetlistEvent::AddEntry(v) => self.setlist_entry_add(v),
            SetlistEvent::RemoveEntry(v) => self.setlist_entry_remove(v),
            SetlistEvent::SettingsChange(id, settings) => {
                self.setlist_settings_changed(id, settings)
            }
            SetlistEvent::Replace(v) => self.setlist_replace(v),
            SetlistEvent::SetCurrentSetlist(v) => self.set_current_setlist(v),
        }
    }

    fn add(&mut self, _setlist: Setlist) {
        todo!()
    }

    fn delete(&mut self, _setlist: Setlist) {
        todo!()
    }

    fn update(&mut self, _setlist: Setlist) {
        todo!()
    }

    fn fetch_setlists(&mut self) {
        todo!()
    }

    fn setlist_entry_add(&mut self, song: SetlistEntry) {
        let song_id = song.id();
        let current_setlist = self
            .state
            .current_setlist()
            .expect("No current setlist defined");
        let mut new_setlist = (*current_setlist).clone();
        match new_setlist.add(song) {
            Ok(_) => debug!("Did add song to setlist {}", song_id),
            Err(e) => error!("Could not add song to setlist: {:?}", e),
        }
        self.set_state(None, self.state.with_current_setlist(new_setlist), true);
        <Self as SetlistHandler>::commit_changes(self);
    }

    fn setlist_entry_remove(&mut self, song_id: SongId) {
        let current_setlist = self
            .state
            .current_setlist()
            .expect("No current setlist defined");
        let mut new_setlist = (*current_setlist).clone();
        match new_setlist.remove_by_id(song_id.clone()) {
            Ok(_) => info!("Removed song {} from setlist", song_id),
            Err(_) => warn!("Could not remove song {} from setlist", song_id),
        }
        self.set_state(None, self.state.with_current_setlist(new_setlist), true);
        <Self as SetlistHandler>::commit_changes(self);
    }

    fn setlist_settings_changed(&mut self, song_id: SongId, song_settings: SongSettings) {
        debug!(
            "Settings changed song {} {:#?}",
            song_id,
            self.state.current_setlist()
        );
        let setlist = self
            .state
            .current_setlist()
            .expect("No current setlist defined");
        let current_entry = match setlist.get(song_id.clone()) {
            None => {
                warn!("Could not find song {} in setlist", song_id);
                return;
            }
            Some(c) => c,
        };
        let new_entry = current_entry.with_settings(song_settings);

        let mut new_setlist = (*setlist).clone();
        if new_setlist.replace(new_entry).is_err() {
            warn!("Could not replace song {} in setlist", song_id);
            return;
        }

        self.set_state(None, self.state.with_current_setlist(new_setlist), true);
        <Self as SetlistHandler>::commit_changes(self);
    }

    fn setlist_replace(&mut self, setlist: Setlist) {
        info!("Replace setlist");
        debug!("{:?}\n=>\n{:?}", self.state.current_setlist(), setlist);
        self.set_state(None, self.state.with_current_setlist(setlist), true);
        <Self as SetlistHandler>::commit_changes(self);
    }

    fn set_current_setlist(&mut self, setlist: Setlist) {
        info!("Set current setlist");
        debug!("{:?}\n=>\n{:?}", self.state.current_setlist(), setlist);
        self.set_state(None, self.state.with_current_setlist(setlist), true);
        <Self as SetlistHandler>::commit_changes(self);
    }

    fn setlist_sorting_changed(&mut self, sorting_change: SortingChange) {
        let setlist = self.state.current_setlist().unwrap();
        let mut new_setlist = (*setlist).clone();
        let move_result =
            new_setlist.move_entry(sorting_change.old_index(), sorting_change.new_index());

        match move_result {
            Ok(_) => {
                self.set_state(None, self.state.with_current_setlist(new_setlist), true);
                <Self as SetlistHandler>::commit_changes(self)
            }
            Err(e) => error!("{}", e),
        }
    }

    fn fetch_setlist(&mut self, ctx: &Context<Self>) {
        let pm = self.persistence_manager.clone();
        let callback = ctx
            .link()
            .callback(move |setlist| Msg::Event(Box::new(SetlistEvent::Replace(setlist).into())));

        spawn_local(async move {
            type Repository<'a> = SetlistWebRepository<'a, PMType>;
            let result = Repository::new(&pm).load().await;

            match result {
                Tri::Some(setlist) => callback.emit(setlist),
                Tri::None => { /* noop */ }
                Tri::Err(e) => error!("{}", e),
            }
        });
    }

    fn commit_changes(&mut self) {
        let pm = self.persistence_manager.clone();
        match self.state.current_setlist() {
            Some(s) => spawn_local(async move {
                type Repository<'a> = SetlistWebRepository<'a, PMType>;
                let mut repository = Repository::new(&pm);

                let result = repository.store(&s).await;
                if let Err(e) = result {
                    error!("Could not commit setlist changes: {}", e.to_string())
                }

                let result = if repository.find_by_id(s.id()).await.is_none() {
                    repository.add((*s).clone()).await
                } else {
                    repository.update((*s).clone()).await
                };

                if let Err(e) = result {
                    error!("Could not commit setlist changes (v2): {}", e.to_string())
                }
            }),
            None => info!("Currently there is no setlist to commit"),
        }
    }
}

impl SettingsHandler for Handler {
    fn handle_settings_event(&mut self, event: SettingsEvent) {
        match event {
            SettingsEvent::Change(song_id, settings) => {
                self.song_settings_change(song_id, settings)
            }
            SettingsEvent::Replace(v) => self.song_settings_replace(v),
        }
    }

    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) {
        let mut song_settings = (*self.state.song_settings()).clone();
        song_settings.store(song_id, settings);
        self.set_state(None, self.state.with_song_settings(song_settings), true);
        <Handler as SettingsHandler>::commit_changes(self);
    }

    fn song_settings_replace(&mut self, settings: SongSettingsMap) {
        info!(
            "Replace Map of Song Settings {:?} with {:?}",
            self.state.song_settings(),
            settings
        );
        self.set_state(None, self.state.with_song_settings(settings), true);
        <Handler as SettingsHandler>::commit_changes(self);
    }

    fn fetch_song_settings(&mut self, ctx: &Context<Self>) {
        let pm = self.persistence_manager.clone();
        let callback = ctx.link().callback(move |settings_map| {
            Msg::Event(Box::new(SettingsEvent::Replace(settings_map).into()))
        });

        spawn_local(async move {
            type Repository<'a> = SettingsWebRepository<'a, PMType>;
            let result = Repository::new(&pm).load().await;

            match result {
                Tri::Some(settings) => callback.emit(settings),
                Tri::None => { /* noop */ }
                Tri::Err(e) => error!("{}", e),
            }
        });
    }

    fn commit_changes(&mut self) {
        let pm = self.persistence_manager.clone();
        let settings = self.state.song_settings();
        spawn_local(async move {
            type Repository<'a> = SettingsWebRepository<'a, PMType>;
            let result = Repository::new(&pm).store(&settings).await;

            if let Err(e) = result {
                error!("Could not commit setting changes: {}", e.to_string())
            }
        });
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HandlerProps {
    pub route: AppRoute,
}

impl Component for Handler {
    type Message = Msg;
    type Properties = HandlerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let on_tick = ctx.link().callback(|_| Msg::Tick);
        let clock_handle = Interval::new(TICK_INTERVAL * 1000, move || on_tick.emit(()));

        let config = Config::default();
        let session_service = Rc::new(SessionService::new(config.clone()));
        let persistence_manager = Handler::build_persistence_manager(&config, Session::default());

        let state = Rc::new(Self::update_state_with_route(&State::default(), ctx));
        let connection_service = ConnectionService::new(config.clone());

        let message_listener = register_ipc_handler(ctx.link().callback(|m| match m {
            IpcMessage::UpdateInfo(i) => Msg::UpdateInfo(i),
        }));

        let keyboard_control =
            KeyboardControl::new(ctx.link().callback(|control| Msg::Control(control)));

        Self {
            persistence_manager,
            fetching: false,
            _clock_handle: clock_handle,
            message_listener,
            config,
            session_service,
            connection_service,
            state,
            keyboard_control,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        trace!("Received Message {:?}", msg);

        match msg {
            Msg::FetchCatalogReady(response) => {
                self.fetching = false;

                match response {
                    Ok(catalog) => {
                        debug!("Catalog fetched with revision: {:?}", catalog.revision());
                        self.set_state(None, self.state.with_catalog(Some(catalog)), true);
                    }
                    Err(error) => {
                        debug!("Catalog fetched with error {}", error);
                        self.set_state(None, self.state.with_error(Some(error)), true);
                    }
                }
            }
            Msg::Ignore => return false,
            Msg::SessionChanged(session) => return self.update_session(ctx, session, true),
            #[cfg(feature = "server_sync")]
            Msg::ConnectionStatusChanged(connection_state) => {
                if self.state.connection_status() != connection_state {
                    self.set_state(
                        None,
                        self.state.with_connection_status(connection_state),
                        true,
                    )
                } else {
                    return false;
                }
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
            Msg::Event(e) => self.handle_event(*e),
            Msg::StateChanged(_state) => unreachable!(), //self.state = Rc::new(state),
            Msg::InitialDataLoaded(r) => self.handle_initial_data(ctx, r),
            Msg::Tick => {
                self.run_scheduled_tasks(ctx);
                return false;
            }
            Msg::UpdateInfo(v) => {
                self.set_state(None, self.state.with_available_version(v.version), true);
            }
            Msg::Control(control) => {
                match control {
                    Control::Navigate(navigate) => {
                        if SongNavigator::new()
                            .navigate(navigate, &self.state)
                            .is_none()
                        {
                            return false;
                        }
                    }
                };
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.set_state(None, Self::update_state_with_route(&self.state, ctx), true);

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        debug!("Redraw the handler");
        let state = self.state.clone();
        let persistence_manager = self.persistence_manager.clone();
        let link = ctx.link();
        let on_event = link.callback(|e| Msg::Event(Box::new(e)));
        let on_setlist_change = link.callback(|e| Msg::Event(Box::new(e)));
        let on_user_login_success = link.callback(Msg::SessionChanged);
        let on_user_login_error = link.callback(|e| {
            error!("{}", e);
            Msg::Ignore
        });

        if let Some(e) = state.error() {
            let window: web_sys::Window = web_sys::window().expect("window not available");
            window
                .alert_with_message(&format!("Could not load the song catalog (error: {})", e))
                .expect("alert failed");
        }

        if state.catalog().is_some() {
            (html! {
                 <App {state}
                    {on_event}
                    {on_setlist_change}
                    {on_user_login_success}
                    {on_user_login_error}
                    {persistence_manager}
                />
            }) as Html
        } else {
            (html! {
                <div class="center-fullscreen loading">
                    <div class="loading-inner">
                        <i class="im im-spinner"></i>
                    </div>
                </div>
            }) as Html
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.fetch_catalog(ctx);
            self.load_initial_data(ctx);
        }
    }
}
