use crate::state::State;
use libchordr::prelude::*;
use std::fmt::Write as _;
use std::rc::Rc;

pub(crate) fn diff(this: &State, other: &State) -> String {
    let mut output = String::new();
    if this.catalog != other.catalog {
        let default = "No Catalog";
        let _ = write!(
            output,
            "Catalog \n  {}\n vs \n  {}\n",
            this.catalog
                .as_ref()
                .map_or(default.to_owned(), |c| c.revision()),
            other
                .catalog
                .as_ref()
                .map_or(default.to_owned(), |c| c.revision())
        );
    }
    if this.connection_status != other.connection_status {
        let _ = write!(
            output,
            "Connection status \n  {:?}\n vs \n  {:?}\n",
            this.connection_status, other.connection_status,
        );
    }
    if this.current_song_id != other.current_song_id {
        let _ = write!(
            output,
            "Current Song ID \n  {:?}\n vs \n  {:?}\n",
            this.current_song_id, other.current_song_id,
        );
    }
    if this.current_setlist != other.current_setlist {
        let default = "No Setlist";
        let describe_setlist = |s: &Rc<Setlist>| format!("{} ({} entries)", s.name(), s.len());
        let _ = write!(
            output,
            "Current Setlist \n  {:?}\n vs \n  {:?}\n",
            this.current_setlist
                .as_ref()
                .map_or(default.to_owned(), describe_setlist),
            other
                .current_setlist
                .as_ref()
                .map_or(default.to_owned(), describe_setlist),
        );
    }
    if !Rc::ptr_eq(&this.session, &other.session) {
        let _ = write!(output, "{:?}\n vs \n  {:?}\n", this.session, other.session);
    }

    if !Rc::ptr_eq(&this.song_settings, &other.song_settings) {
        let _ = write!(
            output,
            "Song Settings \n  {:?}\n vs \n  {:?}\n",
            this.song_settings, other.song_settings
        );
    }
    if this.available_version != other.available_version {
        let _ = write!(
            output,
            "App version \n  {:?}\n vs \n  {:?}\n",
            this.available_version, other.available_version
        );
    }

    output
}
