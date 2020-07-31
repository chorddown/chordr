use crate::components::detail_view::DetailView;
use libchordr::prelude::*;
use std::fmt::Display;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InfoProps {
    pub user: User,
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

        let username = row("Username", user.username());
        let first_name = row("First name", user.first_name());
        let last_name = row("Last name", user.last_name());
        let password = row("Password", user.password());

        html! {
            <DetailView>
                <table class="centered">
                    <tbody>
                        {username}
                        {first_name}
                        {last_name}
                        {password}
                    </tbody>
                </table>
            </DetailView>
        }
    }
}
