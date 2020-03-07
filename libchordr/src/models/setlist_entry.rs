use crate::models::prelude::SongData;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct SetlistEntry<S: SongData> {
    song_data: S,
    song_settings: SongSettings,
}
