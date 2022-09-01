use std::rc::Rc;

use chrono::Utc;
use gloo_dialogs::prompt;
use yew::prelude::*;
use yew::Callback;

use libchordr::models::setlist::SetlistEntry;
use libchordr::models::team::Team;
use libchordr::prelude::{Setlist, User};
use webchordr_common::session::SessionUser;

use crate::state::State;

#[derive(Properties, Clone, PartialEq)]
pub struct AddButtonProps {
    pub text: String,
    pub state: Rc<State>,
    pub on_click: Callback<Setlist>,
    /// Define if the entries of the currently loaded Setlist should be copied to the new Setlist
    pub clone_current: bool,
}

pub struct AddButton {}

impl Component for AddButton {
    type Message = ();
    type Properties = AddButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let state = props.state.clone();
        let clone_current = props.clone_current;

        let on_click_parent = props.on_click.clone();
        let on_click = ctx.link().callback(move |_| {
            if let Some(setlist) = AddButton::build_new_setlist(&state, clone_current) {
                on_click_parent.emit(setlist)
            }
        });

        let icon = if clone_current {
            "im im-copy"
        } else {
            "im im-plus"
        };

        html! {
            <button class="setlist-add-button" onclick={on_click}>
                <i class={icon}></i>
                {&ctx.props().text}
            </button>
        }
    }
}

impl AddButton {
    fn build_new_setlist(state: &State, clone_current: bool) -> Option<Setlist> {
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

        let current_setlist = state.current_setlist();

        let songs = AddButton::build_song_list(current_setlist.clone(), clone_current);
        let team = AddButton::build_team(current_setlist, clone_current);

        Some(Setlist::new(
            name,
            now_utc.timestamp() as i32,
            user,
            team,
            None,
            now_utc,
            now_utc,
            songs,
        ))
    }

    fn build_team(current_setlist: Option<Rc<Setlist>>, clone_current: bool) -> Option<Team> {
        if !clone_current {
            return None;
        }

        current_setlist.and_then(|s| s.team().clone())
    }

    fn build_song_list(
        current_setlist: Option<Rc<Setlist>>,
        clone_current: bool,
    ) -> Vec<SetlistEntry> {
        if !clone_current {
            return vec![];
        }
        match current_setlist {
            Some(ref s) => s.as_ref().clone().into_iter().collect(),
            None => vec![],
        }
    }
}
