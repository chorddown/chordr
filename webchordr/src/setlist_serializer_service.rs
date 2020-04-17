use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{SetlistEntry, SongIdTrait};

pub struct SetlistSerializerService {}

impl SetlistSerializerService {
    pub fn build_share_url(setlist: &Setlist<SetlistEntry>) -> Result<String, WebError> {
        Ok(setlist
            .iter()
            .map(|song| song.id().to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
}
