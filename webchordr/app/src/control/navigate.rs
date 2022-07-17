use crate::state::State;
use libchordr::prelude::{ListEntryTrait, ListTrait};
use web_sys::ScrollToOptions;
use webchordr_common::helpers::window;
use webchordr_common::route::AppRoute;
use yew_router::prelude::{BrowserHistory, History};

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
        match self.get_route(command, state) {
            Some(route) => {
                BrowserHistory::default().push(route.clone());

                window().scroll_to_with_scroll_to_options(ScrollToOptions::new().top(0.0));

                Some(route)
            }
            None => None,
        }
    }

    pub fn get_route(&self, command: Navigate, state: &State) -> Option<AppRoute> {
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
                id: setlist[index + 1].id().into(),
            })
        } else {
            None
        }
    }

    fn previous_song(&self, state: &State) -> Option<AppRoute> {
        let index = self.find_song_index(state)?;
        if index > 0 {
            Some(AppRoute::Song {
                id: state.current_setlist()?[index - 1].id().into(),
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
