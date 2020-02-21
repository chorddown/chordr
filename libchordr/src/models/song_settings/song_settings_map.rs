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
