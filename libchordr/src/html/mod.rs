use crate::error::{Error, Result};

pub mod attribute;
pub mod escape;
pub mod tag;
pub mod tag_builder;

fn validate_xml_identifier(content: &str) -> Result<&str> {
    let bad_character_option =
        content.find(|c: char| false == (c.is_alphanumeric() || c == '-' || c == '_'));
    if bad_character_option.is_some() {
        Err(Error::tag_builder_error(
            "XML identifier contain only alphanumeric characters, - and _",
        ))
    } else {
        Ok(content)
    }
}
