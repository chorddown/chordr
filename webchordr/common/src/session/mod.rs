use libchordr::prelude::User;

pub use self::session_main_data::SessionMainData;
pub use self::session_user::SessionUser;

mod session_main_data;
mod session_user;

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    user: SessionUser,
}

impl Session {
    pub fn unauthenticated() -> Self {
        Self {
            user: SessionUser::Unauthenticated,
        }
    }

    #[deprecated(note = "Use new_with_user()")]
    pub fn with_user(user: User) -> Self {
        Self::new_with_user(user)
    }

    pub fn new_with_user(user: User) -> Self {
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
        Self::unauthenticated()
    }
}
