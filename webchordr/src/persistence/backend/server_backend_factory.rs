use crate::persistence::backend::ServerBackend;
use libchordr::prelude::{Credentials, Password, Username};

pub struct ServerBackendFactory {}

impl ServerBackendFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> ServerBackend {
        ServerBackend::new(
            "http://localhost:9000",
            Some(Credentials::new(
                Username::new("daniel").unwrap(),
                Password::new("passwordhash").unwrap(),
            )),
        )
    }
}
