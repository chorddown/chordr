use std::rc::Rc;

use webchordr_common::route::route;
use yew::prelude::*;

use crate::connection::ConnectionStatus;
use crate::helpers::Class;
use crate::session::{Session, SessionUser};
use crate::state::State;

#[derive(Properties, Clone)]
pub struct NavItemProps {
    pub state: Rc<State>,
    pub session: Rc<Session>,
}

impl PartialEq for NavItemProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state) && Rc::ptr_eq(&self.session, &other.session)
    }
}

pub enum Msg {}

pub struct NavItem {
    props: NavItemProps,
}

impl Component for NavItem {
    type Message = Msg;
    type Properties = NavItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let state = &self.props.state;
        let session = state.session();
        let user = session.user();

        let on_line = state.connection_status() == ConnectionStatus::OnLine;
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
                    <a role="button" class=class href={route("/user/info")} title=user_description>
                        <i class="im im-user-male"></i>
                        <i class="im im-check-mark"></i>
                    </a>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                let title = match state.connection_status() {
                    ConnectionStatus::OnLine => "Click to log in",
                    ConnectionStatus::ServerNotReachable => "Server not reachable",
                    ConnectionStatus::Offline => "Device offline",
                };

                let class = class.add("not-logged-in");

                (html! {
                    <a role="button" class=class href={route("/user/login")} title=title>
                        <i class="im im-user-male"></i>
                    </a>
                }) as Html
            }
        }
    }
}
