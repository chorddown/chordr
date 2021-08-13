use std::fmt::{Display, Error, Formatter};

use crate::error::Result;

use super::escape::Escape;
use crate::html::tag::Tag;

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
