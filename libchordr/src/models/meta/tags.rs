use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::slice::Iter;
use std::str::FromStr;

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Clone, Debug, Hash, Ord)]
pub struct Tag(String);

impl Tag {
    pub(crate) fn new<S: Into<String>>(content: S) -> Self {
        Tag(content.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_string_without_hashtag(&self) -> String {
        self.0.clone()
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Tag {
    type Err = TagError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized_tag = value.trim().trim_matches('#');
        if normalized_tag.is_empty() {
            return Err(TagError::Empty);
        }

        if normalized_tag.contains('#') {
            return Err(TagError::InnerHashTag);
        }

        return Ok(Tag(normalized_tag.to_owned()));
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "#{}", self.0)
    }
}

#[derive(Deserialize, Serialize, PartialEq, PartialOrd, Clone, Debug)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn iter(&self) -> Iter<Tag> {
        self.0.iter()
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self(vec![])
    }
}

impl From<Vec<Tag>> for Tags {
    fn from(value: Vec<Tag>) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Tags {
    type Error = TagError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for Tags {
    type Err = TagError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.split('#').collect::<Vec<&str>>();
        let length = parts.len();
        let mut tags = Vec::with_capacity(length);

        for part in parts {
            if let Ok(tag) = Tag::from_str(part) {
                tags.push(tag)
            }
        }

        Ok(Tags(tags))
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(
            self.0
                .iter()
                .map(|t: &Tag| t.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .as_str(),
        )
    }
}

impl IntoIterator for Tags {
    type Item = Tag;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq)]
pub enum TagError {
    Empty,
    InnerHashTag,
}

impl std::error::Error for TagError {}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(match self {
            Self::Empty => "Empty tag",
            Self::InnerHashTag => "Tag must not contain a '#' character",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_from_string() {
        assert_eq!(Ok(Tag("Nice".to_owned())), Tag::from_str("Nice"));
        assert_eq!(Ok(Tag("Nice".to_owned())), Tag::from_str("#Nice"));
        assert_eq!(
            Ok(Tag("Amazing Grace".to_owned())),
            Tag::from_str("Amazing Grace")
        );
        assert_eq!(
            Ok(Tag("Amazing Grace".to_owned())),
            Tag::from_str("#Amazing Grace")
        );
    }

    #[test]
    fn tag_from_string_w_err() {
        assert_eq!(Err(TagError::Empty), Tag::from_str(""));
        assert_eq!(Err(TagError::Empty), Tag::from_str("#"));
        assert_eq!(Err(TagError::InnerHashTag), Tag::from_str("Inner#ash"));
    }
}
