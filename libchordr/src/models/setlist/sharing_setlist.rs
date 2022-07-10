use crate::data_exchange::serde_helper::deserialize_i32_fromstr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::list::List;
use crate::models::setlist::sharing_setlist_entry::SharingSetlistEntry;
use crate::prelude::Setlist;

/// A subset of a `Setlist` intended for sharing
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharingSetlist {
    pub name: String,

    #[serde(deserialize_with = "deserialize_i32_fromstr")]
    pub id: i32,
    // owner: User,
    // team: Option<Team>,
    pub songs: List<SharingSetlistEntry>,
    pub gig_date: Option<DateTime<Utc>>,
    pub creation_date: DateTime<Utc>,
    pub modification_date: DateTime<Utc>,
}

impl From<Setlist> for SharingSetlist {
    fn from(s: Setlist) -> Self {
        SharingSetlist {
            name: s.name,
            id: s.id,
            songs: s
                .songs
                .iter()
                .map(SharingSetlistEntry::from)
                .collect::<List<SharingSetlistEntry>>(),
            gig_date: s.gig_date,
            creation_date: s.creation_date,
            modification_date: s.modification_date,
        }
    }
}

impl From<&Setlist> for SharingSetlist {
    fn from(s: &Setlist) -> Self {
        SharingSetlist {
            name: s.name.clone(),
            id: s.id,
            songs: s
                .songs
                .iter()
                .map(SharingSetlistEntry::from)
                .collect::<List<SharingSetlistEntry>>(),
            gig_date: s.gig_date,
            creation_date: s.creation_date,
            modification_date: s.modification_date,
        }
    }
}
