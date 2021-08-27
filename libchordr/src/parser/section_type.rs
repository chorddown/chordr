use crate::tokenizer::Modifier;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SectionType {
    Chorus,
    Bridge,
    Unknown,
}

impl SectionType {}

impl From<Modifier> for SectionType {
    fn from(m: Modifier) -> Self {
        match m {
            Modifier::Chorus => SectionType::Chorus,
            Modifier::Bridge => SectionType::Bridge,
            Modifier::None => SectionType::Unknown,
        }
    }
}

impl From<&Modifier> for SectionType {
    fn from(m: &Modifier) -> Self {
        match m {
            Modifier::Chorus => SectionType::Chorus,
            Modifier::Bridge => SectionType::Bridge,
            Modifier::None => SectionType::Unknown,
        }
    }
}
