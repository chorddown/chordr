use crate::connection::ConnectionStatus;
use crate::session::Session;
use chrono::Utc;
use libchordr::models::catalog::Catalog;
use libchordr::models::setlist::Setlist;
use libchordr::prelude::{CatalogTrait, ListTrait, SongId, SongSettingsMap, User};
pub use song_info::SongInfo;
use std::rc::Rc;
use webchordr_common::errors::WebError;

mod song_info;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    catalog: Option<Rc<Catalog>>,
    connection_status: ConnectionStatus,
    current_song_id: Option<SongId>,
    current_setlist: Option<Rc<Setlist>>,
    session: Rc<Session>,
    song_settings: Rc<SongSettingsMap>,
    error: Option<WebError>,
    available_version: Option<String>,
}

#[allow(unused)]
impl State {
    pub fn new(
        catalog: Option<Catalog>,
        setlist: Option<Setlist>,
        current_song_id: Option<SongId>,
        song_settings: SongSettingsMap,
        connection_status: ConnectionStatus,
        session: Session,
        error: Option<WebError>,
        available_version: Option<String>,
    ) -> Self {
        Self {
            catalog: catalog.map(Rc::new),
            connection_status,
            current_song_id,
            current_setlist: setlist.map(Rc::new),
            session: Rc::new(session),
            song_settings: Rc::new(song_settings),
            error,
            available_version,
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

    pub fn available_version(&self) -> &Option<String> {
        &self.available_version
    }

    pub fn set_available_version(&mut self, version: String) {
        self.available_version = Some(version);
    }

    pub fn with_available_version(&self, version: String) -> Self {
        let mut clone = self.clone();
        clone.set_available_version(version);

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

    pub fn current_song_id(&self) -> Option<&SongId> {
        self.current_song_id.as_ref()
    }

    pub fn with_current_song_id(&self, song_id: SongId) -> Self {
        let mut clone = self.clone();
        clone.current_song_id = Some(song_id);

        clone
    }

    pub fn without_current_song_id(&self) -> Self {
        let mut clone = self.clone();
        clone.current_song_id = None;

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

    pub(crate) fn diff(&self, other: &Self) -> String {
        let mut output = String::new();
        if self.catalog != other.catalog {
            let default = "No Catalog";
            output.push_str(&format!(
                "Catalog \n  {}\n vs \n  {}\n",
                self.catalog
                    .as_ref()
                    .map_or(default.to_owned(), |c| c.revision()),
                other
                    .catalog
                    .as_ref()
                    .map_or(default.to_owned(), |c| c.revision())
            ));
        }
        if self.connection_status != other.connection_status {
            let default = "No Catalog";
            output.push_str(&format!(
                "Connection status \n  {:?}\n vs \n  {:?}\n",
                self.connection_status, other.connection_status,
            ));
        }
        if self.current_song_id != other.current_song_id {
            output.push_str(&format!(
                "Current Song ID \n  {:?}\n vs \n  {:?}\n",
                self.current_song_id, other.current_song_id,
            ));
        }
        if self.current_setlist != other.current_setlist {
            let default = "No Setlist";
            let describe_setlist = |s: &Rc<Setlist>| format!("{} ({} entries)", s.name(), s.len());
            output.push_str(&format!(
                "Current Setlist \n  {:?}\n vs \n  {:?}\n",
                self.current_setlist
                    .as_ref()
                    .map_or(default.to_owned(), describe_setlist),
                other
                    .current_setlist
                    .as_ref()
                    .map_or(default.to_owned(), describe_setlist),
            ));
        }
        if !Rc::ptr_eq(&self.session, &other.session) {
            output.push_str(&format!(
                "{:?}\n vs \n  {:?}\n",
                self.session, other.session
            ));
        }

        if !Rc::ptr_eq(&self.song_settings, &other.song_settings) {
            output.push_str(&format!(
                "Song Settings \n  {:?}\n vs \n  {:?}\n",
                self.song_settings, other.song_settings
            ));
        }
        if self.available_version != other.available_version {
            output.push_str(&format!(
                "App version \n  {:?}\n vs \n  {:?}\n",
                self.available_version, other.available_version
            ));
        }

        output
    }
}

impl Default for State {
    fn default() -> Self {
        let user = User::unknown();
        let now = Utc::now();

        Self::new(
            None,
            Some(Setlist::new("", 0, user, None, Some(now), now, now, vec![])),
            None,
            SongSettingsMap::new(),
            ConnectionStatus::OnLine,
            Session::default(),
            None,
            None,
        )
    }
}
