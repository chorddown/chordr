pub mod command;
pub mod db;
pub mod repository;

use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::traits::RecordIdTrait;
use libchordr::prelude::SetlistEntry;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserSetlist {
    pub id: i32,
    pub user: i32,
    pub user_name: String,
    pub sorting: i32,
    pub entries: Vec<SetlistEntry>,
}

impl UserSetlist {
    pub fn from_data(setlist_db: SetlistDb, db_entries: Vec<SetlistDbEntry>) -> Self {
        let entries = db_entries
            .iter()
            .map(SetlistEntry::from_song)
            .collect::<Vec<SetlistEntry>>();

        Self {
            id: setlist_db.id,
            user: setlist_db.user,
            user_name: setlist_db.user_name,
            sorting: setlist_db.sorting,
            entries,
        }
    }
}

impl From<(SetlistDb, Vec<SetlistDbEntry>)> for UserSetlist {
    fn from(tuple: (SetlistDb, Vec<SetlistDbEntry>)) -> Self {
        Self::from_data(tuple.0, tuple.1)
    }
}

impl RecordIdTrait for UserSetlist {
    type Id = i32;

    fn id(self) -> Self::Id {
        self.id
    }
}
