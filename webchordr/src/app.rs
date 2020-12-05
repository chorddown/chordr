use crate::components::nav::Nav;
use crate::components::reload_section::ReloadSection;
use crate::components::setlist::SetlistLoad;
use crate::components::song_browser::SongBrowser;
use crate::components::song_search::SongSearch;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::components::user::Info as UserInfo;
use crate::components::user::Login as UserLogin;
use crate::config::Config;
use crate::events::{Event, SetlistEvent, SettingsEvent};
use crate::route::{AppRoute, SetlistRoute, UserRoute};
use crate::session::Session;
use crate::state::State;
use crate::WebError;
use libchordr::prelude::*;
use log::{debug, error, info};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppRouteState {}

impl Default for AppRouteState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct AppProperties {
    pub on_event: Callback<Event>,
    pub on_setlist_change: Callback<Event>,
    pub on_user_login_success: Callback<Session>,
    pub on_user_login_error: Callback<WebError>,
    pub state: Rc<State>,
}

pub struct App {
    /// Keep a reference to the RouteService so that it doesn't get dropped
    _route_service: RouteService<AppRouteState>,
    route: Route<AppRouteState>,
    expand: bool,
    config: Config,
    props: AppProperties,
    link: ComponentLink<App>,
}

#[derive(Debug)]
pub enum Msg {
    ToggleMenu,
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
        let state = self.props.state.clone();
        if state.catalog().is_none() {
            return html! {};
        }

        let catalog = state.catalog().unwrap();
        if let Some(song) = catalog.get(song_id.clone()) {
            let add = self.props.on_event.reform(|s| SetlistEvent::Add(s).into());
            let remove = self
                .props
                .on_event
                .reform(|s| SetlistEvent::Remove(s).into());
            let change = self.props.on_event.reform(|s: (SongId, SongSettings)| {
                Event::Pair(
                    Box::new(SettingsEvent::Change(s.0.clone(), s.1.clone()).into()),
                    Box::new(SetlistEvent::SettingsChange(s.0, s.1).into()),
                )
            });
            let is_on_setlist = if let Some(ref setlist) = state.current_setlist() {
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

        match self.props.state.song_settings().get(song_id) {
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
        match &self.props.state.current_setlist() {
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

        (match &self.props.state.catalog() {
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
        (match &self.props.state.catalog() {
            Some(catalog) => {
                html! {<SongSearch catalog=catalog show_back_button=show_back_button />}
            }
            None => html! {},
        }) as Html
    }

    fn view_setlist_route(&self, route: SetlistRoute) -> Html {
        let state = self.props.state.clone();
        match route {
            SetlistRoute::Load { serialized_setlist } => match &state.catalog() {
                None => html! {},
                Some(catalog) => {
                    let replace = self.props.on_event.reform(|e| e);
                    let catalog = catalog.clone();
                    let setlist = state.current_setlist();

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
        let user = self.props.state.session().user().clone();
        let on_login_success = self.props.on_user_login_success.reform(|i| i);
        let on_login_error = self.props.on_user_login_error.reform(|i| i);

        match route {
            UserRoute::Info => html! { <UserInfo user=user /> },
            UserRoute::Login => html! {
                <UserLogin
                    user=user
                    config=self.config.clone()
                    on_success=on_login_success
                    on_error=on_login_error
                />
            },
        }
    }

    fn view_nav(&self, current_song_id: Option<SongId>) -> Html {
        let on_toggle = self.link.callback(|_| Msg::ToggleMenu);
        let on_setlist_change = self.props.on_setlist_change.reform(|i| i);
        let state = self.props.state.clone();

        html! {
            <Nav
                expand=self.expand
                current_song_id=current_song_id
                on_toggle=on_toggle
                on_setlist_change=on_setlist_change
                state=state
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
}

impl Component for App {
    type Message = Msg;
    type Properties = AppProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<AppRouteState> = RouteService::new();
        let route = Route::from(route_service.get_route());
        route_service.register_callback(link.callback(Msg::RouteChanged));

        let config = Config::default();

        Self {
            _route_service: route_service,
            expand: true,
            link,
            route,
            config,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ToggleMenu => self.expand = !self.expand,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
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
            // self.fetch_catalog();
            // self.try_login_and_update_session();
        }
    }
}
