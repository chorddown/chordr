use libchordr::prelude::{SongId, SongSettings, SongSettingsMap};
use webchordr_events::SettingsEvent;
use yew::{Component, Context};

pub trait SettingsHandler: Component {
    /// Handle the given [`SongSettings`] related event
    fn handle_settings_event(&mut self, event: SettingsEvent);

    /// Invoked when the [`SongSettings`] for the Song with the given [`SongId`] changed
    fn song_settings_change(&mut self, song_id: SongId, settings: SongSettings);

    /// Replace the locally stored collection of [`SongSettings`]
    fn song_settings_replace(&mut self, settings: SongSettingsMap);

    /// Load the [`SongSettings`] from the persistent storage
    fn fetch_song_settings(&mut self, ctx: &Context<Self>);

    /// Commit the [`SongSettings`] to the persistent storage
    fn commit_changes(&mut self);
}
