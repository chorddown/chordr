pub use crate::metadata::iterator::iter::MetadataIterator;
use crate::metadata::keyword::MetadataKeyword;
use crate::models::metadata::Metadata;
use crate::models::song_meta_trait::MetadataTrait;
use crate::models::song_metadata::SongMetadata;

pub use self::iter_item::MetadataIterItem;

mod into_iter;
mod iter;
mod iter_item;
mod iter_item_value;

const FIELDS_LEN: usize = 18;
const FIELDS: [MetadataKeyword; 18] = [
    MetadataKeyword::Title,
    MetadataKeyword::Subtitle,
    MetadataKeyword::Artist,
    MetadataKeyword::Composer,
    MetadataKeyword::Lyricist,
    MetadataKeyword::Copyright,
    MetadataKeyword::Album,
    MetadataKeyword::Year,
    MetadataKeyword::Key,
    MetadataKeyword::OriginalKey,
    MetadataKeyword::Time,
    MetadataKeyword::Tempo,
    MetadataKeyword::Duration,
    MetadataKeyword::Capo,
    MetadataKeyword::OriginalTitle,
    MetadataKeyword::AlternativeTitle,
    MetadataKeyword::CCLISongId,
    MetadataKeyword::BNotation,
];

fn call_field_method<T: MetadataTrait>(metadata: &T, keyword: MetadataKeyword) -> MetadataIterItem {
    match keyword {
        MetadataKeyword::Title => (keyword, metadata.title()).into(),
        MetadataKeyword::Subtitle => (keyword, metadata.subtitle()).into(),
        MetadataKeyword::Artist => (keyword, metadata.artist()).into(),
        MetadataKeyword::Composer => (keyword, metadata.composer()).into(),
        MetadataKeyword::Lyricist => (keyword, metadata.lyricist()).into(),
        MetadataKeyword::Copyright => (keyword, metadata.copyright()).into(),
        MetadataKeyword::Album => (keyword, metadata.album()).into(),
        MetadataKeyword::Year => (keyword, metadata.year()).into(),
        MetadataKeyword::Key => (keyword, metadata.key()).into(),
        MetadataKeyword::OriginalKey => (keyword, metadata.original_key()).into(),
        MetadataKeyword::Time => (keyword, metadata.time()).into(),
        MetadataKeyword::Tempo => (keyword, metadata.tempo()).into(),
        MetadataKeyword::Duration => (keyword, metadata.duration()).into(),
        MetadataKeyword::Capo => (keyword, metadata.capo()).into(),
        MetadataKeyword::OriginalTitle => (keyword, metadata.original_title()).into(),
        MetadataKeyword::AlternativeTitle => (keyword, metadata.alternative_title()).into(),
        MetadataKeyword::CCLISongId => (keyword, metadata.ccli_song_id()).into(),
        MetadataKeyword::BNotation => (keyword, metadata.b_notation()).into(),
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
    use crate::metadata::iterator::iter_item_value::MetadataIterItemValue;
    use crate::models::chord::Note;
    use crate::models::metadata::BNotation;
    use crate::models::metadata::Metadata;

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
            MetadataIterItemValue::String("Great song".to_string()), // Title
            MetadataIterItemValue::String("Not great".to_string()),  // Subtitle
            MetadataIterItemValue::String("Me".to_string()),         // Artist
            MetadataIterItemValue::String("I".to_string()),          // Composer
            MetadataIterItemValue::String("Myself".to_string()),     // Lyricist
            MetadataIterItemValue::String("Somebody".to_string()),   // Copyright
            MetadataIterItemValue::String("ME".to_string()),         // Album
            MetadataIterItemValue::String("2021".to_string()),       // Year
            MetadataIterItemValue::Chord(Note::A.into()),            // Key
            MetadataIterItemValue::Chord(Note::D.into()),            // Original Key
            MetadataIterItemValue::None,                             // Time
            MetadataIterItemValue::None,                             // Tempo
            MetadataIterItemValue::None,                             // Duration
            MetadataIterItemValue::None,                             // Capo
            MetadataIterItemValue::None,                             // Original Title
            MetadataIterItemValue::None,                             // Alternative Title
            MetadataIterItemValue::None,                             // CCLI Song ID
            MetadataIterItemValue::BNotation(BNotation::H),          // B-Notation
        ];
        for (actual, expected) in actual.zip(expected.clone().into_iter()) {
            assert_eq!(actual.value, expected);
        }

        // `metadata` was only borrowed above. Here we consume it using `.into_iter()`
        for (actual, expected) in metadata.into_iter().zip(expected.into_iter()) {
            assert_eq!(actual.value, expected);
        }
    }
}
