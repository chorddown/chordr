mod deserialize;
mod serialize;

pub use crate::models::setlist::sharing_setlist::SharingSetlist;
pub use deserialize::DeserializeService;
use serde::{Deserialize, Serialize};
pub use serialize::SerializeService;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct SharingSetlistWithVersion {
    #[serde(rename = "v")]
    version: String,
    #[serde(flatten)]
    setlist: SharingSetlist,
}

#[cfg(test)]
mod test {
    use crate::prelude::{FileType, Formatting, Setlist, SetlistEntry, SongId, SongSettings};
    use crate::test_helpers::get_test_user;
    use chrono::{DateTime, Utc};

    pub fn build_test_setlist_entry<S: Into<SongId>>(
        id: S,
        settings: Option<SongSettings>,
    ) -> SetlistEntry {
        let song_id = id.into();
        let title = format!("Song {}", song_id);

        SetlistEntry::new(song_id, FileType::Chorddown, title, settings)
    }

    pub fn build_test_setlist() -> Setlist {
        let test_date = DateTime::parse_from_rfc2822("Fri, 20 May 2022 16:04:37 +0200")
            .unwrap()
            .with_timezone(&Utc);

        Setlist::new(
            "Setlist name",
            1,
            get_test_user(),
            None,
            Some(test_date.clone()),
            test_date.clone(),
            test_date.clone(),
            vec![
                build_test_setlist_entry("my-first-song.chorddown", None),
                build_test_setlist_entry("another-song.chorddown", None),
                build_test_setlist_entry("best-one.chorddown", None),
                build_test_setlist_entry(
                    "nice-track.chorddown",
                    Some(SongSettings::new(-2, Formatting::default(), "Some notes")),
                ),
                build_test_setlist_entry("last-song.chorddown", None),
            ],
        )
    }

    pub fn build_test_setlist_string() -> &'static str {
        "v=2\
        &name=Setlist+name\
        &id=1\
        &songs[0][id]=my-first-song.chorddown\
        &songs[1][id]=another-song.chorddown\
        &songs[2][id]=best-one.chorddown\
        &songs[3][id]=nice-track.chorddown\
        &songs[3][tr]=-2\
        &songs[3][nt]=Some+notes\
        &songs[4][id]=last-song.chorddown\
        &gig_date=2022-05-20T14%3A04%3A37Z\
        &creation_date=2022-05-20T14%3A04%3A37Z\
        &modification_date=2022-05-20T14%3A04%3A37Z"
    }
}
