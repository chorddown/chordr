use crate::components::setlist::SetlistShareButton;
use crate::components::song_list::SongList;
use crate::events::Event;
use libchordr::models::setlist::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub songs: Rc<Setlist<SetlistEntry>>,
    pub expand: bool,
    pub on_toggle: Callback<()>,
    pub on_setlist_change: Callback<Event>,
}

#[allow(dead_code)]
pub struct Nav {
    props: NavProps,
}

impl Nav {
    fn view_song_list(&self) -> Html {
        let songs = &self.props.songs;
        let on_setlist_change = self.props.on_setlist_change.reform(|e| e);

        html! {
            <SongList songs=songs on_setlist_change=on_setlist_change sortable=self.props.expand />
        }
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.props.on_toggle.reform(|_| ());

        (if self.props.expand {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "→" }</button>
                    <SetlistShareButton setlist=self.props.songs.clone()/>
                    <a role="button" class="home" href="/#" title="Go to home screen">
                        <i class="im im-home"></i>
                        <span>{ "Home" }</span>
                    </a>
                </footer>
            }
        } else {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "︎←" }</button>
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
        })
    }
}
