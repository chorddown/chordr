use crate::models::structure::Section;
use crate::repeat_detector::RepeatDetector;

pub(crate) struct ReferenceResolver {}

impl ReferenceResolver {
    pub(crate) fn new() -> Self {
        Self {}
    }

    /// Try to find the target Section pointed to by [reference]
    pub(crate) fn resolve_reference<'a, 'b, 'c>(
        &'a self,
        reference: &'b Section,
        sections: &'c [Section],
    ) -> Option<&'c Section> {
        if sections.is_empty() {
            return None;
        }

        let result = sections
            .into_iter()
            .find(|section| section != &reference && section.identifier == reference.identifier)
            // .cloned()
            ;
        if result.is_some() {
            return result;
        }

        let section_to_repeat = RepeatDetector::new().detect(&reference.title).ok()?;
        sections
            .into_iter()
            .find(|section| section.identifier == section_to_repeat.identifier)
        // .cloned()
    }

    // /// Search the sections for a Section matching (but not equal to) the [reference]
    // ///
    // /// In contrast to [resolve_reference] this will not check for "repeat" statements
    // fn resolve_reference_by_identifier(
    //     &self,
    //     reference: &Section,
    //     sections: &[Section],
    // ) -> Option<Section> {
    //     sections
    //         .into_iter()
    //         .find(|section| section != &reference && section.identifier == reference.identifier)
    //         .cloned()
    // }
}

impl Default for ReferenceResolver {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::models::structure::reference_resolver::ReferenceResolver;
    use crate::parser::{Node, SectionType};
    use crate::test_helpers::get_test_ast;
    use crate::tokenizer::Modifier;

    #[test]
    fn resolve_reference_test() {
        let sections = get_test_ast().get_sections();
        let resolver = ReferenceResolver::new();
        let result = resolver.resolve_reference(&sections[2], &sections).unwrap();
        assert_eq!(&sections[0], result);
    }

    #[test]
    fn resolve_reference_with_repeat_prefix_test() {
        let sections = Node::Document(vec![
            Node::section(1, "Title", Modifier::None, vec![]),
            Node::section(2, "Chorus", Modifier::Chorus, vec![]),
            Node::section(2, "Verse 1", Modifier::None, vec![]),
            Node::Section {
                head: Box::new(Node::quote("2x Chorus")),
                section_type: SectionType::Reference,
                children: vec![],
            },
            Node::newline(),
        ])
        .get_sections();

        let resolver = ReferenceResolver::new();
        let result = resolver.resolve_reference(&sections[2], &sections).unwrap();
        assert_eq!(&sections[0], result);
    }

    #[test]
    fn resolve_reference_with_repeat_prefix_multi_word_test() {
        let sections = Node::Document(vec![
            Node::section(1, "Title", Modifier::None, vec![]),
            Node::section(2, "My Chorus", Modifier::Chorus, vec![]),
            Node::section(2, "Verse 1", Modifier::None, vec![]),
            Node::Section {
                head: Box::new(Node::quote("2x My Chorus")),
                section_type: SectionType::Reference,
                children: vec![],
            },
            Node::newline(),
        ])
        .get_sections();

        let resolver = ReferenceResolver::new();
        let result = resolver.resolve_reference(&sections[2], &sections).unwrap();
        assert_eq!(&sections[0], result);
    }

    #[test]
    fn resolve_reference_with_repeat_suffix_test() {
        let sections = Node::Document(vec![
            Node::section(1, "Title", Modifier::None, vec![]),
            Node::section(2, "Chorus", Modifier::Chorus, vec![]),
            Node::section(2, "Verse 1", Modifier::None, vec![]),
            Node::Section {
                head: Box::new(Node::quote("Chorus 2x")),
                section_type: SectionType::Reference,
                children: vec![],
            },
            Node::newline(),
        ])
        .get_sections();

        let resolver = ReferenceResolver::new();
        let result = resolver.resolve_reference(&sections[2], &sections).unwrap();
        assert_eq!(&sections[0], result);
    }

    #[test]
    fn resolve_reference_with_repeat_suffix_multi_word_test() {
        let sections = Node::Document(vec![
            Node::section(1, "Title", Modifier::None, vec![]),
            Node::section(2, "My Chorus", Modifier::Chorus, vec![]),
            Node::section(2, "Verse 1", Modifier::None, vec![]),
            Node::Section {
                head: Box::new(Node::quote("My Chorus 2x")),
                section_type: SectionType::Reference,
                children: vec![],
            },
            Node::newline(),
        ])
        .get_sections();

        let resolver = ReferenceResolver::new();
        let result = resolver.resolve_reference(&sections[2], &sections).unwrap();
        assert_eq!(&sections[0], result);
    }
}
