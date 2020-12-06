mod session_service;
mod session_user;

pub use self::session_service::SessionService;
pub use self::session_user::SessionUser;
use libchordr::prelude::User;

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    user: SessionUser,
}

impl Session {
    pub fn with_user(user: User) -> Self {
        Self {
            user: SessionUser::LoggedIn(user),
        }
    }

    pub fn user(&self) -> &SessionUser {
        &self.user
    }

    pub fn is_authenticated(&self) -> bool {
        match self.user {
            SessionUser::LoggedIn(_) => true,
            SessionUser::Unauthenticated => false,
        }
    }

    pub fn is_unauthenticated(&self) -> bool {
        !self.is_authenticated()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            user: SessionUser::Unauthenticated,
        }
    }
}
