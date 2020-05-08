use crate::diesel::QueryDsl;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use crate::schema::setlist;
use crate::schema::setlist::dsl::setlist as all_setlists;
use crate::ConnectionType;
use diesel::{self, prelude::*};

#[table_name = "setlist"]
#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, AsChangeset, Debug, Clone)]
pub struct SetlistDb {
    pub id: i32,
    pub user: i32,
    pub user_name: String,
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
