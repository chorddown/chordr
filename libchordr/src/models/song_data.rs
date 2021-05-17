use crate::models::song_id::SongIdTrait;

pub trait SongData: SongIdTrait {
    fn title(&self) -> String;
}
