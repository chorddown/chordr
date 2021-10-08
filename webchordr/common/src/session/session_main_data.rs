use crate::session::Session;
use libchordr::models::user::MainData;

#[derive(Debug)]
pub struct SessionMainData {
    pub session: Session,
    pub main_data: MainData,
}
