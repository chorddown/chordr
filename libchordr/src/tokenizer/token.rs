#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    Headline { level: u8, text: String },
    Literal(String),
    Chord(String),
    // FormattedLiteral(String),
    // Meta { key: String, text: String },
    Quote(String),
    Newline,
}

impl Token {
    pub fn headline<S: Into<String>>(level: u8, value: S) -> Self {
        Token::Headline { level, text: value.into() }
    }

    pub fn chord<S: Into<String>>(value: S) -> Self {
        Token::Chord(value.into())
    }

    pub fn newline() -> Self {
        Token::Newline
    }

    pub fn literal<S: Into<String>>(value: S) -> Self {
        Token::Literal(value.into())
    }

    pub fn quote<S: Into<String>>(value: S) -> Self {
        Token::Quote(value.into())
    }
}
