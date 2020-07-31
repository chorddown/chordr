pub mod setlist_events;
pub mod settings_events;
pub mod sorting_change;

pub use self::setlist_events::SetlistEvent;
pub use self::settings_events::SettingsEvent;
pub use self::sorting_change::SortingChange;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub const SETLIST_CHANGE_SORTING: &str = "chordr:setlist-change-sorting";

pub trait EventTrait {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    /// Sorting of entries in the Setlist changed
    SortingChange(SortingChange),

    /// Events related to [`SongSettings`]
    SettingsEvent(SettingsEvent),

    /// Events related to [`Setlist`s]
    SetlistEvent(SetlistEvent),

    /// A pair of events triggered at once
    Pair(Box<Event>, Box<Event>),
}

impl From<SortingChange> for Event {
    fn from(s: SortingChange) -> Self {
        Event::SortingChange(s)
    }
}

impl From<SetlistEvent> for Event {
    fn from(s: SetlistEvent) -> Self {
        Event::SetlistEvent(s)
    }
}

impl From<SettingsEvent> for Event {
    fn from(s: SettingsEvent) -> Self {
        Event::SettingsEvent(s)
    }
}
