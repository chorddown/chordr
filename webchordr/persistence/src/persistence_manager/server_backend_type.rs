use log::warn;

use webchordr_common::constants::{
    STORAGE_KEY_SETLIST, STORAGE_KEY_SETTINGS, STORAGE_NAMESPACE, STORAGE_V2_KEY_SETLIST,
    TEST_STORAGE_NAMESPACE,
};

use crate::persistence_manager::CommandContext;

pub(crate) enum ServerBackendType {
    Server,
    Transient,
}

impl ServerBackendType {
    pub(crate) fn from_context(context: &CommandContext) -> Self {
        if context.namespace != STORAGE_NAMESPACE && context.namespace != TEST_STORAGE_NAMESPACE {
            panic!(
                "No server backend found for namespace: '{}'",
                context.namespace
            )
        }

        match context.key.as_str() {
            STORAGE_KEY_SETLIST => ServerBackendType::Transient,
            STORAGE_V2_KEY_SETLIST => ServerBackendType::Server,
            STORAGE_KEY_SETTINGS => ServerBackendType::Transient,
            _ => {
                warn!("No server backend found for key: '{}'", context.key);
                ServerBackendType::Transient
            }
        }
    }
}
