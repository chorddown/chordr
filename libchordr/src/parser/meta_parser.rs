use crate::tokenizer::{Meta, Token};

pub use super::meta_information::MetaInformation;
pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
pub use super::*;

pub struct MetaParser {}

impl ParserTrait for MetaParser {
    type OkType = MetaInformation;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error> {
        self.parse_borrowed(&tokens)
    }
}

impl MetaParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_borrowed(&mut self, tokens: &[Token]) -> Result<MetaInformation, Error> {
        let mut meta = MetaInformation::default();
        for token in tokens {
            meta = self.visit(token, meta);
        }

        Ok(meta)
    }

    fn visit(&self, token: &Token, meta: MetaInformation) -> MetaInformation {
        log::trace!("Visit token: {:?}", token);
        match token {
            Token::Chord(_) => self.visit_chord(token, meta),
            Token::Headline {
                level,
                ref text,
                modifier: _,
            } => {
                if *level == 1 {
                    MetaInformation {
                        title: Some(text.clone()),
                        ..meta
                    }
                } else {
                    meta
                }
            }
            // Token::Meta(Meta::BNotation(notation)) => {
            //     let mut new_meta = meta;
            //     new_meta.reinterpret_keys_with_b_notation(*notation);
            //     // new_meta.assign_from_token(token_meta);
            //     new_meta
            // }
            Token::Meta(token_meta) => {
                let mut new_meta = meta;
                new_meta.assign_from_token(token_meta);
                if let Meta::BNotation(b_notation) = token_meta {
                    new_meta.reinterpret_keys_with_b_notation(*b_notation);
                }
                new_meta
            }
            _ => meta,
        }
    }

    fn visit_chord(&self, token: &Token, meta: MetaInformation) -> MetaInformation {
        let chords = if let Token::Chord(c) = token {
            c
        } else {
            unreachable!("Invalid Token given")
        };
        if BNotation::contains_european_chord(chords) {
            MetaInformation {
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
    use crate::test_helpers::get_test_tokens_small;
    use crate::tokenizer::Modifier;

    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = MetaParser::new();
        let result = parser.parse(get_test_tokens_small());

        assert!(result.is_ok());
        assert_eq!(
            Some("Swing Low Sweet Chariot".to_string()),
            result.unwrap().title
        );
    }

    #[test]
    fn test_detect_b_notation() {
        let mut parser = MetaParser::new();
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
