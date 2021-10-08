use yew::prelude::*;

pub struct SongSearchLink {}

impl Component for SongSearchLink {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let href = format!("#/song-search/");

        html! { <a class="song-search-home" href=href><i class="im im-magnifier"></i></a> }
    }
}
