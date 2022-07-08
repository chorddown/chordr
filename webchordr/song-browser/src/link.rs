use super::SONG_BROWSER_PLACEHOLDER;
use webchordr_common::components::link::Link;
use webchordr_common::route::{route, AppRoute};
use yew::prelude::*;

pub struct SongBrowserLink {}

impl Component for SongBrowserLink {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SongBrowserLink {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let href = route(format!("song-browser/{}", SONG_BROWSER_PLACEHOLDER));

        let to = AppRoute::SongBrowser {
            chars: SONG_BROWSER_PLACEHOLDER.to_string(),
        };
        html! { <Link class="song-browser-home" {to}><i class="im im-home"></i></Link> }
    }
}
