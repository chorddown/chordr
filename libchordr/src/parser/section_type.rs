use crate::tokenizer::Modifier;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SectionType {
    Chorus,
    Verse,
}

impl SectionType {}

impl From<Modifier> for SectionType {
    fn from(m: Modifier) -> Self {
        match m {
            Modifier::Chorus => SectionType::Chorus,
            Modifier::None => SectionType::Verse,
        }
    }
}
