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
        use self::PropfindParseError::*;
        if let Xml(e) = self {
            f.write_str(&e.msg())
        } else {
            f.write_str(match *self {
                UnknownDocument => "not a propfind response",
                InvalidFieldValue => "field must only contain text",
                UnknownElement => "document must only contain responses",
                UnknownField => "unsupported field",
                ExpectedEndOfDocument => "expected end of document",
                Xml(_) => unreachable!(),
            })
        }
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
