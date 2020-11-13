use libchordr::prelude::User;

#[derive(Debug, Clone, PartialEq)]
pub enum SessionUser {
    Unauthenticated,
    LoggedIn(User),
}
