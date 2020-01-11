pub mod catalog;
pub mod file_type;
pub mod prelude;
pub mod song;
pub mod song_data;
pub mod song_id;
pub mod song_list;
pub mod meta;

#[deprecated(note = "Please use meta::meta_trait instead")]
pub use self::meta::meta_trait as song_meta_trait;

pub mod song_meta;
