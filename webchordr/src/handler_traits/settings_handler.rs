use crate::events::SettingsEvent;
use libchordr::prelude::{SongId, SongSettings, SongSettingsMap};

pub trait SettingsHandler {
    fn handle_settings_event(&mut self, event: SettingsEvent);

    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) -> ();

    fn song_settings_replace(&mut self, settings: SongSettingsMap);

    fn fetch_song_settings(&mut self);

    fn commit_changes(&mut self);
}
