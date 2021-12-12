use crate::tokenizer::{RawMetadata, Token};

pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
pub use super::*;
pub use crate::models::metadata::Metadata;

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
        let mut meta = Metadata::default();
        for token in tokens {
            meta = self.visit(token, meta);
        }

        Ok(meta)
    }

    fn visit(&self, token: &Token, meta: Metadata) -> Metadata {
        log::trace!("Visit token: {:?}", token);
        match token {
            Token::Chord(_) => self.visit_chord(token, meta),
            Token::Headline {
                level,
                ref text,
                modifier: _,
            } => {
                if *level == 1 {
                    Metadata {
                        title: Some(text.clone()),
                        ..meta
                    }
                } else {
                    meta
                }
            }
            // Token::Meta(Meta::BNotation(notation)) => {
            //     let mut new_meta = metadata;
            //     new_meta.reinterpret_keys_with_b_notation(*notation);
            //     // new_meta.assign_from_token(token_meta);
            //     new_meta
            // }
            Token::Metadata(token_meta) => {
                let mut new_meta = meta;
                new_meta.assign_from_token(token_meta);
                if let RawMetadata::BNotation(b_notation) = token_meta {
                    new_meta.reinterpret_keys_with_b_notation(*b_notation);
                }
                new_meta
            }
            _ => meta,
        }
    }

    fn visit_chord(&self, token: &Token, meta: Metadata) -> Metadata {
        let chords = if let Token::Chord(c) = token {
            c
        } else {
            unreachable!("Invalid Token given")
        };
        if BNotation::contains_european_chord(chords) {
            Metadata {
                b_notation: BNotation::H,
                ..meta
            }
        } else {
            meta
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
