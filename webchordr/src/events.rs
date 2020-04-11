pub mod sorting_change;
pub mod setlist_events;

use serde::{Deserialize, Serialize};
pub use self::sorting_change::SortingChange;
pub use self::setlist_events::SetlistEvent;

pub const SETLIST_CHANGE_SORTING: &str = "chordr:setlist-change-sorting";

pub trait EventTrait {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    SortingChange(SortingChange),
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
