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
    SortingChange(SortingChange),
    SettingsEvent(SettingsEvent),
    SetlistEvent(SetlistEvent),
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
