use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme {
    Newline,
    ChordStart,
    ChordEnd,
    HeaderStart,
    QuoteStart,
    Colon,
    ChorusMark,
    BridgeMark,
    Literal(String),
    Eof,
}

const NEWLINE: char = '\n';
const CHORD_START: char = '[';
const CHORD_END: char = ']';
const HEADER_START: char = '#';
// const HEADER_END: char = NEWLINE;
const QUOTE_START: char = '>';
// const QUOTE_END: char = NEWLINE;
const COLON: char = ':';
const CHORUS_MARK: char = '!';
const BRIDGE_MARK: char = '-';

impl Lexeme {
    fn literal<S: Into<String>>(s: S) -> Self {
        Lexeme::Literal(s.into())
    }

    // pub fn as_str(&self) -> &str {
    //     match self {
    //         Lexeme::Newline => NEWLINE,
    //         Lexeme::ChordStart => CHORD_START,
    //         Lexeme::ChordEnd => CHORD_END,
    //         Lexeme::HeaderStart => HEADER_START,
    //         Lexeme::QuoteStart => QUOTE_START,
    //         Lexeme::Colon => COLON,
    //         Lexeme::ChorusMark => CHORUS_MARK,
    //         Lexeme::BridgeMark => BRIDGE_MARK,
    //         Lexeme::Literal(inner) => inner.as_str(),
    //     }
    // }

    pub fn to_string(&self) -> String {
        match self {
            Lexeme::Newline => NEWLINE.to_string(),
            Lexeme::ChordStart => CHORD_START.to_string(),
            Lexeme::ChordEnd => CHORD_END.to_string(),
            Lexeme::HeaderStart => HEADER_START.to_string(),
            Lexeme::QuoteStart => QUOTE_START.to_string(),
            Lexeme::Colon => COLON.to_string(),
            Lexeme::ChorusMark => CHORUS_MARK.to_string(),
            Lexeme::BridgeMark => BRIDGE_MARK.to_string(),
            Lexeme::Literal(inner) => inner.clone(),
            Lexeme::Eof => "".to_owned(),
        }
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Lexeme::Literal(inner) = self {
            write!(f, "{}", inner)
        } else if let Lexeme::Eof = self {
            write!(f, "")
        } else {
            write!(
                f,
                "{}",
                match self {
                    Lexeme::Newline => NEWLINE,
                    Lexeme::ChordStart => CHORD_START,
                    Lexeme::ChordEnd => CHORD_END,
                    Lexeme::HeaderStart => HEADER_START,
                    Lexeme::QuoteStart => QUOTE_START,
                    Lexeme::Colon => COLON,
                    Lexeme::ChorusMark => CHORUS_MARK,
                    Lexeme::BridgeMark => BRIDGE_MARK,
                    Lexeme::Literal(_) => unreachable!(),
                    Lexeme::Eof => unreachable!(),
                }
            )
        }
    }
}

pub struct Scanner {
    lexemes: Vec<Lexeme>,
}

impl Scanner {
    pub fn new() -> Self {
        Self { lexemes: vec![] }
    }
    pub fn scan(mut self, input: &str) -> Vec<Lexeme> {
        // let mut lexemes: Vec<Lexeme> = vec![];
        let mut literal_buffer = String::new();
        let mut chars = input.chars();
        while let Some(current_character) = chars.next() {
            match current_character {
                NEWLINE => self.push_n_drain(&mut literal_buffer, Lexeme::Newline),
                CHORD_START => self.push_n_drain(&mut literal_buffer, Lexeme::ChordStart),
                CHORD_END => self.push_n_drain(&mut literal_buffer, Lexeme::ChordEnd),
                HEADER_START => self.push_n_drain(&mut literal_buffer, Lexeme::HeaderStart),
                QUOTE_START => self.push_n_drain(&mut literal_buffer, Lexeme::QuoteStart),
                COLON => self.push_n_drain(&mut literal_buffer, Lexeme::Colon),
                CHORUS_MARK => self.push_n_drain(&mut literal_buffer, Lexeme::ChorusMark),
                BRIDGE_MARK => self.push_n_drain(&mut literal_buffer, Lexeme::BridgeMark),
                _ => literal_buffer.push(current_character),
            }
        }
        self.build_literal(&mut literal_buffer);

        self.lexemes.push(Lexeme::Eof);
        self.lexemes
    }

    fn push_n_drain(&mut self, literal_buffer: &mut String, l: Lexeme) {
        self.build_literal(literal_buffer);

        self.lexemes.push(l)
    }

    fn build_literal(&mut self, literal_buffer: &mut String) {
        if !literal_buffer.is_empty() {
            self.lexemes.push(Lexeme::Literal(literal_buffer.clone()));
            literal_buffer.clear();
        }
    }
}

mod shortcuts {
    use super::Lexeme;
    #[allow(unused_imports)]
    pub(super) use Lexeme::ChordEnd as CE;
    #[allow(unused_imports)]
    pub(super) use Lexeme::ChordStart as CS;
    #[allow(unused_imports)]
    pub(super) use Lexeme::Eof as EOF;
    #[allow(unused_imports)]
    pub(super) use Lexeme::HeaderStart as H;
    #[allow(unused_imports)]
    pub(super) use Lexeme::Newline as NL;

    #[allow(dead_code)]
    pub(super) fn lit(literal: &str) -> Lexeme {
        Lexeme::literal(literal)
    }
}

#[cfg(test)]
mod tests {
    use super::shortcuts::*;
    use super::*;

    #[test]
    fn scan_test() {
        let content = r"
# Swing Low Sweet Chariot

Composer: Daniel Corn

##! Chorus
Swing [D]low, sweet [G]chari[D]ot,
";
        let scanner = Scanner::new();
        let lexemes = scanner.scan(content);
        assert_eq!(lexemes.len(), 30);

        assert_eq!(
            lexemes,
            vec![
                NL,
                H,
                lit(" Swing Low Sweet Chariot"),
                NL,
                NL,
                lit("Composer"),
                Lexeme::Colon,
                lit(" Daniel Corn"),
                NL,
                NL,
                H,
                H,
                Lexeme::ChorusMark,
                lit(" Chorus"),
                NL,
                lit("Swing "),
                CS,
                lit("D"),
                CE,
                lit("low, sweet "),
                CS,
                lit("G"),
                CE,
                lit("chari"),
                CS,
                lit("D"),
                CE,
                lit("ot,"),
                NL,
                EOF,
            ]
        );
    }

    #[test]
    fn scan_test_complex_chords() {
        let lexemes = Scanner::new().scan("[Dm7]");
        assert_eq!(lexemes, vec![CS, lit("Dm7"), CE, EOF]);

        let lexemes = Scanner::new().scan("[Dm7/B]");
        assert_eq!(lexemes, vec![CS, lit("Dm7/B"), CE, EOF]);

        let lexemes = Scanner::new().scan("[D#m7]");
        assert_eq!(lexemes, vec![CS, lit("D"), H, lit("m7"), CE, EOF]);

        let lexemes = Scanner::new().scan("[B/F#maj7]");
        assert_eq!(lexemes, vec![CS, lit("B/F"), H, lit("maj7"), CE, EOF]);
    }
}
