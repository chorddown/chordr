use crate::components::song_list::SongList;
use libchordr::models::setlist::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    #[props(required)]
    pub songs: Rc<Setlist<SetlistEntry>>,
    #[props(required)]
    pub show_menu: bool,
    #[props(required)]
    pub on_toggle: Callback<()>,
}

#[allow(dead_code)]
pub struct Nav {
    props: NavProps,
}

impl Nav {
    fn view_song_list(&self) -> Html {
        let songs = &self.props.songs;

        html! {
            <div class="song-list">
                <SongList songs=songs/>
            </div>
        }
    }

    fn view_nav_footer(&self) -> Html {
        let toggle_menu = self.props.on_toggle.reform(|_| ());

        (if self.props.show_menu {
            html! {
                <footer>
                    <button class="toggle-menu" onclick=toggle_menu>{ "→" }</button>
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
        if self.props.show_menu {
            menu_classes.push("-visible");
        } else {
            menu_classes.push("-hidden");
        };

        let song_list = if self.props.show_menu {
            self.view_song_list()
        } else {
            html! {}
        };

        html! {
            <nav class=menu_classes>
                { song_list }
                { self.view_nav_footer() }
            </nav>
        }
    }
}
