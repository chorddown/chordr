use std::rc::Rc;

use chrono::Utc;
use gloo_dialogs::prompt;
use yew::prelude::*;
use yew::Callback;

use libchordr::prelude::{Setlist, User};
use webchordr_common::session::SessionUser;

use crate::state::State;

#[derive(Properties, Clone, PartialEq)]
pub struct AddButtonProps {
    pub state: Rc<State>,
    pub on_click: Callback<Setlist>,
}
pub struct AddButton {
    props: AddButtonProps,
    link: ComponentLink<Self>,
}

impl Component for AddButton {
    type Message = ();
    type Properties = AddButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        let state = self.props.state.clone();

        let on_click_parent = self.props.on_click.reform(|i| i);
        let on_click = self.link.callback_once(move |_| {
            if let Some(setlist) = AddButton::build_new_setlist(&state) {
                on_click_parent.emit(setlist)
            }
        });

        html! {
            <button class="setlist-add-button" onclick=on_click>
                <i class="im im-plus"></i>
                {"Create new setlist"}
            </button>
        }
    }
}

impl AddButton {
    fn build_new_setlist(state: &State) -> Option<Setlist> {
        let name = match prompt("Name of the new setlist:", None) {
            Some(name) if !name.trim().is_empty() => name,
            _ => return None,
        };

        let session = state.session();
        let now_utc = Utc::now();

        let user = match session.user() {
            SessionUser::LoggedIn(user) => user.clone(),
            SessionUser::Unauthenticated => User::unknown(),
        };

        Some(Setlist::new(
            name,
            now_utc.timestamp() as i32,
            user,
            None,
            None,
            now_utc,
            now_utc,
            vec![],
        ))
    }
}
