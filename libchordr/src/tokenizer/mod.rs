mod token;
mod mode;
mod directive;

pub struct Tokenizer {}

use self::mode::Mode;
pub use self::token::Token;
pub use self::directive::Directive;

pub type TokenLine = Vec<Token>;

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut collected_literal = String::from("");
        let mut mode = Mode::Literal;

        for character in input.chars() {
            // If no lock is set check if the current character signals a start of a block
            if mode == Mode::Literal {
                let new_mode = Mode::from_char(character);

                // If the mode is Newline
                if new_mode.is_self_closing() {
                    // Add token for the previous mode
                    Tokenizer::add_token(&mut tokens, &mut collected_literal, mode);

                    // and the Newline token
                    tokens.push(Token::from_mode_and_literal(new_mode, ""));

                    mode = Mode::Literal;

                    continue;
                }

                // If the mode changed and it did not change to None
                if new_mode != mode && new_mode != Mode::Literal {
                    Tokenizer::add_token(&mut tokens, &mut collected_literal, mode);
                    mode = new_mode;

                    continue;
                }
            } else if mode.is_end_character(character) {
                Tokenizer::add_token(&mut tokens, &mut collected_literal, mode);
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
        let content = include_str!("../../tests/resources/swing_low_sweet_chariot.cho");
        let tokens = Tokenizer::new().tokenize(content);
        assert_eq!(tokens.len(), 65);

        let mut tokens_iter = tokens.iter();
        for token in get_test_tokens() {
            assert_eq!(&token, tokens_iter.next().unwrap());
        }
    }
}
