use std::error::Error;
use xml::reader::Error as XmlError;

/// Response used for listing files.
#[derive(Default, Debug)]
pub struct PropfindResponse {
    /// URL of the resource
    pub href: String,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PropfindParseError {
    UnknownDocument,
    InvalidFieldValue,
    UnknownElement,
    UnknownField,
    ExpectedEndOfDocument,
    Xml(XmlError),
}

impl From<XmlError> for PropfindParseError {
    fn from(e: XmlError) -> Self {
        PropfindParseError::Xml(e)
    }
}

impl std::fmt::Display for PropfindParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let description = match self {
            PropfindParseError::UnknownDocument => "not a propfind response",
            PropfindParseError::InvalidFieldValue => "field must only contain text",
            PropfindParseError::UnknownElement => "document must only contain responses",
            PropfindParseError::UnknownField => "unsupported field",
            PropfindParseError::ExpectedEndOfDocument => "expected end of document",
            PropfindParseError::Xml(ref e) => &e.msg(),
        };
        write!(f, "{}", description)
    }
}

impl Error for PropfindParseError {
    fn cause(&self) -> Option<&dyn Error> {
        use self::PropfindParseError::*;
        match *self {
            Xml(ref e) => Some(e as &dyn Error),
            _ => None,
        }
    }
}
