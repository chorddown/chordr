use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum SectionIdentifierError {
    Empty,
    UnsupportedInput,
}

impl Display for SectionIdentifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SectionIdentifierError::Empty => f.write_str("Section Identifier must not be empty"),
            SectionIdentifierError::UnsupportedInput => {
                f.write_str("Can not create Section Identifier from given input")
            }
        }
    }
}

impl Error for SectionIdentifierError {}
