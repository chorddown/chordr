use crate::events::EventTrait;
use libchordr::prelude::{SongId, SongSettings, SongSettingsMap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SettingsEvent {
    /// Settings for a Song changed
    Change(SongId, SongSettings),

    /// Replace the complete Map of Song Settings
    Replace(SongSettingsMap),
}

impl EventTrait for SettingsEvent {}
