#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    Chord,
    Directive,
    Newline,
    Comment,
    Literal,
}

const NEWLINE: char = '\n';
const CHORD_START: char = '[';
const CHORD_END: char = ']';
const DIRECTIVE_START: char = '{';
const DIRECTIVE_END: char = '}';
const COMMENT_START: char = '#';
const COMMENT_END: char = NEWLINE;

impl Mode {
    pub fn is_self_closing(&self) -> bool {
        match self {
            Mode::Newline => true,
            _ => false
        }
    }

    pub fn is_end_character(self, character: char) -> bool {
        if self == Mode::Literal  && character == NEWLINE{
            true
        } else if self == Mode::Literal {
            false
        } else if self == Mode::Comment && character == COMMENT_END {
            true
        } else if self == Mode::Directive && character == DIRECTIVE_END {
            true
        } else if self == Mode::Chord && character == CHORD_END {
            true
        } else {
            false
        }
    }

    pub fn from_char(character: char) -> Self {
        if character == COMMENT_START {
            // Start of a comment
            Mode::Comment
        } else if character == DIRECTIVE_START {
            // Start of a directive
            Mode::Directive
        } else if character == CHORD_START {
            // Start of a chord
            Mode::Chord
        } else if character == NEWLINE {
            Mode::Newline
        } else {
            Mode::Literal
        }
    }
}
