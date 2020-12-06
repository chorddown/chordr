use crate::models::setlist::Setlist;
use crate::models::user::User;
use crate::prelude::SongSettingsMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MainData {
    pub user: User,
    pub latest_setlist: Option<Setlist>,
    pub song_settings: Option<SongSettingsMap>,
}

impl MainData {
    pub fn with_user(&self, user: User) -> Self {
        let mut clone = self.clone();
        clone.user = user;

        clone
    }
}
