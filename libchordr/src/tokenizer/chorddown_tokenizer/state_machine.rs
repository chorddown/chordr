use super::mode::Mode;
use super::scanner::Lexeme;
use crate::tokenizer::{Meta, Modifier, Token};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub(crate) struct FSM {
    state: Mode,
    literal_buffer: String,
    header_level: u8,
    header_modifier: Modifier,
    error: Option<StateError>,
}

impl FSM {
    pub fn new() -> Self {
        Self {
            state: Mode::Bof,
            literal_buffer: String::new(),
            header_level: 0,
            header_modifier: Modifier::None,
            error: None,
        }
    }

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
                    self.error = Some(StateError::UnexpectedChordEnd);

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
                        self.error = Some(StateError::UnclosedChord);
                        Some(Mode::Newline)
                    }
                    Lexeme::ChordStart => {
                        // Nested chord
                        self.append_lexeme(lexeme);
                        self.error = Some(StateError::NestedChord);
                        None
                    }
                    Lexeme::ChordEnd => Some(Mode::Literal),
                    Lexeme::QuoteStart
                    | Lexeme::Colon
                    | Lexeme::ChorusMark
                    | Lexeme::BridgeMark => {
                        self.append_lexeme(lexeme);
                        self.error = Some(StateError::InvalidChordCharacter);
                        None
                    }
                    Lexeme::Literal(_) => {
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::Eof => {
                        self.error = Some(StateError::UnexpectedEndOfFile);
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
                        // Chord inside a header
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::QuoteStart | Lexeme::Colon => {
                        self.append_lexeme(lexeme);
                        None
                    }
                    Lexeme::ChorusMark => {
                        self.header_modifier = Modifier::Chorus;
                        None
                    }
                    Lexeme::BridgeMark => {
                        self.header_modifier = Modifier::Bridge;
                        None
                    }
                    Lexeme::Literal(_) => {
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
                        self.error = Some(StateError::UnexpectedHeaderStart);
                        self.append_lexeme(lexeme);
                        None
                    }

                    Lexeme::ChordStart => Some(Mode::Chord),
                    Lexeme::ChordEnd => {
                        // Chord End without an opening bracket
                        self.error = Some(StateError::UnexpectedChordEnd);

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
                        self.error = Some(StateError::UnexpectedEndOfFile);
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
            self.header_modifier,
        );

        self.header_level = 0;
        self.header_modifier = Modifier::None;
        Some(token)
    }

    fn consume_buffer(&mut self) -> String {
        ::std::mem::replace(&mut self.literal_buffer, String::new())
    }

    fn append_lexeme(&mut self, lexeme: &Lexeme) {
        self.literal_buffer.push_str(lexeme.to_string().as_str())
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
        write!(f, "{}", match self {
            StateError::UnclosedChord => "UnclosedChord",
            StateError::NestedChord => "NestedChord",
            StateError::InvalidChordCharacter => "InvalidChordCharacter",
            StateError::UnexpectedChordEnd => "UnexpectedChordEnd",
            StateError::UnexpectedHeaderStart => "UnexpectedHeaderStart",
            StateError::UnexpectedEndOfFile => "UnexpectedEndOfFile",
        })
    }
}

impl Error for StateError {}
