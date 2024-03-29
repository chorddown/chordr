use crate::connection::ConnectionStatus;
use crate::helpers::Class;
use crate::session::{Session, SessionUser};
use crate::state::State;
use std::rc::Rc;
use webchordr_common::prelude::*;
use yew::prelude::*;

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

pub struct NavItem {}

impl Component for NavItem {
    type Message = ();
    type Properties = NavItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &ctx.props().state;
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
                let class = class.add("logged-in").to_classes();
                let to = AppRoute::UserInfo;

                (html! {
                    <Link role="button" {class} {to} title={user_description}>
                        <i class="im im-user-male"></i>
                        <i class="im im-check-mark"></i>
                    </Link>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                let title = match state.connection_status() {
                    ConnectionStatus::OnLine => "Click to log in",
                    ConnectionStatus::ServerNotReachable => "Server not reachable",
                    ConnectionStatus::Offline => "Device offline",
                };

                let class = class.add("not-logged-in").to_classes();
                let to = AppRoute::UserLogin;

                (html! {
                    <Link role="button" {class} {to} title={title}>
                        <i class="im im-user-male"></i>
                    </Link>
                }) as Html
            }
        }
    }
}
