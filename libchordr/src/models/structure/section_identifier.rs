use crate::parser::Node;
use crate::tokenizer::Token;

use super::section_identifier_error::SectionIdentifierError;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub struct SectionIdentifier(String);

impl TryFrom<&str> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cleaned = String::with_capacity(value.len());
        for c in value.trim().chars() {
            match c {
                'Ä' => cleaned.push_str("ae"),
                'ä' => cleaned.push_str("ae"),
                'Ö' => cleaned.push_str("oe"),
                'ö' => cleaned.push_str("oe"),
                'Ü' => cleaned.push_str("ue"),
                'ü' => cleaned.push_str("ue"),
                'ß' => cleaned.push_str("ss"),
                '-' => cleaned.push('-'),
                _ if c.is_whitespace() => cleaned.push('-'),
                _ if c.is_ascii() => cleaned.push(c),
                _ => { /* noop */ }
            }
        }
        cleaned.shrink_to_fit();
        cleaned.make_ascii_lowercase();

        if cleaned.is_empty() {
            return Err(SectionIdentifierError::Empty);
        }

        Ok(SectionIdentifier(cleaned))
    }
}

// fn modifier_to_string(modifier: Modifier) -> &'static str {
//     match modifier {
//         Modifier::Chorus => "c-",
//         Modifier::Bridge => "b-",
//         Modifier::None => "",
//     }
// }

impl TryFrom<&Token> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Headline {
                text,
                level: _level,
                modifier: _modifier,
            } => SectionIdentifier::try_from(text.as_str()),
            Token::Quote(l) => SectionIdentifier::try_from(l.as_str()),
            Token::Literal(l) => SectionIdentifier::try_from(l.as_str()),
            _ => Err(SectionIdentifierError::UnsupportedInput),
        }
    }
}

impl TryFrom<Token> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        SectionIdentifier::try_from(&value)
    }
}

impl TryFrom<&Node> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        match node {
            Node::Headline(h) => SectionIdentifier::try_from(h),
            Node::Quote(h) => SectionIdentifier::try_from(h),
            _ => Err(SectionIdentifierError::UnsupportedInput),
        }
    }
}

impl TryFrom<Node> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        SectionIdentifier::try_from(&node)
    }
}

impl TryFrom<String> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SectionIdentifier::try_from(value.as_str())
    }
}

impl TryFrom<&String> for SectionIdentifier {
    type Error = SectionIdentifierError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        SectionIdentifier::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Modifier;
    use std::convert::TryFrom;

    #[test]
    fn try_from_str_test() {
        assert_eq!(
            SectionIdentifier::try_from("A cool new section").unwrap(),
            SectionIdentifier("a-cool-new-section".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from("Eine großartige Überschrift").unwrap(),
            SectionIdentifier("eine-grossartige-ueberschrift".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from("A   lot   of   space").unwrap(),
            SectionIdentifier("a---lot---of---space".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from("Tabs\tare\there").unwrap(),
            SectionIdentifier("tabs-are-here".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from("    Surrounding space    ").unwrap(),
            SectionIdentifier("surrounding-space".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from("Dashes - all-over-the-place").unwrap(),
            SectionIdentifier("dashes---all-over-the-place".to_owned())
        );

        // "y̆" is not a single `char` but "y"+" ̆"
        assert_eq!(
            SectionIdentifier::try_from("y̆").unwrap(),
            SectionIdentifier("y".to_owned())
        );
    }

    #[test]
    fn try_from_node_test() {
        let modifier = Modifier::Chorus;
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "A cool new section", modifier)).unwrap(),
            SectionIdentifier("a-cool-new-section".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "A cool new section", modifier)).unwrap(),
            SectionIdentifier("a-cool-new-section".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "Eine großartige Überschrift", modifier))
                .unwrap(),
            SectionIdentifier("eine-grossartige-ueberschrift".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "A   lot   of   space", modifier))
                .unwrap(),
            SectionIdentifier("a---lot---of---space".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "Tabs\tare\there", modifier)).unwrap(),
            SectionIdentifier("tabs-are-here".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "    Surrounding space    ", modifier))
                .unwrap(),
            SectionIdentifier("surrounding-space".to_owned())
        );
        assert_eq!(
            SectionIdentifier::try_from(Node::headline(1, "Dashes - all-over-the-place", modifier))
                .unwrap(),
            SectionIdentifier("dashes---all-over-the-place".to_owned())
        );
    }

    #[test]
    fn try_from_newline_should_fail_test() {
        assert!(SectionIdentifier::try_from(Node::newline()).is_err());
        assert_eq!(
            SectionIdentifierError::UnsupportedInput,
            SectionIdentifier::try_from(Node::newline()).unwrap_err()
        );
    }

    #[test]
    fn try_from_empty_should_fail_test() {
        assert_eq!(
            SectionIdentifierError::Empty,
            SectionIdentifier::try_from("").unwrap_err()
        );
        assert_eq!(
            SectionIdentifierError::Empty,
            SectionIdentifier::try_from(" ").unwrap_err()
        );
        assert_eq!(
            SectionIdentifierError::Empty,
            SectionIdentifier::try_from("\t").unwrap_err()
        );
        assert_eq!(
            SectionIdentifierError::Empty,
            SectionIdentifier::try_from("\n").unwrap_err()
        );
        assert_eq!(
            SectionIdentifierError::Empty,
            SectionIdentifier::try_from("§ê").unwrap_err()
        );
    }
}
