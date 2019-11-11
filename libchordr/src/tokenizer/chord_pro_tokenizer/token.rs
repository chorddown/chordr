use super::directive::Directive;
use super::mode::Mode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Chord(String),
    Directive(Directive),
    Literal(String),
    Newline,
    Comment(String),
}

impl Token {
    pub fn from_mode_and_literal(mode: Mode, literal: &str) -> Token {
        match mode {
            Mode::Chord => Token::Chord(literal.trim().to_owned()),
            Mode::Comment => Token::Comment(literal.trim().to_owned()),
            Mode::Literal => Token::Literal(literal.to_owned()),
            Mode::Newline => Token::Newline,
            Mode::Directive => Token::Directive(literal.into())
        }
    }

    pub fn chord<S: Into<String>>(value: S) -> Self {
        Token::Chord(value.into())
    }

    pub fn literal<S: Into<String>>(value: S) -> Self {
        Token::Literal(value.into())
    }

    pub fn comment<S: Into<String>>(value: S) -> Self {
        Token::Comment(value.into())
    }
}
