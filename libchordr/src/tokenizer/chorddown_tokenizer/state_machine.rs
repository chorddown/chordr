use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use crate::tokenizer::{Meta, Modifier, Token};

use super::lexeme::Lexeme;
use super::mode::Mode;

pub(crate) struct FSM {
    state: Mode,
    literal_buffer: String,
    header_level: u8,
    header_modifier: Option<Modifier>,
    warnings: Vec<StateError>,
}

impl FSM {
    pub fn new() -> Self {
        Self {
            state: Mode::Bof,
            literal_buffer: String::new(),
            header_level: 0,
            header_modifier: None,
            warnings: vec![],
        }
    }

    /// Return the [Mode] that is signaled by `lexeme` if it did change
    ///
    /// `None` is returned if there is no change to the current [Mode] (=`self.state`)
    pub fn characterize_lexeme(&mut self, lexeme: &Lexeme) -> Option<Mode> {
        match self.state {
            Mode::Bof | Mode::Newline => match lexeme {
                Lexeme::HeaderStart => {
                    self.header_level = 1;
                    Some(Mode::Header)
                }
                Lexeme::Newline => Some(Mode::Newline),
                Lexeme::ChordStart => Some(Mode::Chord),
                Lexeme::ChordEnd => {
                    self.warnings.push(StateError::UnexpectedChordEnd);

                    Some(Mode::Literal)
                }
                Lexeme::QuoteStart => Some(Mode::Quote),
                Lexeme::Colon | Lexeme::ChorusMark | Lexeme::BridgeMark | Lexeme::Literal(_) => {
                    self.append_lexeme(lexeme);
                    Some(Mode::Literal)
                }
                Lexeme::Eof => FSM::build_eof(),
            },

            Mode::Chord => {
                match lexeme {
                    Lexeme::HeaderStart => {
                        // This denotes a sharp inside the chord
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::Newline => {
                        // Unclosed chord
                        self.warnings.push(StateError::UnclosedChord);
                        Some(Mode::Newline)
                    }
                    Lexeme::ChordStart => {
                        // Nested chord
                        self.append_lexeme(lexeme);
                        self.warnings.push(StateError::NestedChord);
                        None
                    }
                    Lexeme::ChordEnd => Some(Mode::Literal),
                    Lexeme::QuoteStart
                    | Lexeme::Colon
                    | Lexeme::ChorusMark
                    | Lexeme::BridgeMark => {
                        self.append_lexeme(lexeme);
                        self.warnings.push(StateError::InvalidChordCharacter);
                        None
                    }
                    Lexeme::Literal(_) => {
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::Eof => {
                        self.warnings.push(StateError::UnexpectedEndOfFile);
                        FSM::build_eof()
                    }
                }
            }
            Mode::Header => {
                match lexeme {
                    Lexeme::HeaderStart => {
                        self.header_level += 1;
                        None
                    }
                    Lexeme::Newline => Some(Mode::Newline),
                    Lexeme::ChordStart | Lexeme::ChordEnd => {
                        self.set_header_modifier_for_lexeme(lexeme);
                        // Chord inside a header
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::QuoteStart | Lexeme::Colon => {
                        self.set_header_modifier_for_lexeme(lexeme);
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::ChorusMark => {
                        if self.header_modifier.is_some() {
                            // A header modifier has already been detected -> append this lexeme
                            self.append_lexeme(lexeme);
                        } else {
                            self.set_header_modifier_for_lexeme(lexeme);
                        }
                        None
                    }
                    Lexeme::BridgeMark => {
                        if self.header_modifier.is_some() {
                            // A header modifier has already been detected -> append this lexeme
                            self.append_lexeme(lexeme);
                        } else {
                            self.set_header_modifier_for_lexeme(lexeme);
                        }
                        None
                    }
                    Lexeme::Literal(_) => {
                        self.set_header_modifier_for_lexeme(lexeme);
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::Eof => FSM::build_eof(),
                }
            }
            Mode::Quote => match lexeme {
                Lexeme::Newline => Some(Mode::Newline),
                Lexeme::HeaderStart
                | Lexeme::ChordStart
                | Lexeme::ChordEnd
                | Lexeme::QuoteStart
                | Lexeme::Colon
                | Lexeme::ChorusMark
                | Lexeme::BridgeMark
                | Lexeme::Literal(_) => {
                    self.append_lexeme(lexeme);
                    None
                }
                Lexeme::Eof => FSM::build_eof(),
            },
            Mode::Literal => {
                match lexeme {
                    Lexeme::Newline => Some(Mode::Newline),
                    Lexeme::HeaderStart => {
                        self.warnings.push(StateError::UnexpectedHeaderStart);
                        self.append_lexeme(lexeme);
                        None
                    }

                    Lexeme::ChordStart => Some(Mode::Chord),
                    Lexeme::ChordEnd => {
                        // Chord End without an opening bracket
                        self.warnings.push(StateError::UnexpectedChordEnd);

                        None
                    }
                    Lexeme::QuoteStart
                    | Lexeme::Colon
                    | Lexeme::ChorusMark
                    | Lexeme::BridgeMark
                    | Lexeme::Literal(_) => {
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::Eof => {
                        self.warnings.push(StateError::UnexpectedEndOfFile);
                        FSM::build_eof()
                    }
                }
            }
            Mode::Eof => unreachable!(),
        }
    }

    fn build_eof() -> Option<Mode> {
        Some(Mode::Eof)
    }

    pub fn build_token(&mut self) -> Option<Token> {
        match self.state {
            Mode::Header => self.build_token_from_header(),
            Mode::Chord => Some(Token::chord(self.consume_buffer())),
            Mode::Newline => Some(Token::newline()),
            Mode::Quote => Some(Token::quote(self.consume_buffer().trim_start())),
            Mode::Literal => self.build_token_from_literal(),
            Mode::Bof => None,
            Mode::Eof => unreachable!(),
        }
    }

    pub fn set_state(&mut self, state: Mode) {
        self.state = state
    }

    fn build_token_from_header(&mut self) -> Option<Token> {
        let token = Token::headline(
            self.header_level,
            self.consume_buffer().trim_start(),
            self.header_modifier.unwrap_or(Modifier::None),
        );

        self.header_level = 0;
        self.header_modifier = None;
        Some(token)
    }

    fn consume_buffer(&mut self) -> String {
        std::mem::take(&mut self.literal_buffer)
    }

    fn append_lexeme(&mut self, lexeme: &Lexeme) {
        match lexeme {
            Lexeme::Literal(inner) => self.literal_buffer.push_str(inner),
            Lexeme::Eof => {}
            _ => self.literal_buffer.push(lexeme.as_char()),
        }
    }

    fn build_token_from_literal(&mut self) -> Option<Token> {
        let literal = self.consume_buffer();
        if literal.is_empty() {
            return None;
        }
        match Meta::try_from(&literal) {
            Ok(meta) => Some(Token::Meta(meta)),
            Err(_) => Some(Token::Literal(literal)),
        }
    }

    fn set_header_modifier_for_lexeme(&mut self, lexeme: &Lexeme) {
        if self.header_modifier.is_none() {
            if let Some(v) = lexeme.detect_modifier() {
                self.header_modifier = Some(v)
            }
        }
    }
}

#[derive(Debug)]
enum StateError {
    UnclosedChord,
    NestedChord,
    InvalidChordCharacter,
    UnexpectedChordEnd,
    UnexpectedHeaderStart,
    UnexpectedEndOfFile,
}

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StateError::UnclosedChord => f.write_str("UnclosedChord"),
            StateError::NestedChord => f.write_str("NestedChord"),
            StateError::InvalidChordCharacter => f.write_str("InvalidChordCharacter"),
            StateError::UnexpectedChordEnd => f.write_str("UnexpectedChordEnd"),
            StateError::UnexpectedHeaderStart => f.write_str("UnexpectedHeaderStart"),
            StateError::UnexpectedEndOfFile => f.write_str("UnexpectedEndOfFile"),
        }
    }
}

impl Error for StateError {}
