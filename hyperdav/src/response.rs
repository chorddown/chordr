use std;
use std::error::Error;
use std::io::Read;
use xml::reader::{Error as XmlError, EventReader, XmlEvent};
use xml::ParserConfig;

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
        write!(f, "{}", self.to_string())
    }
}

impl Error for PropfindParseError {
    fn description(&self) -> &str {
        use self::PropfindParseError::*;
        match *self {
            UnknownDocument => "not a propfind response",
            InvalidFieldValue => "field must only contain text",
            UnknownElement => "document must only contain responses",
            UnknownField => "unsupported field",
            ExpectedEndOfDocument => "expected end of document",
            Xml(ref e) => e.msg(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        use self::PropfindParseError::*;
        match *self {
            Xml(ref e) => Some(e as &dyn Error),
            _ => None,
        }
    }
}

pub fn parse_propfind_response<R: Read>(
    read: R,
) -> Result<Vec<PropfindResponse>, PropfindParseError> {
    enum Field {
        Href,
        Ignored,
    }

    enum State {
        Items,
        Item {
            item: PropfindResponse,
            field: Option<Field>,
        },
        Start,
        End,
    }

    let parser = EventReader::new_with_config(
        read,
        ParserConfig {
            trim_whitespace: true,
            cdata_to_characters: true,
            ..Default::default()
        },
    );
    let mut items = Vec::new();
    let mut state = State::Start;

    for e in parser {
        let e = e?;
        state = match state {
            State::Start => match e {
                XmlEvent::StartDocument { .. } => State::Start,
                XmlEvent::StartElement { ref name, .. } if name.local_name == "multistatus" => {
                    State::Items
                }
                _ => return Err(PropfindParseError::UnknownDocument),
            },
            State::End => match e {
                XmlEvent::EndDocument => return Ok(items),
                _ => return Err(PropfindParseError::ExpectedEndOfDocument),
            },
            State::Items => match e {
                XmlEvent::EndElement { .. } => State::End,
                XmlEvent::StartElement { ref name, .. } if name.local_name == "response" => {
                    State::Item {
                        item: PropfindResponse::default(),
                        field: None,
                    }
                }
                _ => return Err(PropfindParseError::UnknownElement),
            },
            State::Item { field: None, item } => {
                match e {
                    XmlEvent::StartElement { name, .. } => match &*name.local_name {
                        "href" => State::Item {
                            field: Some(Field::Href),
                            item,
                        },
                        _ => State::Item {
                            field: Some(Field::Ignored),
                            item,
                        },
                    },
                    XmlEvent::EndElement { name } => {
                        match &*name.local_name {
                            "response" => {
                                items.push(item);
                                State::Items
                            }
                            // ignore nested elements for now
                            _ => State::Item { field: None, item },
                        }
                    }
                    _ => State::Item { field: None, item },
                }
            }
            State::Item {
                field: Some(field),
                mut item,
            } => {
                match e {
                    XmlEvent::Characters(s) => {
                        match field {
                            Field::Href => item.href = s,
                            Field::Ignored => {}
                        };
                        State::Item {
                            field: Some(field),
                            item,
                        }
                    }
                    XmlEvent::EndElement { .. } => State::Item { field: None, item },
                    // ignore nested element for now
                    XmlEvent::StartElement { .. } => State::Item {
                        field: Some(field),
                        item,
                    },
                    _ => return Err(PropfindParseError::InvalidFieldValue),
                }
            }
        }
    }

    Ok(items)
}
