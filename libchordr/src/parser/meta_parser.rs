pub use super::meta_information::MetaInformation;
pub use super::*;
pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
use crate::tokenizer::Token;

pub struct MetaParser {}

impl ParserTrait for MetaParser {
    type Result = MetaInformation;


    fn parse(&mut self, tokens: Vec<Token>) -> MetaInformation {
        let mut meta = MetaInformation::default();
        for token in tokens {
            meta = self.visit(token, meta);
        }

        meta
    }
}


impl MetaParser {
    pub fn new() -> Self {
        Self {}
    }

    fn visit(&mut self, token: Token, meta: MetaInformation) -> MetaInformation {
        match token {
            Token::Chord(_) => self.visit_chord(token, meta),
            Token::Headline {
                level,
                ref text,
                modifier: _,
            } => {
                if level == 1 {
                    MetaInformation { title: Some(text.clone()), ..meta }
                } else {
                    meta
                }
            }
            Token::Meta(token_meta) => {
                let mut new_meta = meta.clone();
                new_meta.assign_from_token(&token_meta);
                new_meta
            }
            _ => meta
        }
    }

    fn visit_chord(&mut self, token: Token, meta: MetaInformation) -> MetaInformation {
        let chords = if let Token::Chord(c) = token { c } else { unreachable!("Invalid Token given") };

        if BNotation::is_european_chord(&chords) {
//            self.meta.b_notation = BNotation::H;
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
    use super::*;
    use crate::tokenizer::Modifier;

    #[test]
    fn test_parse() {
        let mut parser = MetaParser::new();
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
            result.title
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

            assert_eq!(result.b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with standard B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("E"),
            ]);

            assert_eq!(result.b_notation, BNotation::B);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/ text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
                Token::literal("A text"),
            ]);

            assert_eq!(result.b_notation, BNotation::H);
        }
        {
            let result = parser.parse(vec![
                Token::headline(1, "Test with european B notation w/o text", Modifier::None),
                Token::newline(),
                Token::chord("H"),
            ]);

            assert_eq!(result.b_notation, BNotation::H);
        }
    }
}
