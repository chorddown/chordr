use crate::tokenizer::chorddown_tokenizer::lexeme::Lexeme;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Mode {
    Chord = 8,
    Header = 4,
    Newline = 10,
    Quote = 6,
    Literal = 0,
    Bof = 100,
    Eof = 110,
}

const NEWLINE: char = '\n';
const CHORD_START: char = '[';
#[allow(dead_code)]
const CHORD_END: char = ']';
const HEADER_START: char = '#';
#[allow(dead_code)]
const HEADER_END: char = NEWLINE;
const QUOTE_START: char = '>';
#[allow(dead_code)]
const QUOTE_END: char = NEWLINE;

impl Mode {
    pub fn from_char(character: char) -> Self {
        match character {
            QUOTE_START => Mode::Quote,
            HEADER_START => Mode::Header,
            CHORD_START => Mode::Chord,
            NEWLINE => Mode::Newline,
            _ => Mode::Literal,
        }
    }

    pub fn from_lexeme(lexeme: &Lexeme) -> Self {
        match *lexeme {
            Lexeme::QuoteStart => Mode::Quote,
            Lexeme::HeaderStart => Mode::Header,
            Lexeme::ChordStart => Mode::Chord,
            Lexeme::Newline => Mode::Newline,
            _ => Mode::Literal,
        }
    }

    #[allow(unused)]
    pub fn is_terminated_by(self, mode: Mode) -> bool {
        // Newline is special: it can terminate any mode, but will also be terminated by any other mode
        if self == Mode::Newline {
            true
        } else {
            self < mode
        }
    }
}

impl From<char> for Mode {
    fn from(character: char) -> Self {
        Mode::from_char(character)
    }
}

impl From<&char> for Mode {
    fn from(character: &char) -> Self {
        Mode::from_char(*character)
    }
}

impl From<&Lexeme> for Mode {
    fn from(lexeme: &Lexeme) -> Self {
        Mode::from_lexeme(lexeme)
    }
}

impl From<Lexeme> for Mode {
    fn from(lexeme: Lexeme) -> Self {
        Mode::from_lexeme(&lexeme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newline_is_terminated_by_everything() {
        assert!(Mode::Newline.is_terminated_by(Mode::Newline));
        assert!(Mode::Newline.is_terminated_by(Mode::Chord));
        assert!(Mode::Newline.is_terminated_by(Mode::Header));
        assert!(Mode::Newline.is_terminated_by(Mode::Quote));
        assert!(Mode::Newline.is_terminated_by(Mode::Literal));
    }

    #[test]
    fn test_everything_is_terminated_by_newline() {
        assert!(Mode::Newline.is_terminated_by(Mode::Newline));
        assert!(Mode::Chord.is_terminated_by(Mode::Newline));
        assert!(Mode::Header.is_terminated_by(Mode::Newline));
        assert!(Mode::Quote.is_terminated_by(Mode::Newline));
        assert!(Mode::Literal.is_terminated_by(Mode::Newline));
    }

    #[test]
    fn test_chord_is_terminated_by_newline_only() {
        assert!(Mode::Chord.is_terminated_by(Mode::Newline));
        assert!(!Mode::Chord.is_terminated_by(Mode::Chord));
        assert!(!Mode::Chord.is_terminated_by(Mode::Header));
        assert!(!Mode::Chord.is_terminated_by(Mode::Quote));
        assert!(!Mode::Chord.is_terminated_by(Mode::Literal));
    }

    #[test]
    fn test_header_is_terminated_by() {
        assert!(Mode::Header.is_terminated_by(Mode::Newline));
        assert!(Mode::Header.is_terminated_by(Mode::Chord));
        assert!(!Mode::Header.is_terminated_by(Mode::Header));
        assert!(Mode::Header.is_terminated_by(Mode::Quote));
        assert!(!Mode::Header.is_terminated_by(Mode::Literal));
    }

    #[test]
    fn test_quote_is_terminated_by() {
        assert!(Mode::Quote.is_terminated_by(Mode::Newline));
        assert!(Mode::Quote.is_terminated_by(Mode::Chord));
        assert!(!Mode::Quote.is_terminated_by(Mode::Header));
        assert!(!Mode::Quote.is_terminated_by(Mode::Quote));
        assert!(!Mode::Quote.is_terminated_by(Mode::Literal));
    }

    #[test]
    fn test_literal_is_terminated_by_everything_but_self() {
        assert!(Mode::Literal.is_terminated_by(Mode::Newline));
        assert!(Mode::Literal.is_terminated_by(Mode::Chord));
        assert!(Mode::Literal.is_terminated_by(Mode::Header));
        assert!(Mode::Literal.is_terminated_by(Mode::Quote));
        assert!(!Mode::Literal.is_terminated_by(Mode::Literal));
    }
}
