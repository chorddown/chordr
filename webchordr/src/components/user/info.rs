use crate::components::detail_view::DetailView;
use std::fmt::Display;
use yew::prelude::*;
use crate::session::SessionUser;

#[derive(Properties, PartialEq, Clone)]
pub struct InfoProps {
    pub user: SessionUser,
}

pub enum Msg {}

pub struct Info {
    props: InfoProps,
    _link: ComponentLink<Self>,
}

impl Component for Info {
    type Message = Msg;
    type Properties = InfoProps;

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
        let user = &self.props.user;

        fn row(caption: &str, value: impl Display) -> Html {
            html! { <tr><th>{caption}</th><td>{value.to_string()}</td></tr> }
        }

        match user {
            SessionUser::LoggedIn(user) => {
                let username = row("Username", user.username());
                let first_name = row("First name", user.first_name());
                let last_name = row("Last name", user.last_name());

                (html! {
                    <DetailView>
                        <table class="centered">
                            <tbody>
                                {username}
                                {first_name}
                                {last_name}
                            </tbody>
                        </table>
                    </DetailView>
                }) as Html
            }
            SessionUser::Unauthenticated => {
                let not_logged_in = row("Not logged in", "");

                (html! {
                    <DetailView>
                        <table class="centered">
                            <tbody>
                                {not_logged_in}
                            </tbody>
                        </table>
                    </DetailView>
                }) as Html
            }
        }
    }
}
