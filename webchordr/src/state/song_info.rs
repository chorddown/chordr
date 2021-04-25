use yew::prelude::*;

use libchordr::prelude::{Song, SongSettings};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct SongInfo {
    pub song: Song,
    pub song_settings: SongSettings,
    pub is_on_setlist: bool,
}
