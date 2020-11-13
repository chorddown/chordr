use yew::prelude::*;
use crate::session::{SessionUser, Session};
use std::rc::Rc;

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

        match user {
            SessionUser::LoggedIn(user) => {
                let user_description = format!("{} {} aka {}", user.first_name(), user.last_name(), user.username());

                (html! {
                    <a role="button" class="user-state logged-in" href="/#/user/info" title=user_description>
                        <i class="im im-user-male"></i>
                        <i class="im im-check-mark"></i>
                    </a>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                (html! {
                    <a role="button" class="user-state not-logged-in" href="/#/user/login" title="Click to log in">
                        <i class="im im-user-male"></i>
                    </a>
                }) as Html
            }
        }
    }
}
