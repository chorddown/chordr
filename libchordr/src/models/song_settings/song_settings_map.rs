use crate::models::record_trait::RecordTrait;
use crate::models::song_id::SongId;
use crate::models::song_settings::SongSettings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A map of [SongSettings] to [SongId]s
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SongSettingsMap(HashMap<SongId, SongSettings>);

impl SongSettingsMap {
    pub fn new() -> Self {
        SongSettingsMap { 0: HashMap::new() }
    }

    pub fn store(&mut self, song_id: SongId, settings: SongSettings) -> Option<SongSettings> {
        self.0.insert(song_id, settings)
    }

    pub fn get(&self, song_id: &SongId) -> Option<&SongSettings> {
        self.0.get(song_id)
    }
}

impl Default for SongSettingsMap {
    fn default() -> Self {
        SongSettingsMap::new()
    }
}

impl RecordTrait for SongSettingsMap {
    type Id = &'static str;

    fn id(self) -> Self::Id {
        "song-settings"
    }
}
