#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Mode {
    Chord = 8,
    Header = 4,
    Newline = 10,
    Quote = 6,
    Literal = 0,
}

const NEWLINE: char = '\n';
const CHORD_START: char = '[';
const CHORD_END: char = ']';
const HEADER_START: char = '#';
const HEADER_END: char = NEWLINE;
const QUOTE_START: char = '>';
const QUOTE_END: char = NEWLINE;

pub(crate) trait ModePartner {
    fn is_end_of(&self, mode: Mode) -> bool;
    fn is_signal(&self) -> bool;
    fn is_terminator(&self, mode: Mode) -> bool;
}

impl ModePartner for char {
    fn is_end_of(&self, mode: Mode) -> bool {
        match mode {
            Mode::Chord => self == &CHORD_END,
            Mode::Header => self == &HEADER_END,
            Mode::Newline => unreachable!(),
            Mode::Quote => self == &QUOTE_END,
            Mode::Literal => self == &NEWLINE,
        }
    }

    #[allow(unreachable_patterns)]
    fn is_signal(&self) -> bool {
        match *self {
            NEWLINE => true,
            CHORD_START => true,
            CHORD_END => true,
            HEADER_START => true,
            HEADER_END => true,
            QUOTE_START => true,
            QUOTE_END => true,
            _ => false,
        }
    }

    fn is_terminator(&self, mode: Mode) -> bool {
        self.is_end_of(mode) || mode.is_terminated_by(&self.into())
    }
}

impl Mode {
    pub fn is_self_closing(&self) -> bool {
        match self {
            Mode::Newline => true,
            _ => false
        }
    }

    pub fn is_terminated_by_char(&self, character: char) -> bool {
        character.is_end_of(*self) || self.is_terminated_by(&character.into())
    }

    pub fn from_char(character: char) -> Self {
        match character {
            QUOTE_START => Mode::Quote,
            HEADER_START => Mode::Header,
            CHORD_START => Mode::Chord,
            NEWLINE => Mode::Newline,
            _ => Mode::Literal,
        }
    }

    fn is_terminated_by(&self, mode: &Mode) -> bool {
        self < mode
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
