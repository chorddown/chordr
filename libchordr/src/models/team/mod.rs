mod team_id;

pub use self::team_id::TeamId;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    id: TeamId,
    name: String,
    users: Vec<User>,
}

impl Team {
    pub fn new<I: Into<TeamId>, N: Into<String>>(id: I, name: N, users: Vec<User>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            users,
        }
    }

    pub fn id(&self) -> &TeamId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn users(&self) -> &Vec<User> {
        &self.users
    }
}
