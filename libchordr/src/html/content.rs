use std::fmt::{Display, Error, Formatter};

use crate::error::Result;
use crate::html::tag::Tag;

use super::escape::Escape;

#[derive(Clone, Debug)]
pub enum Content {
    None,
    Some(String),
    Tag(Box<Tag>),
    Raw(String),
}

impl Content {
    pub(crate) fn from_string<S: Into<String>>(content: S) -> Content {
        Content::Some(content.into())
    }

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

impl From<Tag> for Content {
    fn from(t: Tag) -> Self {
        Content::Tag(Box::new(t))
    }
}
