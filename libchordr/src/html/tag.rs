use super::attribute::Attribute;
use super::escape::Escape;
use super::validate_xml_identifier;
use crate::error::Result;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug)]
pub enum Content {
    None,
    Some(String),
    Tag(Box<Tag /*<'a>*/>),
    Raw(String),
}

impl Content {
    pub fn is_empty(&self) -> bool {
        match self {
            Content::None => true,
            Content::Some(s) => s.is_empty(),
            Content::Raw(s) => s.is_empty(),
            Content::Tag(_) => false,
        }
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Content::Some(c) => write!(f, "{}", Escape(c)),
            Content::None => Ok(()),
            Content::Raw(c) => write!(f, "{}", c),
            Content::Tag(t) => write!(f, "{}", *t),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tag /*<'a>*/ {
    blank: bool,
    tag_name: Option<String>,
    content: Content,
    attributes: Option<HashSet<Attribute /*<'a>*/>>,
}

impl<'a> Tag /*<'a>*/ {
    pub fn new<S: Into<String>>(
        tag_name: S,
        content: Content,
        attributes: Option<HashSet<Attribute /*<'a>*/>>,
    ) -> Self {
        let tag_name_string = tag_name.into();
        match validate_xml_identifier(&tag_name_string) {
            Err(e) => panic!(e.to_string()),
            Ok(_) => {}
        }
        Self {
            tag_name: Some(tag_name_string),
            content,
            attributes,
            blank: false,
        }
    }
    pub fn raw(content: Content) -> Self {
        Self {
            tag_name: None,
            content,
            attributes: Default::default(),
            blank: false,
        }
    }

    /// Build a new Tag instance that will have an empty tag name
    ///
    /// (like a Fragment in React)
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self::new("".to_owned(), Content::None, None)
    }

    /// Build a new Tag instance that will render as an empty string
    pub fn blank() -> Self {
        Self {
            tag_name: None,
            content: Content::None,
            attributes: None,
            blank: true,
        }
    }

    pub fn with_name(tag_name: &'a str) -> Self {
        Self::new(tag_name.to_owned(), Content::None, None)
    }

    #[allow(dead_code)]
    pub fn text_node(content: &'a str) -> Self {
        Self::new("".to_owned(), Content::Some(content.into()), None)
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn is_self_closing(&self) -> bool {
        self.content.is_empty() && !self.has_attributes()
    }

    pub fn is_blank(&self) -> bool {
        self.blank
    }

    pub fn has_attributes(&self) -> bool {
        self.attributes.is_some()
    }

    pub fn tag_name(&self) -> Option<String> {
        match self.tag_name {
            Some(ref t) => Some(t.clone()),
            None => None,
        }
    }

    pub fn is_raw_wrapper(&self) -> bool {
        if self.tag_name.is_none() || self.tag_name().unwrap().is_empty() {
            return !self.content.is_empty();
        } else {
            return false;
        }
    }
}

impl<'a> Display for Tag /*<'a>*/ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.is_blank() {
            return write!(f, "");
        }

        if self.has_attributes() && self.tag_name().is_none() {
            panic!("Attributes without tag name");
        }

        if self.is_raw_wrapper() {
            write!(f, "{}", self.content)
        } else if self.is_self_closing() {
            // if self.tag_name() == None || self.tag_name().unwrap().is_empty() {
            //     panic!("No tag name provided");
            // }

            write!(f, "<{}/>", self.tag_name().unwrap())
        } else {
            let tag_name = self.tag_name().unwrap();
            let mut node = String::new();
            if !tag_name.is_empty() {
                node = format!("<{}", tag_name);
            }

            if let Some(attributes) = &self.attributes {
                let mut attributes_sorted = attributes.iter().collect::<Vec<_>>();
                attributes_sorted.sort();
                node.push_str(&format!(
                    " {}",
                    attributes_sorted
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ));
            }

            node.push_str(&format!(">{}", self.content));
            node.push_str(&format!("</{}>", tag_name));

            write!(f, "{}", node)
        }
    }
}
