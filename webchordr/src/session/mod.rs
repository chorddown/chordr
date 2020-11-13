mod session_user;
mod session_service;

use libchordr::prelude::User;
pub use self::session_user::SessionUser;
pub use self::session_service::SessionService;

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    user: SessionUser,
}

impl Session {
    pub fn with_user(user: User) -> Self {
        Self { user: SessionUser::LoggedIn(user) }
    }

    pub fn user(&self) -> &SessionUser {
        &self.user
    }
}

impl Default for Session {
    fn default() -> Self {
        Self { user: SessionUser::Unauthenticated }
    }
}
