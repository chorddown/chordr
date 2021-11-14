use std::rc::Rc;

use chrono::Utc;

use libchordr::models::catalog::Catalog;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{SongSettingsMap, User};
pub use song_info::SongInfo;
use webchordr_common::errors::WebError;

use crate::connection::ConnectionStatus;
use crate::session::Session;

mod song_info;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    catalog: Option<Rc<Catalog>>,
    connection_status: ConnectionStatus,
    current_setlist: Option<Rc<Setlist>>,
    session: Rc<Session>,
    song_settings: Rc<SongSettingsMap>,
    error: Option<WebError>,
}

#[allow(unused)]
impl State {
    pub fn new(
        catalog: Option<Catalog>,
        setlist: Option<Setlist>,
        song_settings: SongSettingsMap,
        connection_status: ConnectionStatus,
        session: Session,
        error: Option<WebError>,
    ) -> Self {
        Self {
            catalog: catalog.map(Rc::new),
            connection_status,
            current_setlist: setlist.map(Rc::new),
            session: Rc::new(session),
            song_settings: Rc::new(song_settings),
            error,
        }
    }

    pub fn catalog(&self) -> Option<Rc<Catalog>> {
        self.catalog.clone()
    }

    pub fn set_catalog(&mut self, catalog: Option<Catalog>) {
        self.catalog = catalog.map(Rc::new)
    }

    pub fn with_catalog(&self, catalog: Option<Catalog>) -> Self {
        let mut clone = self.clone();
        clone.set_catalog(catalog);

        clone
    }

    pub fn error(&self) -> Option<WebError> {
        self.error.clone()
    }

    pub fn set_error(&mut self, error: Option<WebError>) {
        self.error = error
    }

    pub fn with_error(&self, error: Option<WebError>) -> Self {
        let mut clone = self.clone();
        clone.set_error(error);

        clone
    }

    pub fn connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }

    pub fn set_connection_status(&mut self, connection_status: ConnectionStatus) {
        self.connection_status = connection_status
    }

    pub fn with_connection_status(&self, connection_status: ConnectionStatus) -> Self {
        let mut clone = self.clone();
        clone.set_connection_status(connection_status);

        clone
    }

    pub fn current_setlist(&self) -> Option<Rc<Setlist>> {
        self.current_setlist.clone()
    }

    pub fn set_current_setlist(&mut self, setlist: Setlist) {
        self.current_setlist = Some(Rc::new(setlist))
    }

    pub fn with_current_setlist(&self, setlist: Setlist) -> Self {
        let mut clone = self.clone();
        clone.set_current_setlist(setlist);

        clone
    }

    pub fn session(&self) -> Rc<Session> {
        self.session.clone()
    }

    pub fn set_session(&mut self, session: Session) {
        self.session = Rc::new(session)
    }

    pub fn with_session(&self, session: Session) -> Self {
        let mut clone = self.clone();
        clone.set_session(session);

        clone
    }

    pub fn song_settings(&self) -> Rc<SongSettingsMap> {
        self.song_settings.clone()
    }

    pub fn set_song_settings(&mut self, song_settings_map: SongSettingsMap) {
        self.song_settings = Rc::new(song_settings_map)
    }

    pub fn with_song_settings(&self, song_settings_map: SongSettingsMap) -> Self {
        let mut clone = self.clone();
        clone.set_song_settings(song_settings_map);

        clone
    }
}

impl Default for State {
    fn default() -> Self {
        let user = User::unknown();
        let now = Utc::now();

        Self::new(
            None,
            Some(Setlist::new("", 0, user, None, Some(now), now, now, vec![])),
            SongSettingsMap::new(),
            ConnectionStatus::OnLine,
            Session::default(),
            None,
        )
    }
}
