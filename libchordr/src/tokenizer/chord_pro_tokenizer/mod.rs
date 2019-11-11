mod mode;
mod token;
mod directive;

use self::mode::Mode;
use self::token::Token;
use crate::tokenizer::{TokenLine, Tokenizer};

pub struct ChordProTokenizer {}

impl Tokenizer for ChordProTokenizer {
    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        let mut collected_literal = String::from("");
        let mut mode = Mode::Literal;
        let mut tokens: Vec<Token> = vec![];
        for character in line.chars() {
            // If no lock is set check if the current character signals a start of a block
            if mode == Mode::Literal {
                let new_mode = Mode::from_char(character);

                // If the mode is Newline
                if new_mode.is_self_closing() {
                    // Add token for the previous mode
                    ChordProTokenizer::add_token(&mut tokens, &mut collected_literal, mode);

                    // and the Newline token
                    tokens.push(Token::from_mode_and_literal(new_mode, ""));

                    mode = Mode::Literal;

                    continue;
                }

                // If the mode changed and it did not change to None
                if new_mode != mode && new_mode != Mode::Literal {
                    ChordProTokenizer::add_token(&mut tokens, &mut collected_literal, mode);
                    mode = new_mode;

                    continue;
                }
            } else if mode.is_end_character(character) {
                ChordProTokenizer::add_token(&mut tokens, &mut collected_literal, mode);
                mode = Mode::Literal;

                continue;
            }

            collected_literal.push(character);
        }
        if !collected_literal.is_empty() {
            tokens.push(Token::from_mode_and_literal(mode, &collected_literal));
        }
        tokens
    }
}

impl ChordProTokenizer {
    pub fn new() -> Self {
        Self {}
    }

    fn add_token(tokens: &mut Vec<Token>, collected_literal: &mut String, mode: Mode) {
        if !collected_literal.is_empty() {
            tokens.push(Token::from_mode_and_literal(mode, &collected_literal));
            collected_literal.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_tokens;

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../../tests/resources/swing_low_sweet_chariot.cho");
        let token_lines = ChordProTokenizer::new().tokenize(content);
        assert_eq!(token_lines.len(), 17);

        let mut token_lines_iter = token_lines.iter();

        for expected_line in get_test_tokens() {
            let line = token_lines_iter.next().unwrap();
            let mut line_iter = line.iter();
            for expected_token in expected_line {
                let actual_token = line_iter.next().unwrap();
                assert_eq!(&expected_token, actual_token);
            }
        }
    }
}
