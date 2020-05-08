use crate::schema::setlist_entry;
use crate::schema::setlist_entry::dsl::setlist_entry as setlist_entries;
use crate::ConnectionType;
use diesel::{self, prelude::*};
use libchordr::models::file_type::FileType;
use libchordr::models::song_data::SongData;
use libchordr::models::song_id::{SongId, SongIdTrait};
use std::convert::TryFrom;
use crate::domain::setlist::db::SetlistDb;
use libchordr::prelude::SetlistEntry;

#[derive(Serialize, Identifiable, Associations, Queryable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "setlist_entry"]
#[belongs_to(SetlistDb)]
pub struct SetlistDbEntry {
    pub id: Option<i32>,
    pub song_id: String,
    pub file_type: String,
    pub title: Option<String>,
    pub setlist_db_id: i32,
}

impl SetlistDbEntry {
    pub fn all(conn: &ConnectionType) -> Vec<SetlistDbEntry> {
        setlist_entries.load::<SetlistDbEntry>(conn).unwrap()
    }

    pub fn count_all(conn: &ConnectionType) -> i64 {
        setlist_entries.count().get_result(conn).unwrap()
    }

    pub fn find_by_setlist(conn: &ConnectionType, setlist: &SetlistDb) -> Vec<SetlistDbEntry> {
        SetlistDbEntry::belonging_to(setlist)
            .load::<SetlistDbEntry>(conn)
            .expect("Error loading entries")
    }
    pub fn delete_all(conn: &ConnectionType) -> bool {
        diesel::delete(setlist_entries).execute(conn).is_ok()
    }

    pub fn from(entry: &SetlistEntry, setlist_db: &SetlistDb) -> Self {
        Self {
            id: None,
            song_id: entry.id().to_string(),
            file_type: entry.file_type().to_string(),
            title: Some(entry.title()),
            setlist_db_id: setlist_db.id,
        }
    }
}

impl SongIdTrait for SetlistDbEntry {
    fn id(&self) -> SongId {
        SongId::new(&self.song_id)
    }
}

impl SongData for SetlistDbEntry {
    fn title(&self) -> String {
        self.title.clone().unwrap_or(String::new())
    }

    fn file_type(&self) -> FileType {
        FileType::try_from(self.file_type.as_str()).unwrap_or(FileType::Chorddown)
    }
}
