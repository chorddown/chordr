use crate::domain::setlist::db::SetlistDb;
use crate::error::SrvError;
use crate::schema::setlist_entry;
use crate::schema::setlist_entry::dsl::setlist_entry as setlist_entries;
use crate::ConnectionType;
use chrono::NaiveDateTime;
use diesel::{self, prelude::*};
use libchordr::models::file_type::FileType;
use libchordr::models::list::ListEntryTrait;
use libchordr::models::song_data::SongData;
use libchordr::models::song_id::{SongId, SongIdTrait};
use libchordr::prelude::{SetlistEntry, SongSettings};
use std::convert::{TryFrom, TryInto};

#[derive(
    Serialize, Identifiable, Associations, Queryable, Insertable, AsChangeset, Debug, Clone,
)]
#[table_name = "setlist_entry"]
#[belongs_to(SetlistDb)]
pub struct SetlistDbEntry {
    pub id: Option<i32>,
    pub song_id: String,
    pub file_type: String,
    pub title: Option<String>,
    pub settings: Option<String>,
    pub setlist_db_id: i32,
    pub modification_date: Option<NaiveDateTime>,
}

impl SetlistDbEntry {
    pub fn all(conn: &ConnectionType) -> Vec<SetlistDbEntry> {
        setlist_entries.load::<SetlistDbEntry>(conn).unwrap()
    }

    pub fn count_all(conn: &ConnectionType) -> i64 {
        setlist_entries.count().get_result(conn).unwrap()
    }

    pub fn find_by_setlist(
        conn: &ConnectionType,
        setlist: &SetlistDb,
    ) -> Result<Vec<SetlistDbEntry>, SrvError> {
        Ok(SetlistDbEntry::belonging_to(setlist).load::<SetlistDbEntry>(conn)?)
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
            settings: entry.settings().map(|s| serialize_song_settings(&s)),
            setlist_db_id: setlist_db.id,
            modification_date: None,
        }
    }
}

fn serialize_song_settings(s: &SongSettings) -> String {
    serde_json::to_string(&s).expect("Settings could not be serialized")
}

fn deserialize_song_settings(s: &str) -> Result<SongSettings, SrvError> {
    Ok(serde_json::from_str(s)?)
}

impl SongIdTrait for SetlistDbEntry {}

impl ListEntryTrait for SetlistDbEntry {
    type Id = SongId;
    fn id(&self) -> SongId {
        SongId::new(&self.song_id)
    }
}

impl SongData for SetlistDbEntry {
    fn title(&self) -> String {
        self.title.clone().unwrap_or_default()
    }

    fn file_type(&self) -> FileType {
        FileType::try_from(self.file_type.as_str()).unwrap_or(FileType::Chorddown)
    }
}

impl TryFrom<SetlistDbEntry> for SetlistEntry {
    type Error = SrvError;

    fn try_from(value: SetlistDbEntry) -> Result<Self, Self::Error> {
        let settings = match value.settings {
            Some(s) => Some(deserialize_song_settings(&s)?),
            None => None,
        };

        Ok(SetlistEntry::new(
            value.song_id.as_str(),
            value.file_type.try_into()?,
            value.title.unwrap_or_default(),
            settings,
        ))
    }
}
