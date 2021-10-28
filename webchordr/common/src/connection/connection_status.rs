#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectionStatus {
    OnLine,
    ServerNotReachable,
    Offline,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Offline
    }
}
