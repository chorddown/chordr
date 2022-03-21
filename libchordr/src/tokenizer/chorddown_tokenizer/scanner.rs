use std::io::BufRead;

use crate::error::Error;
use crate::tokenizer::chorddown_tokenizer::lexeme::Lexeme;

use super::keywords::{
    BRIDGE_MARK, CHORD_END, CHORD_START, CHORUS_MARK, COLON, HEADER_START, NEWLINE, QUOTE_START,
};

const LITERAL_BUFFER_CAPACITY: usize = 20;

pub struct Scanner {
    lexemes: Vec<Lexeme>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            lexemes: Vec::with_capacity(400),
        }
    }

    pub fn scan<R: BufRead>(mut self, mut input: R) -> Result<Vec<Lexeme>, Error> {
        let mut literal_buffer = String::with_capacity(LITERAL_BUFFER_CAPACITY);

        let mut line = String::new();
        while 0 < input.read_line(&mut line)? {
            log::trace!("Scan line `{}`", line.trim_end());
            let chars = line.chars();

            for current_character in chars {
                match current_character {
                    NEWLINE => self.build_n_push(&mut literal_buffer, Lexeme::Newline),
                    CHORD_START => self.build_n_push(&mut literal_buffer, Lexeme::ChordStart),
                    CHORD_END => self.build_n_push(&mut literal_buffer, Lexeme::ChordEnd),
                    HEADER_START => self.build_n_push(&mut literal_buffer, Lexeme::HeaderStart),
                    QUOTE_START => self.build_n_push(&mut literal_buffer, Lexeme::QuoteStart),
                    COLON => self.build_n_push(&mut literal_buffer, Lexeme::Colon),
                    CHORUS_MARK => self.build_n_push(&mut literal_buffer, Lexeme::ChorusMark),
                    BRIDGE_MARK => self.build_n_push(&mut literal_buffer, Lexeme::BridgeMark),
                    _ => literal_buffer.push(current_character),
                }
            }

            line.clear()
        }

        self.build_n_push(&mut literal_buffer, Lexeme::Eof);

        Ok(self.lexemes)
    }

    /// Call `build_n_drain()` with `literal_buffer` and append the given `lexeme` the collection
    #[inline(always)]
    fn build_n_push(&mut self, literal_buffer: &mut String, lexeme: Lexeme) {
        self.build_n_drain(literal_buffer);

        self.lexemes.push(lexeme)
    }

    /// Build a `Lexeme::Literal` from the current content of `literal_buffer`, push it to the
    /// lexemes and replace the buffer with an empty string
    #[inline(always)]
    fn build_n_drain(&mut self, literal_buffer: &mut String) {
        if !literal_buffer.is_empty() {
            self.lexemes.push(Lexeme::Literal(std::mem::replace(
                literal_buffer,
                String::with_capacity(LITERAL_BUFFER_CAPACITY),
            )));
        }
    }
}

mod shortcuts {
    #[allow(unused_imports)]
    pub(super) use Lexeme::BridgeMark as B;
    #[allow(unused_imports)]
    pub(super) use Lexeme::ChordEnd as CE;
    #[allow(unused_imports)]
    pub(super) use Lexeme::ChordStart as CS;
    #[allow(unused_imports)]
    pub(super) use Lexeme::ChorusMark as CM;
    #[allow(unused_imports)]
    pub(super) use Lexeme::Eof as EOF;
    #[allow(unused_imports)]
    pub(super) use Lexeme::HeaderStart as H;
    #[allow(unused_imports)]
    pub(super) use Lexeme::Newline as NL;

    use super::Lexeme;

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
        let lexemes = scanner.scan(content.as_bytes()).unwrap();
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
        let lexemes = Scanner::new().scan("[Dm7]".as_bytes()).unwrap();
        assert_eq!(lexemes, vec![CS, lit("Dm7"), CE, EOF]);

        let lexemes = Scanner::new().scan("[Dm7/B]".as_bytes()).unwrap();
        assert_eq!(lexemes, vec![CS, lit("Dm7/B"), CE, EOF]);

        let lexemes = Scanner::new().scan("[D#m7]".as_bytes()).unwrap();
        assert_eq!(lexemes, vec![CS, lit("D"), H, lit("m7"), CE, EOF]);

        let lexemes = Scanner::new().scan("[B/F#maj7]".as_bytes()).unwrap();
        assert_eq!(lexemes, vec![CS, lit("B/F"), H, lit("maj7"), CE, EOF]);
    }

    #[test]
    fn scan_pre_chorus_test() {
        let content = r"
##- Pre-chorus
";
        let scanner = Scanner::new();
        let lexemes = scanner.scan(content.as_bytes()).unwrap();
        assert_eq!(lexemes.len(), 9);

        assert_eq!(
            lexemes,
            vec![NL, H, H, B, lit(" Pre"), B, lit("chorus"), NL, EOF]
        );
    }

    #[test]
    fn scan_bride_with_exclamation_marks_test() {
        let content = r"##- Bride Loud!!";
        let scanner = Scanner::new();
        let lexemes = scanner.scan(content.as_bytes()).unwrap();
        // assert_eq!(lexemes.len(), 7);

        assert_eq!(lexemes, vec![H, H, B, lit(" Bride Loud"), CM, CM, EOF,]);
    }

    #[test]
    fn scan_chorus_with_exclamation_marks_test() {
        let content = r"##! Chorus Loud!!";
        let scanner = Scanner::new();
        let lexemes = scanner.scan(content.as_bytes()).unwrap();
        assert_eq!(lexemes.len(), 7);

        assert_eq!(lexemes, vec![H, H, CM, lit(" Chorus Loud"), CM, CM, EOF,]);
    }
}
