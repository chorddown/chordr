use crate::errors::{SharingError, WebError};
use libchordr::data_exchange::setlist::SerializeService as LibChordrSerializeService;
use libchordr::data_exchange::DxSerializer;
use libchordr::models::setlist::Setlist;

pub struct SerializeService {}

impl SerializeService {
    pub fn serialize(setlist: &Setlist) -> Result<String, WebError> {
        Ok(LibChordrSerializeService::serialize(setlist).map_err(|e| SharingError::from(e))?)
    }
}
