use crate::tokenizer::Modifier;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SectionType {
    Verse,
    Chorus,
    Bridge,
}

impl From<Modifier> for SectionType {
    fn from(m: Modifier) -> Self {
        match m {
            Modifier::Chorus => SectionType::Chorus,
            Modifier::Bridge => SectionType::Bridge,
            Modifier::None => SectionType::Verse,
        }
    }
}

impl From<&Modifier> for SectionType {
    fn from(m: &Modifier) -> Self {
        match m {
            Modifier::Chorus => SectionType::Chorus,
            Modifier::Bridge => SectionType::Bridge,
            Modifier::None => SectionType::Verse,
        }
    }
}
