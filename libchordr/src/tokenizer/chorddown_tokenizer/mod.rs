mod mode;
mod scanner;
mod state_machine;

use self::state_machine::FSM;
use crate::tokenizer::chorddown_tokenizer::scanner::Scanner;
use crate::tokenizer::{Token, Tokenizer};

pub(crate) struct ChorddownTokenizer {}

impl ChorddownTokenizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tokenizer for ChorddownTokenizer {
    fn tokenize(&self, line: &str) -> Vec<Token> {
        let lexemes_vec = Scanner::new().scan(line);
        let mut lexemes = lexemes_vec.iter().peekable();
        let mut tokens: Vec<Token> = vec![];
        let mut fsm = FSM::new();

        while let Some(lexeme) = lexemes.next() {
            if let Some(changed_state) = fsm.characterize_lexeme(lexeme) {
                let token = fsm.build_token();

                if let Some(token) = token {
                    tokens.push(token);
                }
                fsm.set_state(changed_state);
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::meta::BNotation;
    use crate::test_helpers::get_test_tokens;
    use crate::tokenizer::{Meta, Modifier};

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../../tests/resources/swing_low_sweet_chariot.chorddown");
        let token_lines = ChorddownTokenizer::new().tokenize(content);
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
        let token_lines = ChorddownTokenizer::new().tokenize(content);
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
        let tokens = ChorddownTokenizer::new().tokenize(content);
        assert_eq!(
            tokens.get(0),
            Some(&Token::Meta(Meta::composer("Daniel Corn")))
        );
        assert_eq!(tokens.get(1), Some(&Token::Newline));
        assert_eq!(
            tokens.get(2),
            Some(&Token::Meta(Meta::artist("The Fantastic Corns")))
        );
        assert_eq!(tokens.get(3), Some(&Token::Newline));
        assert_eq!(tokens.get(4), Some(&Token::Meta(Meta::key("Cm"))));
        assert_eq!(tokens.get(5), Some(&Token::Newline));
    }

    #[test]
    fn test_tokenize_newline() {
        let content = "\n\n\n";
        let tokens = ChorddownTokenizer::new().tokenize(content);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens, vec![Token::Newline, Token::Newline, Token::Newline]);
    }

    #[test]
    fn test_tokenize_meta_key() {
        let content = r"
Key: C#m
";
        let tokens = ChorddownTokenizer::new().tokenize(content);

        assert_eq!(
            tokens,
            vec![
                Token::Newline,
                Token::Meta(Meta::key("C#m")),
                Token::Newline
            ]
        );
    }

    #[test]
    fn test_tokenize_meta_with_inline_sharp() {
        let tokens = ChorddownTokenizer::new().tokenize("Album: Song in C#m");
        assert_eq!(tokens, vec![Token::Meta(Meta::album("Song in C#m"))]);
    }

    #[test]
    fn test_tokenize_meta_b_notation() {
        let tokenizer = ChorddownTokenizer::new();
        // H
        {
            let tokens = tokenizer.tokenize("B Notation: H");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B_Notation: H");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("BNotation: H");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::H)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B-Notation: H");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::H)))
            );
        }
        // B
        {
            let tokens = tokenizer.tokenize("B Notation: B");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B_Notation: B");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("BNotation: B");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::B)))
            );
        }
        {
            let tokens = tokenizer.tokenize("B-Notation: B");

            assert_eq!(
                tokens.get(0),
                Some(&Token::Meta(Meta::BNotation(BNotation::B)))
            );
        }
    }
}
