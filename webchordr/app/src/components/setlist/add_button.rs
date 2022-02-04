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

fn default_text() -> String {
    "Create new setlist".to_string()
}

#[derive(Properties, Clone, PartialEq)]
pub struct AddButtonProps {
    #[prop_or_else(default_text)]
    pub text: String,
    pub state: Rc<State>,
    pub on_click: Callback<Setlist>,
    /// Define if the entries of the currently loaded Setlist should be copied to the new Setlist
    pub clone_current: bool,
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
        let clone_current = self.props.clone_current;

        let on_click_parent = self.props.on_click.reform(|i| i);
        let on_click = self.link.callback_once(move |_| {
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
            <button class="setlist-add-button" onclick=on_click>
                <i class=icon></i>
                {&self.props.text}
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
