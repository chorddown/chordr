use crate::components::setlist::SetlistShareButton;
use crate::components::song_view::SongNotes;
use crate::components::user::NavItem as UserNavButton;
use crate::service::song_info_service::SongInfoService;
use crate::state::{SongInfo, State};
use libchordr::models::song_list::SongList as SongListModel;
use libchordr::prelude::{ListEntryTrait, SongId, SongSettings};
use std::rc::Rc;
use webchordr_common::route::route;
use webchordr_drag_n_drop::{Dropzone, OnDropArgument};
use webchordr_events::Event;
use webchordr_events::SetlistEvent;
use webchordr_song_list::SongList as SongListView;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct NavProps {
    pub expand: bool,
    pub on_toggle: Callback<()>,
    pub on_setlist_change: Callback<Event>,
    pub on_settings_change: Callback<(SongId, SongSettings)>,
    pub current_song_info: Option<SongInfo>,
    pub state: Rc<State>,
}

impl PartialEq for NavProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
            && self.current_song_info == other.current_song_info
            && self.expand == other.expand
    }
}

#[allow(dead_code)]
pub struct Nav {
    props: NavProps,
}

impl Nav {
    fn view_song_list(&self) -> Html {
        let state = self.props.state.clone();
        let current_setlist = state.current_setlist();
        let songs = match &current_setlist {
            Some(setlist) => setlist.as_song_list(),
            None => SongListModel::new(),
        };
        let on_setlist_change = self.props.on_setlist_change.reform(|e| e);
        let highlighted_song_id = self
            .props
            .current_song_info
            .as_ref()
            .map(|info| info.song.id());
        let name_element = match current_setlist {
            Some(s) if !s.name().is_empty() => {
                html! {<header class="setlist-name">{s.name()}</header>}
            }
            _ => html! {},
        };

        let on_setlist_change_after_drop = on_setlist_change.clone();
        let on_drop = Callback::from(move |drop_arguments: OnDropArgument| {
            if let Some(song_id) = drop_arguments.dataset.get("songId") {
                let song_info_service = SongInfoService::new();
                let id = SongId::from(song_id);
                if let Some(song_info) = song_info_service.get_song_info_from_state(&id, &state) {
                    on_setlist_change_after_drop
                        .emit(SetlistEvent::AddEntry(song_info.into()).into())
                }
            }
        });
        let item_selector = ".song-browser-song-list.song-list .grid-button".to_string();

        html! {
            <Dropzone class="song-list-container" on_drop=on_drop item_selector=item_selector>
                {name_element}
                <SongListView
                    songs=songs
                    on_setlist_change=on_setlist_change
                    sortable=self.props.expand
                    highlighted_song_id=highlighted_song_id
                />
            </Dropzone>
        }
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.props.on_toggle.reform(|_| ());

        let session = self.props.state.session();
        let state = self.props.state.clone();
        let setlist_share_button = match &state.current_setlist().clone() {
            Some(s) => html! { <SetlistShareButton setlist=s/>},
            None => html! {},
        };

        let home_button = match state.available_version() {
            Some(_) => html! {
                <a role="button" class="update" href="/" title="Update">
                    <i class="im im-sync"></i>
                    <span>{ "Update" }</span>
                </a>
            },
            None => html! {
                <a role="button" class="home" href="/#" title="Go to home screen">
                    <i class="im im-home"></i>
                    <span>{ "Home" }</span>
                </a>
            },
        };

        let user_nav_button = if cfg!(feature = "server_sync") {
            html! {<UserNavButton state=state session=session />}
        } else {
            html! {}
        };

        (if self.props.expand {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>
                        <i class="im im-angle-right"></i>
                    </button>
                    {setlist_share_button}
                    {home_button}
                    <a role="button" class="setlist" href={route("/setlist/list")} title="List setlists">
                        <i class="im im-data"></i>
                        <span>{ "Setlist" }</span>
                    </a>
                    {user_nav_button}
                </footer>
            }
        } else {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>
                        <i class="im im-angle-left"></i>
                    </button>
                </footer>
            }
        }) as Html
    }

    fn view_notes_section(&self) -> Html {
        (match &self.props.current_song_info {
            Some(i) => {
                let on_settings_change = self.props.on_settings_change.reform(|e| e);

                html! {<SongNotes song_info=i.clone() on_change=on_settings_change/>}
            }
            None => html! {},
        }) as Html
    }
}

impl Component for Nav {
    type Message = ();
    type Properties = NavProps;

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
        let mut menu_classes = vec!["menu"];
        if self.props.expand {
            menu_classes.push("-visible");
        } else {
            menu_classes.push("-hidden");
        };
        if self.props.current_song_info.is_some() {
            menu_classes.push("-w-notes");
        }

        (html! {
            <nav class=menu_classes>
                { self.view_song_list() }
                { self.view_notes_section() }
                { self.view_nav_footer() }
            </nav>
        }) as Html
    }
}
