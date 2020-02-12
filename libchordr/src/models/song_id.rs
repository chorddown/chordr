pub type SongId = String;

pub trait SongIdTrait {
    /// Return a unique identifier of the Song
    fn id(&self) -> SongId;
}
