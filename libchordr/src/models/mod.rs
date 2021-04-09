pub mod catalog;
pub mod chord;
pub mod file_type;
pub mod list;
pub mod meta;
pub mod prelude;
pub mod record_trait;
pub mod record_id_trait;
pub mod setlist;
pub mod song;
pub mod song_data;
pub mod song_id;
pub mod song_list;
pub mod song_settings;
pub mod team;
pub mod user;

#[deprecated(note = "Please use meta::meta_trait instead")]
pub use self::meta::meta_trait as song_meta_trait;

pub mod song_meta;
