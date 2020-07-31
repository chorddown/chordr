pub mod command;
pub mod db;
pub mod repository;

use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist_entry::db::SetlistDbEntry;
use chrono::{DateTime, Utc};
use libchordr::prelude::{Setlist, SetlistEntry, Team, User};
use std::convert::TryInto;

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct UserSetlist {
//     pub id: i32,
//     pub user: i32,
//     pub user_name: String,
//     pub sorting: i32,
//     pub entries: Vec<SetlistEntry>,
//
// }

pub(crate) fn setlist_from_data(
    setlist_db: SetlistDb,
    db_entries: Vec<SetlistDbEntry>,
    owner: User,
    team: Option<Team>,
    // gig_date: Option<NaiveDateTime>,
    // creation_date: NaiveDateTime,
    // modification_date: NaiveDateTime,
) -> Setlist {
    let entries = db_entries
        .into_iter()
        .map(|e| e.try_into().expect("Could not convert setlist"))
        .collect::<Vec<SetlistEntry>>();

    Setlist::new(
        setlist_db.name,
        setlist_db.id,
        owner,
        team,
        setlist_db.gig_date.map(|s| DateTime::from_utc(s, Utc)),
        DateTime::from_utc(setlist_db.creation_date, Utc),
        DateTime::from_utc(setlist_db.modification_date, Utc),
        entries, //  setlist_db.songs: Vec<SetlistEntry>,
    )
    //     id: setlist_db.id,
    //     user: setlist_db.owner,
    //     user_name: setlist_db.user_name,
    //     sorting: setlist_db.sorting,
    //     entries,
    // }
}

// impl RecordIdTrait for UserSetlist {
//     type Id = i32;
//
//     fn id(self) -> Self::Id {
//         self.id
//     }
// }
