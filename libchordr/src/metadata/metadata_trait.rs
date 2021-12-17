use crate::models::chord::Chord;
use crate::models::metadata::BNotation;

pub trait MetadataTrait {
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

// impl<T: MetadataTrait + ?Sized> MetadataTrait for Box<T> {
//     fn title(&self) -> Option<&str> {
//         (**self).title()
//     }
//
//     fn subtitle(&self) -> Option<&str> {
//         (**self).subtitle()
//     }
//
//     fn artist(&self) -> Option<&str> {
//         (**self).artist()
//     }
//
//     fn composer(&self) -> Option<&str> {
//         (**self).composer()
//     }
//
//     fn lyricist(&self) -> Option<&str> {
//         (**self).lyricist()
//     }
//
//     fn copyright(&self) -> Option<&str> {
//         (**self).copyright()
//     }
//
//     fn album(&self) -> Option<&str> {
//         (**self).album()
//     }
//
//     fn year(&self) -> Option<&str> {
//         (**self).year()
//     }
//
//     fn key(&self) -> Option<&Chord> {
//         (**self).key()
//     }
//
//     fn original_key(&self) -> Option<&Chord> {
//         (**self).original_key()
//     }
//
//     fn time(&self) -> Option<&str> {
//         (**self).time()
//     }
//
//     fn tempo(&self) -> Option<&str> {
//         (**self).tempo()
//     }
//
//     fn duration(&self) -> Option<&str> {
//         (**self).duration()
//     }
//
//     fn capo(&self) -> Option<&str> {
//         (**self).capo()
//     }
//
//     fn original_title(&self) -> Option<&str> {
//         (**self).original_title()
//     }
//
//     fn alternative_title(&self) -> Option<&str> {
//         (**self).alternative_title()
//     }
//
//     fn ccli_song_id(&self) -> Option<&str> {
//         (**self).ccli_song_id()
//     }
//
//     fn b_notation(&self) -> BNotation {
//         (**self).b_notation()
//     }
// }
