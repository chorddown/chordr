use std::rc::Rc;
use std::sync::Arc;

use log::{debug, error, info};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use yew::{html, Callback, Component, Context, Html, Properties};
use yew_router::prelude::*;

use libchordr::prelude::*;
use webchordr_common::config::Config;
use webchordr_common::errors::WebError;
use webchordr_events::{Event, SetlistEvent, SettingsEvent};
use webchordr_persistence::persistence_manager::PMType;
use webchordr_song_browser::SongBrowser;

use crate::components::nav::Nav;
use crate::components::reload_section::ReloadSection;
use crate::components::setlist::List as SetlistList;
use crate::components::setlist::SetlistLoad;
use crate::components::song_search::SongSearch;
use crate::components::song_view::SongView;
use crate::components::start_screen::StartScreen;
use crate::components::user::Info as UserInfo;
use crate::components::user::Login as UserLogin;
use crate::service::song_info_service::SongInfoService;
use crate::session::Session;
use crate::state::{SongInfo, State};
use webchordr_common::route::{AppRoute, SetlistRoute, UserRoute};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppRouteState {}

impl Default for AppRouteState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Properties, Clone)]
pub struct AppProperties {
    pub on_event: Callback<Event>,
    pub on_setlist_change: Callback<Event>,
    pub on_user_login_success: Callback<Session>,
    pub on_user_login_error: Callback<WebError>,
    pub state: Rc<State>,
    pub persistence_manager: Arc<PMType>,
}

impl PartialEq for AppProperties {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
            && self.on_event == other.on_event
            && self.on_setlist_change == other.on_setlist_change
            && self.on_user_login_error == other.on_user_login_error
            && self.on_user_login_success == other.on_user_login_success
    }
}

pub struct App {
    /// Keep a reference to the RouteService so that it doesn't get dropped
    // _route_service: RouteService<AppRouteState>,
    // route: Route<AppRouteState>,
    expand: bool,
    config: Config,
}

#[derive(Debug)]
pub enum Msg {
    ToggleMenu,
    RouteChanged, /*(Route<AppRouteState>)*/
}

impl App {
    fn route(&self, ctx: &Context<Self>, app_route: Option<AppRoute>) -> Html {
        // let app_route = Some(AppRoute::Index); //::switch(self.route.clone());
        // let app_route = Some(app_route);
        debug!("Render Route: {:?}", app_route);

        (match app_route {
            Some(AppRoute::Song { id }) => self.view_song(ctx, id.clone()),
            Some(AppRoute::SongBrowser { chars }) => self.view_song_browser(ctx, chars),
            Some(AppRoute::SongSearch) => self.view_song_search(ctx, true),
            Some(AppRoute::SetlistList) => self.view_setlist_route(ctx, SetlistRoute::List),
            Some(AppRoute::SetlistLoad { serialized_setlist }) => {
                self.view_setlist_route(ctx, SetlistRoute::Load { serialized_setlist })
            }
            Some(AppRoute::Setlist) => {
                todo!("route");
                // self.view_setlist_route(ctx, route)
            }

            Some(AppRoute::UserInfo) => self.view_user_route(ctx, UserRoute::Info),
            Some(AppRoute::UserLogin) => self.view_user_route(ctx, UserRoute::Login),
            Some(AppRoute::User) => {
                todo!("route");
                // self.view_user_route(ctx, route)
            }

            Some(AppRoute::Index) => self.view_index(ctx),
            None => self.view_index(ctx),
        }) as Html
    }

    fn view_index(&self, ctx: &Context<Self>) -> Html {
        self.compose(
            html! {
                <>
                    <StartScreen />
                    {self.render_song_browser(ctx,"")}
                    <ReloadSection catalog={ctx.props().state.catalog()} />
                    {self.render_song_search(ctx,false)}
                </>
            },
            self.view_nav(ctx, None),
        )
    }

    fn view_song(&self, ctx: &Context<Self>, song_id: SongId) -> Html {
        if ctx.props().state.catalog().is_none() {
            return html! {};
        }

        if let Some(song_info) = self.get_song_info(ctx, &song_id) {
            let add = ctx
                .props()
                .on_event
                .reform(|s| SetlistEvent::AddEntry(s).into());
            let remove = ctx
                .props()
                .on_event
                .reform(|s| SetlistEvent::RemoveEntry(s).into());
            let change = ctx
                .props()
                .on_event
                .reform(App::reform_settings_change_to_event);
            assert_eq!(song_id, song_info.song.id());
            debug!("Song {} is on list? {}", song_id, song_info.is_on_setlist);

            return self.compose(
                html! {
                    <SongView
                        song_info={song_info}
                        enable_setlists={true}
                        on_setlist_add={add}
                        on_setlist_remove={remove}
                        on_settings_change={change}
                    />
                },
                self.view_nav(ctx, Some(song_id)),
            );
        }

        (match percent_decode_str(song_id.as_str()).decode_utf8() {
            Ok(decoded) => {
                let decoded = decoded.to_string();
                info!("Decoded song ID '{}' to '{}'", song_id, decoded);
                if decoded != song_id.as_str() {
                    self.view_song(ctx, SongId::new(decoded))
                } else {
                    self.compose(
                        html! {<h1>{format!("Could not find song {}", song_id)}</h1>},
                        self.view_nav(ctx, Some(song_id)),
                    )
                }
            }
            Err(e) => {
                error!("Could not decode the song ID {}", e);
                html! {}
            }
        }) as Html
    }

    fn get_song_info(&self, ctx: &Context<Self>, song_id: &SongId) -> Option<SongInfo> {
        SongInfoService::new().get_song_info_from_state(song_id, &ctx.props().state)
    }

    fn view_song_browser<S: Into<String>>(&self, ctx: &Context<Self>, chars: S) -> Html {
        self.compose(
            self.render_song_browser(ctx, chars),
            self.view_nav(ctx, None),
        )
    }

    fn render_song_browser<S: Into<String>>(&self, ctx: &Context<Self>, chars: S) -> Html {
        let chars_as_string = chars.into();
        let chars = match percent_decode_str(&chars_as_string).decode_utf8() {
            Ok(decoded) => decoded.to_string(),
            Err(_) => chars_as_string,
        };

        (match &ctx.props().state.catalog() {
            Some(catalog) => {
                info!("New chars from router: {}", chars);
                html! {<SongBrowser chars={chars} catalog={catalog}/>}
            }
            None => html! {},
        }) as Html
    }

    fn view_song_search(&self, ctx: &Context<Self>, show_back_button: bool) -> Html {
        self.compose(
            self.render_song_search(ctx, show_back_button),
            self.view_nav(ctx, None),
        )
    }

    fn render_song_search(&self, ctx: &Context<Self>, show_back_button: bool) -> Html {
        (match &ctx.props().state.catalog() {
            Some(catalog) => {
                html! {<SongSearch catalog={catalog} show_back_button={show_back_button} />}
            }
            None => html! {},
        }) as Html
    }

    fn view_setlist_route(&self, ctx: &Context<Self>, route: SetlistRoute) -> Html {
        let state = &ctx.props().state;
        match route {
            SetlistRoute::Load { serialized_setlist } => match state.catalog() {
                None => html! {},
                Some(catalog) => {
                    let persistence_manager = ctx.props().persistence_manager.clone();
                    let replace = ctx.props().on_event.reform(|e| e);
                    let catalog = catalog.clone();
                    let setlist = state.current_setlist();

                    self.compose(
                        html! {
                            <SetlistLoad
                                catalog={catalog}
                                persistence_manager={persistence_manager}
                                serialized_setlist={serialized_setlist}
                                on_load={replace}
                                current_setlist={setlist}
                            />
                        },
                        self.view_nav(ctx, None),
                    )
                }
            },
            SetlistRoute::List => {
                let on_event = ctx.props().on_event.reform(|e| e);
                let persistence_manager = ctx.props().persistence_manager.clone();

                self.compose(
                    html! {
                        <SetlistList
                            persistence_manager={persistence_manager}
                            on_event={on_event}
                            setlists={vec![]}
                            state={state}
                        />
                    },
                    self.view_nav(ctx, None),
                )
            }
        }
    }

    fn view_user_route(&self, ctx: &Context<Self>, route: UserRoute) -> Html {
        let user = ctx.props().state.session().user().clone();
        let on_login_success = ctx.props().on_user_login_success.reform(|i| i);
        let on_login_error = ctx.props().on_user_login_error.reform(|i| i);

        match route {
            UserRoute::Info => html! { <UserInfo user={user} /> },
            UserRoute::Login => html! {
                <UserLogin
                    user={user}
                    config={self.config.clone()}
                    on_success={on_login_success}
                    on_error={on_login_error}
                />
            },
        }
    }

    fn view_nav(&self, ctx: &Context<Self>, current_song_id: Option<SongId>) -> Html {
        let on_toggle = ctx.link().callback(|_| Msg::ToggleMenu);
        let on_setlist_change = ctx.props().on_setlist_change.reform(|i| i);
        let state = ctx.props().state.clone();
        let current_song_info = current_song_id.and_then(|s| self.get_song_info(ctx, &s));
        let on_settings_change = ctx
            .props()
            .on_event
            .reform(App::reform_settings_change_to_event);

        html! {
            <Nav
                expand={self.expand}
                {current_song_info}
                {on_toggle}
                {on_settings_change}
                {on_setlist_change}
                {state}
            />
        }
    }

    fn reform_settings_change_to_event(s: (SongId, SongSettings)) -> Event {
        Event::Pair(
            Box::new(SettingsEvent::Change(s.0.clone(), s.1.clone()).into()),
            Box::new(SetlistEvent::SettingsChange(s.0, s.1).into()),
        )
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

    fn create(ctx: &Context<Self>) -> Self {
        // let mut route_service: RouteService<AppRouteState> = RouteService::new();
        // let route = route_service.get_route();
        // route_service.register_callback(ctx.link().callback(Msg::RouteChanged));

        let config = Config::default();

        Self {
            // _route_service: route_service,
            expand: true,
            // route,
            config,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Msg::RouteChanged(route) => self.route = route,
            Msg::RouteChanged => {}
            Msg::ToggleMenu => self.expand = !self.expand,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let main_classes = if self.expand {
            "-menu-visible"
        } else {
            "-menu-hidden"
        };

        debug!("Redraw App");
        let route = ctx.link().location().unwrap().route::<AppRoute>();

        (html! {
            <main class={main_classes}>
                {self.route(ctx, route)}
            </main>
        }) as Html
    }
}
