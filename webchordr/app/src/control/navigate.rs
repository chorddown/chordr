use crate::state::State;
use libchordr::prelude::{ListEntryTrait, ListTrait};
use webchordr_common::route::AppRoute;

#[derive(Debug)]
pub enum Navigate {
    NextSong,
    PreviousSong,
    // ScrollSongViewDown,
    // ScrollSongViewUp,
}

#[derive(Default, Debug)]
pub struct SongNavigator {}

impl SongNavigator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn navigate(&self, command: Navigate, state: &State) -> Option<AppRoute> {
        match command {
            Navigate::NextSong => self.next_song(state),
            Navigate::PreviousSong => self.previous_song(state),
            // Navigate::ScrollSongViewDown => {}
            // Navigate::ScrollSongViewUp => {}
        }
    }

    fn next_song(&self, state: &State) -> Option<AppRoute> {
        let index = self.find_song_index(state)?;
        let setlist = state.current_setlist()?;
        if index + 1 < setlist.len() {
            Some(AppRoute::Song {
                id: setlist[index + 1].id(),
            })
        } else {
            None
        }
    }

    fn previous_song(&self, state: &State) -> Option<AppRoute> {
        let index = self.find_song_index(state)?;
        if index > 0 {
            Some(AppRoute::Song {
                id: state.current_setlist()?[index - 1].id(),
            })
        } else {
            None
        }
    }

    fn find_song_index(&self, state: &State) -> Option<usize> {
        let song_id = state.current_song_id()?;
        let setlist = state.current_setlist()?;

        setlist.position(song_id.clone())
    }
}
