use yew_router::prelude::*;
use libchordr::models::song_id::SongId;
use yew_router::switch::AllowMissing;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/#/song/{id}"]
    Song(SongId),
    #[to = "/#/song-browser/{id}"]
    SongBrowser(String),
    #[to = "/index"]
    Index,
}
