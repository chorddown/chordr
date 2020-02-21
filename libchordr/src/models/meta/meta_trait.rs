use super::b_notation::BNotation;
use std::fmt::Debug;

pub trait MetaTrait: Debug {
    fn title(&self) -> Option<String>;
    fn subtitle(&self) -> Option<String>;
    fn artist(&self) -> Option<String>;
    fn composer(&self) -> Option<String>;
    fn lyricist(&self) -> Option<String>;
    fn copyright(&self) -> Option<String>;
    fn album(&self) -> Option<String>;
    fn year(&self) -> Option<String>;
    fn key(&self) -> Option<String>;
    fn time(&self) -> Option<String>;
    fn tempo(&self) -> Option<String>;
    fn duration(&self) -> Option<String>;
    fn capo(&self) -> Option<String>;
    fn b_notation(&self) -> BNotation;
}

#[deprecated(note = "Please use meta::MetaTrait instead")]
pub use MetaTrait as SongMetaTrait;
