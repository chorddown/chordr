use std::rc::Rc;

use yew::prelude::*;

use libchordr::models::song_list::SongList as SongListModel;
use libchordr::prelude::{ListEntryTrait, SongId, SongSettings};
use webchordr_events::Event;
use webchordr_song_list::SongList as SongListView;

// use crate::components::setlist::SetlistShareButton;
use crate::components::song_view::SongNotes;
use crate::components::user::NavItem as UserNavButton;
use crate::state::{SongInfo, State};

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
        let current_setlist = self.props.state.current_setlist();
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

        html! {
            <div class="song-list-container">
                {name_element}
                <SongListView
                    songs=songs
                    on_setlist_change=on_setlist_change
                    sortable=self.props.expand
                    highlighted_song_id=highlighted_song_id
                />
            </div>
        }
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.props.on_toggle.reform(|_| ());

        // TODO: Implement sharing support
        // let setlist_share_button = match &self.props.songs {
        //     Some(s) => html! { <SetlistShareButton setlist=s.clone()/>},
        //     None => html! {},
        // };
        let setlist_share_button = html! {};
        let session = self.props.state.session();
        let state = self.props.state.clone();

        (if self.props.expand {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>
                        <i class="im im-angle-right"></i>
                    </button>
                    {setlist_share_button}
                    <a role="button" class="home" href="/#" title="Go to home screen">
                        <i class="im im-home"></i>
                        <span>{ "Home" }</span>
                    </a>
                    <a role="button" class="home" href="/#/setlist/list" title="List setlists">
                        <i class="im im-data"></i>
                        <span>{ "Setlist" }</span>
                    </a>
                    <UserNavButton state=state session=session />
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
        // TODO: Check if the Song Notes should be hidden if the nav is collapsed
        // if !self.props.expand {
        //     return html! {};
        // }

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

        (html! {
            <nav class=menu_classes>
                { self.view_song_list() }
                { self.view_notes_section() }
                { self.view_nav_footer() }
            </nav>
        }) as Html
    }
}
