use crate::error::Error;
use crate::models::meta::*;
use crate::parser::meta_parser::MetaParser;
use crate::parser::node_parser::NodeParser;
use crate::tokenizer::Token;

pub use self::meta_information::MetaInformation;
pub use self::node::Node;
pub use self::parser_result::ParserResult;
pub use self::section_type::SectionType;

mod meta_information;
mod meta_parser;
mod node;
mod node_parser;
mod parser_result;
mod section_type;

pub trait ParserTrait {
    type OkType;

    /// Parse the given tokens into the Parser's result
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error>;
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParserTrait for Parser {
    type OkType = ParserResult;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<ParserResult, Error> {
        let meta = MetaParser::new().parse_borrowed(&tokens)?;
        let node = NodeParser::with_b_notation(meta.b_notation).parse(tokens)?;

        Ok(ParserResult::new(node, meta))
    }
}

impl Parser {
    /// Remove leading and duplicate Newline tokens from the stream
    pub fn cleanup_tokens(tokens: Vec<Token>) -> Vec<Token> {
        // Initialize to `true` so that leading Newline tokens will be skipped
        let mut previous_token_was_newline: bool = true;

        tokens
            .into_iter()
            .filter(|token| {
                // Skip multiple Newline tokens
                if *token == Token::Newline && previous_token_was_newline {
                    false
                } else {
                    previous_token_was_newline = *token == Token::Newline;
                    true
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{
        get_test_ast, get_test_ast_small, get_test_tokens, get_test_tokens_small,
    };
    use crate::tokenizer::Modifier;

    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = Parser::new();
        let result = parser.parse(get_test_tokens_small());

        assert!(result.is_ok());
        let parser_result = result.unwrap();
        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            parser_result.meta().title
        );

        let ast = parser_result.node();

        let expected_ast = get_test_ast_small();

        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_2() {
        let mut parser = Parser::new();
        let result = parser.parse(get_test_tokens());

        assert!(result.is_ok());
        let parser_result = result.unwrap();
        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            parser_result.meta().title
        );

        assert_eq!(parser_result.node(), get_test_ast());
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

    #[test]
    fn test_cleanup_tokens() {
        assert_eq!(
            Parser::cleanup_tokens(vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]),
            vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]
        );

        assert_eq!(
            Parser::cleanup_tokens(vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::newline(),
                Token::newline(),
                Token::chord("H"),
            ]),
            vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]
        );

        assert_eq!(
            Parser::cleanup_tokens(vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::newline(),
                Token::chord("H"),
                Token::newline(),
            ]),
            vec![
                Token::headline(1, "Test", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::newline(),
            ]
        );

        assert_eq!(
            Parser::cleanup_tokens(vec![
                Token::newline(),
                Token::newline(),
                Token::chord("H"),
                Token::newline(),
            ]),
            vec![Token::chord("H"), Token::newline(),]
        );
        assert_eq!(
            Parser::cleanup_tokens(vec![
                Token::newline(),
                Token::newline(),
                Token::chord("H"),
                Token::newline(),
                Token::newline(),
            ]),
            vec![Token::chord("H"), Token::newline(),]
        );
    }
}
