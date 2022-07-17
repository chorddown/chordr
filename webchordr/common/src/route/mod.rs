pub use link_to::LinkTo;
pub use song_id_param::SongIdParam;
use yew_router::prelude::*;

mod link_to;
mod song_id_param;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/song/:id")]
    Song { id: SongIdParam },
    #[at("/song-browser/:chars")]
    SongBrowser { chars: String },
    #[at("/song-search")]
    SongSearch,
    #[at("/setlist/list")]
    SetlistList,
    #[at("/setlist/load/:serialized_setlist")]
    SetlistLoad { serialized_setlist: String },
    #[at("/setlist/:r")]
    Setlist,
    #[at("/user/info")]
    UserInfo,
    #[at("/user/login")]
    UserLogin,
    #[at("/user/:r")]
    User,
    #[at("/")]
    Index,
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum SetlistRoute {
    #[at("/setlist/list")]
    List,
    #[at("/setlist/load/:serialized_setlist")]
    Load { serialized_setlist: String },
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum UserRoute {
    #[at("/user/info")]
    Info,
    #[at("/user/login")]
    Login,
}

pub fn route<S: AsRef<str>>(route: S) -> String {
    format!("/{}", route.as_ref().trim_start_matches('/'))
}
