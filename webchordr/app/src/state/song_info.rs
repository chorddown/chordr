use yew::prelude::*;

use libchordr::prelude::{ListEntryTrait, SetlistEntry, Song, SongData, SongSettings};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct SongInfo {
    pub song: Song,
    pub song_settings: SongSettings,
    pub is_on_setlist: bool,
}

impl From<SongInfo> for SetlistEntry {
    fn from(song_info: SongInfo) -> Self {
        SetlistEntry::new(
            song_info.song.id(),
            song_info.song.file_type(),
            song_info.song.title(),
            Some(song_info.song_settings),
        )
    }
}
