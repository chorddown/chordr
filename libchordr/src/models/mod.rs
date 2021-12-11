#[deprecated(note = "Please use crate::metadata::metadata_trait instead")]
pub use crate::metadata::metadata_trait as song_meta_trait;

pub mod catalog;
pub mod chord;
pub mod file_type;
pub mod list;
pub mod metadata;
pub mod prelude;
pub mod record_id_trait;
pub mod record_trait;
pub mod setlist;
pub mod song;
pub mod song_data;
pub mod song_id;
pub mod song_list;
pub mod song_metadata;
pub mod song_settings;
pub mod song_sorting;
pub mod team;
pub mod user;
