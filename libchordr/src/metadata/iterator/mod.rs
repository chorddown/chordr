use crate::metadata::iterator::iter::MetadataIterator;
use crate::metadata::keyword;
use crate::models::metadata::Metadata;
use crate::models::song_meta_trait::MetadataTrait;
use crate::models::song_metadata::SongMetadata;

pub use self::iter_item::MetadataIterItem;

mod into_iter;
mod iter;
mod iter_item;

const FIELDS_LEN: usize = 18;
const FIELDS: [&str; 18] = [
    keyword::TITLE,
    keyword::SUBTITLE,
    keyword::ARTIST,
    keyword::COMPOSER,
    keyword::LYRICIST,
    keyword::COPYRIGHT,
    keyword::ALBUM,
    keyword::YEAR,
    keyword::KEY,
    keyword::ORIGINAL_KEY,
    keyword::TIME,
    keyword::TEMPO,
    keyword::DURATION,
    keyword::CAPO,
    keyword::ORIGINAL_TITLE,
    keyword::ALTERNATIVE_TITLE,
    keyword::CCLI_SONG_ID,
    keyword::B_NOTATION,
];

fn call_field_method<T: MetadataTrait>(metadata: &T, keyword: &str) -> MetadataIterItem {
    match keyword {
        keyword::TITLE => metadata.title().into(),
        keyword::SUBTITLE => metadata.subtitle().into(),
        keyword::ARTIST => metadata.artist().into(),
        keyword::COMPOSER => metadata.composer().into(),
        keyword::LYRICIST => metadata.lyricist().into(),
        keyword::COPYRIGHT => metadata.copyright().into(),
        keyword::ALBUM => metadata.album().into(),
        keyword::YEAR => metadata.year().into(),
        keyword::KEY => metadata.key().into(),
        keyword::ORIGINAL_KEY => metadata.original_key().into(),
        keyword::TIME => metadata.time().into(),
        keyword::TEMPO => metadata.tempo().into(),
        keyword::DURATION => metadata.duration().into(),
        keyword::CAPO => metadata.capo().into(),
        keyword::ORIGINAL_TITLE => metadata.original_title().into(),
        keyword::ALTERNATIVE_TITLE => metadata.alternative_title().into(),
        keyword::CCLI_SONG_ID => metadata.ccli_song_id().into(),
        keyword::B_NOTATION => metadata.b_notation().into(),
        &_ => unreachable!(),
    }
}

impl Metadata {
    pub fn iter(&self) -> MetadataIterator<Metadata> {
        self.into_iter()
    }
}
impl SongMetadata {
    pub fn iter(&self) -> MetadataIterator<SongMetadata> {
        self.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::chord::Note;
    use crate::models::metadata::BNotation;
    use crate::models::metadata::Metadata;

    use super::*;

    #[test]
    fn iterate() {
        let metadata = Metadata {
            title: Some("Great song".to_string()),
            subtitle: Some("Not great".to_string()),
            artist: Some("Me".to_string()),
            composer: Some("I".to_string()),
            lyricist: Some("Myself".to_string()),
            copyright: Some("Somebody".to_string()),
            album: Some("ME".to_string()),
            year: Some("2021".to_string()),
            key: Some(Note::A.into()),
            original_key: Some(Note::D.into()),
            time: None,
            tempo: None,
            duration: None,
            capo: None,
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: BNotation::H,
        };
        let actual = metadata.iter();
        let expected = vec![
            MetadataIterItem::String("Great song".to_string()), // Title
            MetadataIterItem::String("Not great".to_string()),  // Subtitle
            MetadataIterItem::String("Me".to_string()),         // Artist
            MetadataIterItem::String("I".to_string()),          // Composer
            MetadataIterItem::String("Myself".to_string()),     // Lyricist
            MetadataIterItem::String("Somebody".to_string()),   // Copyright
            MetadataIterItem::String("ME".to_string()),         // Album
            MetadataIterItem::String("2021".to_string()),       // Year
            MetadataIterItem::Chord(Note::A.into()),            // Key
            MetadataIterItem::Chord(Note::D.into()),            // Original Key
            MetadataIterItem::None,                             // Time
            MetadataIterItem::None,                             // Tempo
            MetadataIterItem::None,                             // Duration
            MetadataIterItem::None,                             // Capo
            MetadataIterItem::None,                             // Original Title
            MetadataIterItem::None,                             // Alternative Title
            MetadataIterItem::None,                             // CCLI Song ID
            MetadataIterItem::BNotation(BNotation::H),          // B-Notation
        ];
        for (actual, expected) in actual.zip(expected.clone().into_iter()) {
            assert_eq!(actual, expected);
        }

        // `metadata` was only borrowed above. Here we consume it using `.into_iter()`
        for (actual, expected) in metadata.into_iter().zip(expected.into_iter()) {
            assert_eq!(actual, expected);
        }
    }
}
