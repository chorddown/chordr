use crate::components::detail_view::DetailView;
use crate::session::SessionUser;
use std::fmt::Display;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InfoProps {
    pub user: SessionUser,
}

pub enum Msg {}

pub struct Info {}

impl Component for Info {
    type Message = Msg;
    type Properties = InfoProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = &ctx.props().user;

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
