use crate::config::Config;
use crate::connection::{ConnectionService, ConnectionStatus};
use crate::helpers::Class;
use crate::session::{Session, SessionService, SessionUser};
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::IntervalService;

#[derive(Properties, Clone)]
pub struct NavItemProps {
    pub session: Rc<Session>,
    pub session_service: Rc<SessionService>,
}

pub enum Msg {
    CheckConnection,
    ConnectionStateChanged(ConnectionStatus),
}

pub struct NavItem {
    props: NavItemProps,
    link: ComponentLink<Self>,
    _clock_handle: IntervalTask,
    connection_state: ConnectionStatus,
}

impl Component for NavItem {
    type Message = Msg;
    type Properties = NavItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let clock_handle = IntervalService::spawn(
            Duration::from_secs(60),
            link.callback(|_| Msg::CheckConnection),
        );

        Self {
            props,
            link,
            _clock_handle: clock_handle,
            connection_state: ConnectionStatus::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CheckConnection => {
                log::info!("Tick");

                let connection_service = ConnectionService::new(Config::default());
                let change_connection_state =
                    self.link.callback(|i| Msg::ConnectionStateChanged(i));

                spawn_local(async move {
                    let connection_state = connection_service.get_connection_status().await;
                    change_connection_state.emit(connection_state)
                });

                true
            }

            Msg::ConnectionStateChanged(new_connection_state) => {
                if self.connection_state != new_connection_state {
                    self.connection_state = new_connection_state;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.session != props.session {
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let user = &self.props.session.user();

        let on_line = self.connection_state == ConnectionStatus::OnLine;
        let class = Class::new("user-state").add(if on_line {
            "on-line"
        } else {
            "offline disabled"
        });

        match user {
            SessionUser::LoggedIn(user) => {
                let user_description = format!(
                    "{} {} aka {}",
                    user.first_name(),
                    user.last_name(),
                    user.username()
                );
                let class = class.add("logged-in");

                (html! {
                    <a role="button" class=class href="/#/user/info" title=user_description>
                        <i class="im im-user-male"></i>
                        <i class="im im-check-mark"></i>
                    </a>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                let title = match self.connection_state {
                    ConnectionStatus::OnLine => "Click to log in",
                    ConnectionStatus::ServerNotReachable => "Server not reachable",
                    ConnectionStatus::Offline => "Device offline",
                };

                let class = class.add("not-logged-in");

                (html! {
                    <a role="button" class=class href="/#/user/login" title=title>
                        <i class="im im-user-male"></i>
                    </a>
                }) as Html
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::CheckConnection)
        }
    }
}
