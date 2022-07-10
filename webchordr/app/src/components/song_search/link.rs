use webchordr_common::components::link::Link;
use webchordr_common::route::AppRoute;
use yew::prelude::*;

#[function_component(SongSearchLink)]
pub(crate) fn song_search_link() -> Html {
    let to = AppRoute::SongSearch;

    html! { <Link class="song-search-home" {to}><i class="im im-magnifier"></i></Link> }
}
