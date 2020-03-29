use serde::{Deserialize, Serialize};
use super::SortingChange;
use libchordr::prelude::{SetlistEntry, SongId};
use crate::events::EventTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SetlistEvent {
    Add(SetlistEntry),
    Remove(SongId),
    SortingChange(SortingChange),
}

impl EventTrait for SetlistEvent {}
