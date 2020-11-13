// use crate::components::setlist::SetlistShareButton;
use crate::components::song_list::SongList as SongListView;
use crate::events::Event;
use libchordr::models::setlist::*;
use libchordr::models::song_list::SongList as SongListModel;
use libchordr::prelude::SongId;
use std::rc::Rc;
use yew::prelude::*;
use crate::session::Session;
use crate::components::user::NavItem;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub songs: Option<Rc<Setlist>>,
    pub expand: bool,
    pub current_song_id: Option<SongId>,
    pub on_toggle: Callback<()>,
    pub on_setlist_change: Callback<Event>,
    pub session: Rc<Session>,
}

#[allow(dead_code)]
pub struct Nav {
    props: NavProps,
}

impl Nav {
    fn view_song_list(&self) -> Html {
        let songs = match &self.props.songs {
            Some(setlist) => setlist.as_song_list(),
            None => SongListModel::new(),
        };
        let on_setlist_change = self.props.on_setlist_change.reform(|e| e);

        html! {
            <SongListView
                songs=songs
                on_setlist_change=on_setlist_change
                sortable=self.props.expand
                highlighted_song_id=self.props.current_song_id.clone()
            />
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
        let session = self.props.session.clone();

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
                    <NavItem session=session />
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
                { self.view_nav_footer() }
            </nav>
        }) as Html
    }
}
