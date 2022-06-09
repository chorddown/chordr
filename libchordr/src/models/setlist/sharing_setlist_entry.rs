use crate::data_exchange::serde_helper::deserialize_opt_isize_fromstr;
use crate::prelude::{ListEntryTrait, SetlistEntry, SongId};
use serde::{Deserialize, Serialize};

/// A subset of a `SetlistEntry` intended for sharing
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SharingSetlistEntry {
    pub id: SongId,
    #[serde(
        rename = "tr",
        default,
        deserialize_with = "deserialize_opt_isize_fromstr"
    )]
    pub transpose_semitone: Option<isize>, // settings.transpose_semitone
    #[serde(rename = "nt")]
    pub note: Option<String>, // settings.note
}

impl ListEntryTrait for SharingSetlistEntry {
    type Id = <SetlistEntry as ListEntryTrait>::Id;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl From<SetlistEntry> for SharingSetlistEntry {
    fn from(entry: SetlistEntry) -> Self {
        SharingSetlistEntry::from(&entry)
    }
}

impl From<&SetlistEntry> for SharingSetlistEntry {
    fn from(entry: &SetlistEntry) -> Self {
        SharingSetlistEntry {
            id: entry.id(),
            transpose_semitone: entry.settings().map(|s| s.transpose_semitone()),
            note: entry.settings().map(|s| s.note().to_string()),
        }
    }
}
