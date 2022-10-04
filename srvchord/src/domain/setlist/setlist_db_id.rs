use libchordr::models::setlist::SetlistId;
use libchordr::prelude::{Setlist, Username};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type SetlistDbId = i64;

pub trait ToSetlistDbId {
    fn to_setlist_db_uid(self) -> SetlistDbId;
}

pub fn get_setlist_db_id(setlist: &Setlist) -> SetlistDbId {
    let mut hasher = DefaultHasher::new();
    (setlist.id(), setlist.owner().username().to_string()).hash(&mut hasher);

    hasher.finish() as i64
}

impl ToSetlistDbId for &Setlist {
    fn to_setlist_db_uid(self) -> SetlistDbId {
        (self.id(), self.owner().username()).to_setlist_db_uid()
    }
}
impl ToSetlistDbId for (SetlistId, &Username) {
    fn to_setlist_db_uid(self) -> SetlistDbId {
        let mut hasher = DefaultHasher::new();
        (self.0, self.1.to_string()).hash(&mut hasher);

        hasher.finish() as i64
    }
}
impl ToSetlistDbId for (SetlistId, Username) {
    fn to_setlist_db_uid(self) -> SetlistDbId {
        let mut hasher = DefaultHasher::new();
        (self.0, self.1.to_string()).hash(&mut hasher);

        hasher.finish() as i64
    }
}
