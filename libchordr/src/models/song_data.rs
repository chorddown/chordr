use crate::models::song_id::SongId;
use crate::models::file_type::FileType;

pub trait SongData {
    fn id(&self) -> SongId;

    fn title(&self) -> String;

    fn file_type(&self) -> FileType;
}

