use libchordr::prelude::{SongId, SongSettings, SongSettingsMap};
use yew::services::StorageService;

pub trait SettingsHandler {
    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings) -> ();

    fn get_settings(storage_service: &StorageService) -> SongSettingsMap;

    fn commit_changes(&mut self);
}
