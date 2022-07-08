use crate::state::State;

#[derive(Debug)]
pub enum Navigate {
    NextSong,
    PreviousSong,
    ScrollSongViewDown,
    ScrollSongViewUp,
}

#[derive(Default, Debug)]
pub struct SongNavigator {}

impl SongNavigator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn navigate(command: Navigate, state: State) {}
}
