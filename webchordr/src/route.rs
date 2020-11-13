use libchordr::models::song_id::SongId;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#/song/{id}"]
    Song(SongId),
    #[to = "/#/song-browser/{id}"]
    SongBrowser(String),
    #[to = "/#/song-search"]
    SongSearch,
    #[to = "/#/setlist{*:rest}"]
    Setlist(SetlistRoute),
    #[to = "/#/user{*:rest}"]
    User(UserRoute),
    #[to = "/index"]
    Index,
}

#[derive(Switch, Debug, Clone)]
pub enum SetlistRoute {
    #[to = "/load/{serialized_setlist}"]
    Load { serialized_setlist: String },
}

#[derive(Switch, Debug, Clone)]
pub enum UserRoute {
    #[to = "/info"]
    Info,
    #[to = "/login"]
    Login,
}
