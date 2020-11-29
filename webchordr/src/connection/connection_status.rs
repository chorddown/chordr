#[derive(PartialEq, Clone)]
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
