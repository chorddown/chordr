use crate::models::file_type::FileType;
use crate::models::song_id::SongIdTrait;

pub trait SongData: SongIdTrait {
    fn title(&self) -> String;

    fn file_type(&self) -> FileType;
}
