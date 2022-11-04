use crate::persistence_manager::CommandContext;

pub trait ContextProvider {
    /// Return the `namespace` part of the storage key
    fn namespace() -> &'static str;

    /// Return the `key` part of the storage key
    fn key() -> &'static str;

    /// Return the context instance
    fn build_context() -> CommandContext {
        CommandContext::new(Self::namespace(), Self::key())
    }
}
