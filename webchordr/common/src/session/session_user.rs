use libchordr::prelude::{Credentials, User};

#[derive(Debug, Clone, PartialEq)]
pub enum SessionUser {
    Unauthenticated,
    LoggedIn(User),
}

impl SessionUser {
    #[allow(unused)]
    pub fn is_unauthenticated(&self) -> bool {
        match self {
            SessionUser::Unauthenticated => true,
            SessionUser::LoggedIn(_) => false,
        }
    }

    #[allow(unused)]
    pub fn is_logged_in(&self) -> bool {
        match self {
            SessionUser::Unauthenticated => false,
            SessionUser::LoggedIn(_) => true,
        }
    }

    #[allow(unused)]
    pub fn credentials(&self) -> Option<Credentials> {
        match self {
            SessionUser::LoggedIn(user) => Some(Credentials::from(user)),
            SessionUser::Unauthenticated => None,
        }
    }
}
