use crate::state::{SongInfo, State};
use libchordr::prelude::*;
use log::debug;
use std::rc::Rc;

pub struct SongInfoService {}

impl SongInfoService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_song_info_from_state(&self, song_id: &SongId, state: &State) -> Option<SongInfo> {
        let catalog = state.catalog()?;

        self.get_song_info(
            song_id,
            &catalog,
            &state.current_setlist(),
            &state.song_settings(),
        )
    }

    pub fn get_song_info(
        &self,
        song_id: &SongId,
        catalog: &Catalog,
        current_setlist: &Option<Rc<Setlist>>,
        song_settings: &SongSettingsMap,
    ) -> Option<SongInfo> {
        let is_on_setlist = if let Some(ref setlist) = current_setlist {
            setlist.contains_id(song_id.clone())
        } else {
            false
        };

        Some(SongInfo {
            song: self.get_song(song_id, catalog)?,
            song_settings: self.get_settings_for_song(song_id, current_setlist, song_settings),
            is_on_setlist,
        })
    }

    fn get_song(&self, song_id: &SongId, catalog: &Catalog) -> Option<Song> {
        catalog.get(song_id).cloned()
    }

    fn get_settings_for_song(
        &self,
        song_id: &SongId,
        current_setlist: &Option<Rc<Setlist>>,
        song_settings: &SongSettingsMap,
    ) -> SongSettings {
        // Look if there are settings for the `SongId` in the `Setlist`
        if let Some(settings) = self.get_settings_from_setlist(song_id, current_setlist) {
            debug!(
                "Found settings for song in Setlist {}: {:?}",
                song_id, settings
            );

            return settings;
        }

        match song_settings.get(song_id) {
            Some(s) => {
                debug!("Found settings for song {}: {:?}", song_id, s);
                s.clone()
            }
            None => {
                debug!("No settings for song {} found in setlist", song_id);
                SongSettings::default()
            }
        }
    }

    fn get_settings_from_setlist(
        &self,
        song_id: &SongId,
        current_setlist: &Option<Rc<Setlist>>,
    ) -> Option<SongSettings> {
        match current_setlist {
            None => None,
            Some(setlist) => setlist
                .get(song_id.clone())
                .and_then(|entry| entry.settings()),
        }
    }
}
