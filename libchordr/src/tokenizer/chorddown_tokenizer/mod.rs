use std::io::BufRead;

use crate::error::Error;

use super::{Token, Tokenizer};

use self::scanner::Scanner;
use self::state_machine::Fsm;

mod keywords;
mod lexeme;
mod mode;
mod scanner;
mod state_machine;

pub(crate) struct ChorddownTokenizer {}

impl ChorddownTokenizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tokenizer for ChorddownTokenizer {
    fn tokenize<R: BufRead>(&self, input: R) -> Result<Vec<Token>, Error> {
        let lexemes_vec = Scanner::new().scan(input)?;
        let lexemes = lexemes_vec.iter().peekable();
        let mut tokens: Vec<Token> = vec![];
        let mut fsm = Fsm::new();

        for lexeme in lexemes {
            if let Some(changed_state) = fsm.characterize_lexeme(lexeme) {
                let token = fsm.build_token();

                if let Some(token) = token {
                    tokens.push(token);
                }
                fsm.set_state(changed_state);
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::metadata::BNotation;
    use crate::test_helpers::get_test_tokens;
    use crate::tokenizer::{Modifier, RawMetadata};

    use super::*;

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../../tests/resources/swing_low_sweet_chariot.chorddown");
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(token_lines, get_test_tokens());
    }

    #[test]
    fn test_tokenize_main() {
        use Token::Newline;
        let content = r"
# Swing Low Sweet Chariot

##! Chorus
Swing [D]low, sweet [G]chari[D]ot

> A quote [D#m7]
";
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            token_lines,
            vec![
                Newline,
                Token::headline(1, "Swing Low Sweet Chariot", Modifier::None),
                Newline,
                Newline,
                Token::headline(2, "Chorus", Modifier::Chorus),
                Newline,
                Token::literal("Swing "),
                Token::chord("D"),
                Token::literal("low, sweet "),
                Token::chord("G"),
                Token::literal("chari"),
                Token::chord("D"),
                Token::literal("ot"),
                Newline,
                Newline,
                Token::quote("A quote [D#m7]"),
                Newline
            ]
        );
    }

    #[test]
    fn test_tokenize_meta() {
        let content = r"Composer: Daniel Corn
Artist: The Fantastic Corns
Key: Cm
";
        let tokens = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            tokens.get(0),
            Some(&Token::Metadata(RawMetadata::composer("Daniel Corn")))
        );
        assert_eq!(tokens.get(1), Some(&Token::Newline));
        assert_eq!(
            tokens.get(2),
            Some(&Token::Metadata(RawMetadata::artist("The Fantastic Corns")))
        );
        assert_eq!(tokens.get(3), Some(&Token::Newline));
        assert_eq!(
            tokens.get(4),
            Some(&Token::Metadata(RawMetadata::key("Cm")))
        );
        assert_eq!(tokens.get(5), Some(&Token::Newline));
    }

    #[test]
    fn test_tokenize_newline() {
        let content = "\n\n\n";
        let tokens = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens, vec![Token::Newline, Token::Newline, Token::Newline]);
    }

    #[test]
    fn test_tokenize_meta_key() {
        let content = r"
Key: C#m
";
        let tokens = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Newline,
                Token::Metadata(RawMetadata::key("C#m")),
                Token::Newline
            ]
        );
    }

    #[test]
    fn test_tokenize_meta_with_inline_sharp() {
        let tokens = ChorddownTokenizer::new()
            .tokenize("Album: Song in C#m".as_bytes())
            .unwrap();
        assert_eq!(
            tokens,
            vec![Token::Metadata(RawMetadata::album("Song in C#m"))]
        );
    }

    #[test]
    fn test_tokenize_meta_b_notation() {
        let tokenizer = ChorddownTokenizer::new();
        // H
        {
            let tokens = tokenizer.tokenize("B Notation: H".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B_Notation: H".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("BNotation: H".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B-Notation: H".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::H)))
            );
        }
        // B
        {
            let tokens = tokenizer.tokenize("B Notation: B".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B_Notation: B".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("BNotation: B".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B-Notation: B".as_bytes()).unwrap();

            assert_eq!(
                tokens.get(0),
                Some(&Token::Metadata(RawMetadata::BNotation(BNotation::B)))
            );
        }
    }

    #[test]
    fn test_tokenize_pre_chorus() {
        let content = r"##- Pre-chorus";
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            token_lines,
            vec![Token::headline(2, "Pre-chorus", Modifier::Bridge)]
        );
    }

    #[test]
    fn test_tokenize_chorus_with_exclamation_marks() {
        let content = r"##! Chorus Loud!!";
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            token_lines,
            vec![Token::headline(2, "Chorus Loud!!", Modifier::Chorus)]
        );
    }

    #[test]
    fn test_tokenize_bride_with_exclamation_marks() {
        let content = r"##- Bride Loud!!";
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            token_lines,
            vec![Token::headline(2, "Bride Loud!!", Modifier::Bridge)]
        );
    }

    #[test]
    fn test_tokenize_chorus_with_exclamation_marks_at_end() {
        let content = r"## Play Loud!!";
        let token_lines = ChorddownTokenizer::new()
            .tokenize(content.as_bytes())
            .unwrap();
        assert_eq!(
            token_lines,
            vec![Token::headline(2, "Play Loud!!", Modifier::None)]
        );
    }
}
