use log::warn;

use crate::models::structure::{Section, SectionIdentifier};
use crate::parser::Node;
use crate::tokenizer::Token;

pub(crate) struct SectionProvider {}

#[derive(Debug)]
struct Info {
    level: Option<u8>,
    text: String,
}

impl SectionProvider {
    /// Extract the `Section`s of the given Document node
    pub fn get_sections(node: &Node) -> Vec<Section> {
        let mut sections = vec![];
        if let Node::Document(children) = node {
            for child_node in children {
                if let Some(s) = Self::get_section(child_node) {
                    sections.push(s)
                }
            }
        } else {
            warn!("Excepted node to be a Document node")
        }

        sections
    }

    /// Extract the `Section` of the given Section node
    pub fn get_section(node: &Node) -> Option<Section> {
        if let Node::Section {
            head,
            section_type,
            children,
        } = node
        {
            if let Some(info) = Self::extract_headline(head) {
                if info.level.unwrap_or(10) > 1 {
                    let identifier = SectionIdentifier::try_from(head.as_ref()).unwrap();

                    return Some(Section {
                        section_type: *section_type,
                        title: info.text,
                        has_content: !children.is_empty(),
                        is_reference: matches!(**head, Node::Quote(_)),
                        identifier,
                    });
                }
            }
        }

        None
    }

    fn extract_headline(head: &Node) -> Option<Info> {
        match head {
            Node::Headline(Token::Headline {
                level,
                text,
                modifier: _,
            }) => Some(Info {
                level: Some(*level),
                text: text.clone(),
            }),
            Node::Quote(Token::Quote(text)) => Some(Info {
                level: None,
                text: text.clone(),
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::SectionType;
    use crate::test_helpers::{get_test_ast, get_test_ast_small};

    #[test]
    fn get_sections_from_small_ast_test() {
        let input = get_test_ast_small();
        let sections = input.get_sections();

        assert_eq!(2, sections.len());

        assert_eq!(SectionType::Chorus, sections[0].section_type);
        assert_eq!("Chorus", &sections[0].title);
        assert_eq!(SectionType::Verse, sections[1].section_type);
        assert_eq!("Verse", &sections[1].title);
    }

    #[test]
    fn get_sections_from_ast_test() {
        let input = get_test_ast();
        let sections = input.get_sections();

        assert_eq!(3, sections.len());

        assert_eq!(SectionType::Chorus, sections[0].section_type);
        assert_eq!("Chorus", &sections[0].title);
        assert_eq!(SectionType::Verse, sections[1].section_type);
        assert_eq!("Verse 1", &sections[1].title);
        assert_eq!(SectionType::Reference, sections[2].section_type);
        assert_eq!("Chorus", &sections[2].title);
    }
}
