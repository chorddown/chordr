mod meta_information;
mod node;
mod parser_result;
mod node_parser;
mod meta_parser;
mod section_type;

pub use self::meta_information::MetaInformation;
pub use self::node::Node;
pub use self::parser_result::ParserResult;
pub use self::section_type::SectionType;
use crate::models::meta::*;
use crate::tokenizer::Token;
use crate::parser::node_parser::NodeParser;
use crate::parser::meta_parser::MetaParser;

pub trait ParserTrait {
    type Result;

    /// Parse the given tokens into the Parser's result
    fn parse(&mut self, tokens: Vec<Token>) -> Self::Result;
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParserTrait for Parser {
    type Result = ParserResult;

    fn parse(&mut self, tokens: Vec<Token>) -> ParserResult {
        let meta = MetaParser::new().parse(tokens.clone());
        let node = NodeParser::with_b_notation(meta.b_notation).parse(tokens);

        ParserResult::new(node, meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Modifier;

    #[test]
    fn test_parse() {
        let mut parser = Parser::new();
        let result = parser.parse(vec![
            Token::headline(1, "Swing Low Sweet Chariot", Modifier::None),
            Token::newline(),
            Token::headline(2, "Chorus", Modifier::Chorus),
            Token::literal("Swing "),
            Token::chord("D"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::literal("Comin’ for to carry me "),
            Token::chord("A7"),
            Token::literal("home."),
            Token::literal("Swing "),
            Token::chord("D7"),
            Token::headline(2, "Verse", Modifier::None),
            Token::chord("E"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::chord("E"),
            Token::chord("A"),
            Token::newline(),
            Token::chord("B"),
            Token::chord("H"),
        ]);

        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            result.meta().title
        );

        let ast = result.node();

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
                    Node::chord_text_pair("D", "low, sweet "),
                    Node::chord_text_pair("G", "chari"),
                    Node::chord_text_pair("D", "ot,"),
                    Node::text("Comin’ for to carry me "),
                    Node::chord_text_pair("A7", "home."),
                    Node::text("Swing "),
                    Node::chord_standalone("D7"),
                ],
            ),
            Node::section(
                2,
                "Verse",
                Modifier::None,
                vec![
                    Node::chord_text_pair("E", "low, sweet "),
                    Node::chord_text_pair("G", "chari"),
                    Node::chord_text_pair("D", "ot,"),
                    Node::chord_standalone("E"),
                    Node::chord_standalone("A"),
                    Node::newline(),
                    Node::chord_standalone("A#"),
                    Node::chord_standalone("H"),
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

            assert_eq!(result.meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::H);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]);

            assert_eq!(result.meta_as_ref().b_notation, BNotation::H);
        }
    }
}
