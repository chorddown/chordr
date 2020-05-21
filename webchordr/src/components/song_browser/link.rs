use super::SONG_BROWSER_PLACEHOLDER;
use yew::prelude::*;

pub struct SongBrowserLink {}

impl Component for SongBrowserLink {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SongBrowserLink {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let href = format!("#/song-browser/{}", SONG_BROWSER_PLACEHOLDER);

        html! { <a class="song-browser-home" href=href><i class="im im-home"></i></a> }
    }
}
