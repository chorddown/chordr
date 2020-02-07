use crate::error::{Error, Result};
use crate::service::file_entry::FileEntry;
use chrono::{DateTime, FixedOffset};
use std::io::prelude::*;
use xml::reader::{EventReader, XmlEvent};
use xml::ParserConfig;

pub fn parse_propfind_response<R: Read>(read: R) -> Result<Vec<TempFileEntry>> {
    #[derive(Debug, PartialEq)]
    enum Field {
        Href,
        LastModified,
        Size,
        ResourceType,
        Ignored,
    }

    #[derive(Debug)]
    enum State {
        Items,
        Item {
            item: TempFileEntry,
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
                _ => {
                    return Err(Error::xml_parser_error(format!(
                        "Unexpected XML event: {:?}",
                        e
                    )))
                }
            },
            State::End => {
                return match e {
                    XmlEvent::EndDocument => Ok(items),
                    _ => Err(Error::xml_parser_error("Expected End Of Document")),
                }
            }
            State::Items => match e {
                XmlEvent::EndElement { .. } => State::End,
                XmlEvent::StartElement { ref name, .. } if name.local_name == "response" => {
                    State::Item {
                        item: TempFileEntry::default(),
                        field: None,
                    }
                }
                _ => return Err(Error::xml_parser_error("Unknown error")),
            },
            State::Item { field: None, item } => match e {
                XmlEvent::StartElement { name, .. } => match &*name.local_name {
                    "href" => State::Item {
                        field: Some(Field::Href),
                        item,
                    },
                    "getlastmodified" => State::Item {
                        field: Some(Field::LastModified),
                        item,
                    },
                    "getcontentlength" | "size" => State::Item {
                        field: Some(Field::Size),
                        item,
                    },
                    "resourcetype" => State::Item {
                        field: Some(Field::ResourceType),
                        item,
                    },
                    _ => State::Item {
                        field: Some(Field::Ignored),
                        item,
                    },
                },
                XmlEvent::EndElement { name } => match &*name.local_name {
                    "response" => {
                        items.push(item);
                        State::Items
                    }
                    _ => State::Item { field: None, item },
                },
                _ => State::Item { field: None, item },
            },
            State::Item {
                field: Some(field),
                mut item,
            } => match e {
                XmlEvent::Characters(s) => {
                    match field {
                        Field::Href => TempFileEntry::set_path(&mut item, s),
                        Field::Size => TempFileEntry::set_size(&mut item, s),
                        Field::LastModified => TempFileEntry::set_modified_date(&mut item, s)?,
                        Field::ResourceType => unreachable!(),
                        Field::Ignored => {}
                    };

                    State::Item {
                        field: Some(field),
                        item,
                    }
                }
                XmlEvent::EndElement { .. } => State::Item { field: None, item },
                XmlEvent::StartElement { name, .. } => {
                    if field == Field::ResourceType && name.local_name == "collection" {
                        TempFileEntry::set_is_directory(&mut item, true);
                    }
                    State::Item {
                        field: Some(field),
                        item,
                    }
                }
                _ => return Err(Error::xml_parser_error("Invalid Field Value")),
            },
        }
    }

    Ok(items)
}

#[derive(Clone, Debug)]
pub struct TempFileEntry {
    is_directory: bool,
    path: String,
    size: usize,
    modified_date: DateTime<FixedOffset>,
}

impl TempFileEntry {
    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    fn set_path(&mut self, path: String) {
        self.path = path
    }

    fn set_size(&mut self, s: String) {
        match s.parse::<usize>() {
            Ok(size) => self.size = size,
            Err(_) => {
                // Do nothing - keep `self.size` as it is
            }
        };
    }

    fn set_modified_date<S: AsRef<str>>(&mut self, date: S) -> Result<()> {
        let parsed_date = DateTime::parse_from_str(date.as_ref(), "%a, %d %b %Y %H:%M:%S GMT")?;
        self.modified_date = parsed_date;
        Ok(())
    }

    fn set_is_directory(&mut self, is_directory: bool) {
        self.is_directory = is_directory
    }
}

impl Default for TempFileEntry {
    fn default() -> Self {
        let date = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap();

        TempFileEntry {
            is_directory: false,
            path: String::new(),
            size: 0,
            modified_date: date,
        }
    }
}

impl From<TempFileEntry> for FileEntry {
    fn from(file: TempFileEntry) -> Self {
        FileEntry::new(file.path, file.size, file.modified_date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_propfind_response_test() {
        let bytes = include_bytes!("../../../tests/resources/webdav_response.xml");
        let result = parse_propfind_response(&bytes[..]);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 3);
        assert_eq!(
            files
                .iter()
                .filter(|f| !f.is_directory)
                .collect::<Vec<&TempFileEntry>>()
                .len(),
            2
        );
        assert_eq!(files[0].path, "/remote.php/webdav/Lyrics/".to_owned());
        assert_eq!(files[0].is_directory, true);
        assert_eq!(files[0].size, 30);

        assert_eq!(
            files[1].path,
            "/remote.php/webdav/Lyrics/amazing-grace.chorddown".to_owned()
        );
        assert_eq!(files[1].is_directory, false);
        assert_eq!(files[1].size, 12);

        assert_eq!(
            files[2].path,
            "/remote.php/webdav/Lyrics/swing-low.chorddown".to_owned()
        );
        assert_eq!(files[2].is_directory, false);
        assert_eq!(files[2].size, 18);
    }
}
