use webchordr_common::route::route;
use yew::prelude::*;

pub struct SongSearchLink {}

impl Component for SongSearchLink {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let href = route("song-search/");

        html! { <a class="song-search-home" href={href}><i class="im im-magnifier"></i></a> }
    }
}
