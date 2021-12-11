use std::fmt::Debug;

use crate::models::chord::Chord;
use crate::models::metadata::BNotation;

pub trait MetadataTrait: Debug {
    fn title(&self) -> Option<&str>;
    fn subtitle(&self) -> Option<&str>;
    fn artist(&self) -> Option<&str>;
    fn composer(&self) -> Option<&str>;
    fn lyricist(&self) -> Option<&str>;
    fn copyright(&self) -> Option<&str>;
    fn album(&self) -> Option<&str>;
    fn year(&self) -> Option<&str>;
    fn key(&self) -> Option<&Chord>;
    fn original_key(&self) -> Option<&Chord>;
    fn time(&self) -> Option<&str>;
    fn tempo(&self) -> Option<&str>;
    fn duration(&self) -> Option<&str>;
    fn capo(&self) -> Option<&str>;
    fn original_title(&self) -> Option<&str>;
    fn alternative_title(&self) -> Option<&str>;
    fn ccli_song_id(&self) -> Option<&str>;
    fn b_notation(&self) -> BNotation;
}
