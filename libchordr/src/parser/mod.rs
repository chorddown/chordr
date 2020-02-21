mod meta_information;
mod meta_parser;
mod node;
mod node_parser;
mod parser_result;
mod section_type;

pub use self::meta_information::MetaInformation;
pub use self::node::Node;
pub use self::parser_result::ParserResult;
pub use self::section_type::SectionType;
use crate::error::Error;
use crate::models::meta::*;
use crate::parser::meta_parser::MetaParser;
use crate::parser::node_parser::NodeParser;
use crate::tokenizer::Token;

pub trait ParserTrait {
    type OkType;

    /// Parse the given tokens into the Parser's result
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error>;
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParserTrait for Parser {
    type OkType = ParserResult;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<ParserResult, Error> {
        let meta = MetaParser::new().parse(tokens.clone())?;
        let node = NodeParser::with_b_notation(meta.b_notation).parse(tokens)?;

        Ok(ParserResult::new(node, meta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_parser_input;
    use crate::tokenizer::Modifier;

    #[test]
    fn test_parse() {
        let mut parser = Parser::new();
        let result = parser.parse(get_test_parser_input());

        assert!(result.is_ok());
        let parser_result = result.unwrap();
        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            parser_result.meta().title
        );

        let ast = parser_result.node();

        let expected_ast = Node::Document(vec![
            Node::section(
                1,
                "Swing Low Sweet Chariot",
                Modifier::None,
                vec![Node::newline()],
            ),
            Node::section(
                2,
                "Chorus",
                Modifier::Chorus,
                vec![
                    Node::text("Swing "),
                    Node::chord_text_pair("D", "low, sweet ").unwrap(),
                    Node::chord_text_pair("G", "chari").unwrap(),
                    Node::chord_text_pair("D", "ot,").unwrap(),
                    Node::text("Comin’ for to carry me "),
                    Node::chord_text_pair("A7", "home.").unwrap(),
                    Node::text("Swing "),
                    Node::chord_standalone("D7").unwrap(),
                ],
            ),
            Node::section(
                2,
                "Verse",
                Modifier::None,
                vec![
                    Node::chord_text_pair("E", "low, sweet ").unwrap(),
                    Node::chord_text_pair("G", "chari").unwrap(),
                    Node::chord_text_pair("D", "ot,").unwrap(),
                    Node::chord_standalone("E").unwrap(),
                    Node::chord_standalone("A").unwrap(),
                    Node::newline(),
                    Node::chord_standalone("A#").unwrap(),
                    Node::chord_standalone("H").unwrap(),
                ],
            ),
        ]);

        assert_eq!(expected_ast, ast);
    }

    #[test]
    fn test_detect_b_notation() {
        let mut parser = Parser::new();
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
                Token::literal("A text"),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::literal("A text"),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().meta_as_ref().b_notation, BNotation::H);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().meta_as_ref().b_notation, BNotation::H);
        }
    }
}
