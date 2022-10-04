use crate::diesel::QueryDsl;
use crate::domain::setlist::setlist_db_id::{get_setlist_db_id, SetlistDbId};
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::schema::setlist;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::ConnectionType;
use chrono::prelude::*;
use diesel::{self, prelude::*};
use libchordr::prelude::Setlist;

#[derive(
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Insertable,
    AsChangeset,
    Debug,
    Clone,
    PartialEq,
)]
#[table_name = "setlist"]
#[primary_key(uid)]
pub struct SetlistDb {
    pub uid: SetlistDbId,
    pub id: i32,
    pub name: String,
    pub sorting: i32,
    pub owner: String,
    pub team: Option<String>,
    pub gig_date: Option<NaiveDateTime>,
    pub creation_date: NaiveDateTime,
    pub modification_date: NaiveDateTime,
}

impl SetlistDb {
    pub fn all(conn: &ConnectionType) -> Vec<SetlistDb> {
        all_setlists
            .order(setlist::id.desc())
            .load::<SetlistDb>(conn)
            .unwrap()
    }

    pub fn count_all(conn: &ConnectionType) -> i64 {
        all_setlists.count().get_result(conn).unwrap()
    }

    pub fn entries(&self, conn: &ConnectionType) -> Vec<SetlistDbEntry> {
        SetlistDbEntry::belonging_to(self)
            .load::<SetlistDbEntry>(conn)
            .expect("Error loading entries")
    }

    pub fn delete_all(conn: &ConnectionType) -> bool {
        diesel::delete(all_setlists).execute(conn).is_ok()
    }
}

impl From<&Setlist> for SetlistDb {
    fn from(setlist: &Setlist) -> Self {
        Self {
            uid: get_setlist_db_id(setlist),
            name: setlist.name().to_owned(),
            id: setlist.id(),
            owner: setlist.owner().username().to_string(),
            team: setlist.team().as_ref().map(|t| t.id().to_string()),
            gig_date: setlist.gig_date().map(|d| d.naive_utc()),
            creation_date: setlist.creation_date().naive_utc(),
            sorting: 0, // setlist.sorting(),
            modification_date: setlist.modification_date().naive_utc(),
        }
    }
}

impl From<Setlist> for SetlistDb {
    fn from(setlist: Setlist) -> Self {
        Self::from(&setlist)
    }
}
