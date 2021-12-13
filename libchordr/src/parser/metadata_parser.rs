pub use crate::models::metadata::Metadata;
use crate::parser::metadata_builder::MetadataBuilder;
use crate::tokenizer::{RawMetadata, Token};

pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
pub use super::*;

pub struct MetadataParser {}

impl ParserTrait for MetadataParser {
    type OkType = Metadata;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error> {
        self.parse_borrowed(&tokens)
    }
}

impl MetadataParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_borrowed(&mut self, tokens: &[Token]) -> Result<Metadata, Error> {
        let mut metadata_builder = MetadataBuilder::new();
        for token in tokens {
            metadata_builder = self.visit(token, metadata_builder);
        }

        Ok(metadata_builder.build())
    }

    fn visit(&self, token: &Token, metadata_builder: MetadataBuilder) -> MetadataBuilder {
        log::trace!("Visit token: {:?}", token);
        match token {
            Token::Chord(_) => self.visit_chord(token, metadata_builder),
            Token::Headline {
                level,
                ref text,
                modifier: _,
            } => {
                if *level == 1 {
                    metadata_builder.with_title(text)
                } else {
                    metadata_builder
                }
            }
            Token::Metadata(token_meta) => {
                let mut new_metadata_builder = metadata_builder;
                new_metadata_builder.assign_from_token(token_meta.clone());
                if let RawMetadata::BNotation(b_notation) = token_meta {
                    new_metadata_builder.reinterpret_keys_with_b_notation(*b_notation);
                }
                new_metadata_builder
            }
            _ => metadata_builder,
        }
    }

    fn visit_chord(&self, token: &Token, metadata_build: MetadataBuilder) -> MetadataBuilder {
        let chords = if let Token::Chord(c) = token {
            c
        } else {
            unreachable!("Invalid Token given")
        };
        if BNotation::contains_european_chord(chords) {
            metadata_build.with_b_notation(BNotation::H)
        } else {
            metadata_build
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::get_test_parser_input;
    use crate::tokenizer::Modifier;

    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = MetadataParser::new();
        let result = parser.parse(get_test_parser_input());

        assert!(result.is_ok());
        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            result.unwrap().title
        );
    }

    #[test]
    fn test_detect_b_notation() {
        let mut parser = MetadataParser::new();
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.unwrap().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
            ]);

            assert_eq!(result.unwrap().b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.unwrap().b_notation, BNotation::H);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]);

            assert_eq!(result.unwrap().b_notation, BNotation::H);
        }
    }
}
