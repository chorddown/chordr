use std::fmt::{Display, Error, Formatter};

use crate::error::Result;
use crate::html::attribute::AttributeCollection;
use crate::html::content::Content;

use super::validate_xml_identifier;

#[derive(Clone, Debug)]
pub struct DynamicTag /*<'a>*/ {
    blank: bool,
    tag_name: Option<String>,
    content: Content,
    attributes: Option<AttributeCollection>,
}

impl<'a> DynamicTag /*<'a>*/ {
    pub fn new<S: Into<String>>(
        tag_name: S,
        content: Content,
        attributes: Option<AttributeCollection>,
    ) -> Self {
        let tag_name_string = tag_name.into();
        if let Err(e) = validate_xml_identifier(&tag_name_string) {
            panic!("{}", e.to_string())
        }
        Self {
            tag_name: Some(tag_name_string),
            content,
            attributes,
            blank: false,
        }
    }

    /// Build a new Tag instance that will render as an empty string
    #[allow(unused)]
    pub const fn blank() -> Self {
        Self {
            tag_name: None,
            content: Content::None,
            attributes: None,
            blank: true,
        }
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
        self.tag_name.as_ref().cloned()
    }

    pub fn is_raw_wrapper(&self) -> bool {
        if self.tag_name.is_none() || self.tag_name().unwrap().is_empty() {
            !self.content.is_empty()
        } else {
            false
        }
    }
}

impl<'a> Display for DynamicTag /*<'a>*/ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.is_blank() {
            return Ok(());
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
            if !tag_name.is_empty() {
                f.write_str("<")?;
                f.write_str(&tag_name)?;
            }

            if let Some(attributes) = &self.attributes {
                let mut attributes_sorted = attributes.iter().collect::<Vec<_>>();
                attributes_sorted.sort();
                for attribute in attributes_sorted {
                    f.write_str(" ")?;
                    f.write_str(&attribute.to_string())?;
                }
            }

            f.write_str(">")?;
            f.write_str(&self.content.to_string())?;
            f.write_str("</")?;
            f.write_str(&tag_name)?;
            f.write_str(">")?;
            Ok(())
        }
    }
}
