use crate::connection::ConnectionStatus;
use crate::session::Session;
use libchordr::models::catalog::Catalog;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::SongSettingsMap;

#[allow(unused)]
pub struct State {
    catalog: Option<Catalog>,
    connection_status: ConnectionStatus,
    setlist: Setlist,
    session: Session,
    song_settings: SongSettingsMap,
}

impl State {
    pub fn new(
        catalog: Option<Catalog>,
        setlist: Setlist,
        song_settings: SongSettingsMap,
        connection_status: ConnectionStatus,
        session: Session,
    ) -> Self {
        Self {
            catalog,
            connection_status,
            setlist,
            session,
            song_settings,
        }
    }
}
