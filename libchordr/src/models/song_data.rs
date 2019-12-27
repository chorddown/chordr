use crate::models::file_type::FileType;
use crate::models::song_id::SongId;

pub trait SongData {
    fn id(&self) -> SongId;

    fn title(&self) -> String;

    fn file_type(&self) -> FileType;
}
