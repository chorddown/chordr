use super::section_identifier::SectionIdentifier;
use super::section_type::SectionType;

#[derive(Debug, PartialEq, Clone)]
pub struct Section {
    pub section_type: SectionType,
    pub title: String,
    pub identifier: SectionIdentifier,
    pub has_content: bool,
    pub is_reference: bool,
}
