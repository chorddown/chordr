use libchordr::prelude::SongId;
use percent_encoding::percent_decode_str;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct SongIdParam(String);

impl SongIdParam {
    pub fn as_song_id(&self) -> SongId {
        if self.0.contains('%') {
            if let Ok(decoded) = percent_decode_str(&self.0).decode_utf8() {
                return SongId::new(decoded);
            }
        }

        SongId::new(&self.0)
    }

    pub fn from_song_id(song_id: &SongId) -> Self {
        Self(song_id.as_str().to_owned())
    }
}

impl FromStr for SongIdParam {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SongIdParam(s.to_owned()))
    }
}

impl Display for SongIdParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<SongId> for SongIdParam {
    fn from(s: SongId) -> Self {
        Self::from_song_id(&s)
    }
}
