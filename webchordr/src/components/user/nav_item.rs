use yew::prelude::*;
use crate::session::{SessionUser, Session};
use std::rc::Rc;
use crate::helpers::{window, Class};

#[derive(Properties, PartialEq, Clone)]
pub struct NavItemProps {
    pub session: Rc<Session>,
}

pub enum Msg {}

pub struct NavItem {
    props: NavItemProps,
    _link: ComponentLink<Self>,
}

impl Component for NavItem {
    type Message = Msg;
    type Properties = NavItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, _link: link }
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
        let user = &self.props.session.user();

        let on_line = window().navigator().on_line();
        let class = Class::new("user-state").add(if on_line { "on-line" } else { "offline disabled" });

        match user {
            SessionUser::LoggedIn(user) => {
                let user_description = format!("{} {} aka {}", user.first_name(), user.last_name(), user.username());
                let class = class.add("logged-in");

                (html! {
                    <a role="button" class=class href="/#/user/info" title=user_description>
                        <i class="im im-user-male"></i>
                        <i class="im im-check-mark"></i>
                    </a>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                let title = if on_line { "Click to log in" } else { "Device offline" };
                let class = class.add("not-logged-in");

                (html! {
                    <a role="button" class=class href="/#/user/login" title=title>
                        <i class="im im-user-male"></i>
                    </a>
                }) as Html
            }
        }
    }
}
