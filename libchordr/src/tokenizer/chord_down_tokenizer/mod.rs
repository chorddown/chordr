mod mode;

use crate::tokenizer::{Tokenizer, Token};
use mode::Mode;
use crate::tokenizer::chord_down_tokenizer::mode::ModePartner;

pub(crate) struct ChordDownTokenizer {}

impl ChordDownTokenizer {
    pub fn new() -> Self {
        Self {}
    }
}


impl Tokenizer for ChordDownTokenizer {
    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut literal_buffer = String::from("");
        let mut header_level: u8 = 1;

        let mut reference_mode = Mode::Literal;

        let mut chars = line.chars().peekable();
        let mut is_first_frame = true;

        while let Some(current_character) = chars.next() {
            let last_mode = reference_mode;

            if is_first_frame {
                reference_mode = Mode::from_char(current_character);
                is_first_frame = false;
            }

            let current_mode = Mode::from_char(current_character);
            if last_mode == Mode::Header && current_mode == Mode::Header {
                header_level += 1;
            }

            let next_character = *chars.peek().unwrap_or(&'\n');
            let next_mode = Mode::from_char(next_character);

            if !current_character.is_signal() {
                literal_buffer.push(current_character);
            }

            if reference_mode.is_self_closing() {
                add_token(&mut tokens, from_mode_and_literal(reference_mode, &mut literal_buffer, header_level));
            } else if current_character.is_end_of(reference_mode) || reference_mode.is_terminated_by_char(next_character) {
                // Mode changed => build and append a new token
                build_and_add_token(&mut tokens, &mut literal_buffer, reference_mode, header_level);
                reference_mode = next_mode;
            }
        }

//        if !literal_buffer.is_empty() {
//            build_and_add_token(&mut tokens, &mut literal_buffer, current_frame.mode, header_level);
//        }

        tokens
    }
}

fn from_mode_and_literal(mode: Mode, literal: &str, header_level: u8) -> Token {
    match mode {
        Mode::Header => Token::Headline { level: header_level, text: literal.trim().to_owned() },
        Mode::Literal => Token::Literal(literal.to_owned()),
        Mode::Chord => Token::Chord(literal.trim().to_owned()),
        // FormattedLiteral(String),
        // Meta { key: String, text: String },
        Mode::Quote => Token::Quote(literal.trim().to_owned()),
        Mode::Newline => Token::Newline,
    }
}

fn build_and_add_token(tokens: &mut Vec<Token>, collected_literal: &mut String, mode: Mode, header_level: u8) {
    if !collected_literal.is_empty() {
        add_token(tokens, from_mode_and_literal(mode, &collected_literal, header_level));
        collected_literal.clear();
    }
}

fn add_token(tokens: &mut Vec<Token>, token: Token) -> () {
    tokens.push(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_tokens;

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../../tests/resources/swing_low_sweet_chariot.chorddown");
        let token_lines = ChordDownTokenizer::new().tokenize(content);
        assert_eq!(token_lines.len(), 16);

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

