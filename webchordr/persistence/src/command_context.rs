use webchordr_common::constants::{STORAGE_NAMESPACE, TEST_STORAGE_NAMESPACE};

#[derive(Clone, Debug)]
pub struct CommandContext {
    pub namespace: String,
    pub key: String,
}

impl CommandContext {
    pub(crate) fn new<S1: Into<String>, S2: Into<String>>(namespace: S1, key: S2) -> Self {
        let namespace = namespace.into();
        if namespace != STORAGE_NAMESPACE && namespace != TEST_STORAGE_NAMESPACE {
            panic!("No server backend found for namespace: '{}'", namespace)
        }

        Self {
            namespace,
            key: key.into(),
        }
    }
}
