use super::validate_xml_identifier;
use crate::error::Result;
use crate::html::escape::Escape;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Ord)]
pub struct Attribute {
    name: String,
    value: String,
}

impl<'a> Attribute {
    pub fn new(name: &'a str, value: &'a str) -> Result<Self> {
        Ok(Self {
            name: validate_xml_identifier(name)?.to_owned(),
            value: value.to_owned(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<'a> Hash for Attribute /*<'a>*/ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> PartialEq for Attribute /*<'a>*/ {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for Attribute /*<'a>*/ {}

impl<'a> PartialOrd for Attribute /*<'a>*/ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name().partial_cmp(other.name())
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}='{}'", self.name(), Escape(self.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn get_hash<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_hash_eq() {
        let attr1 = Attribute::new("the-name", "a value").unwrap();
        let attr2 = Attribute::new("the-name", "another value").unwrap();

        assert_eq!(get_hash(attr1), get_hash(attr2));
    }

    #[test]
    fn test_hash_ne() {
        let attr1 = Attribute::new("the-name", "a value").unwrap();
        let attr2 = Attribute::new("another-name", "another value").unwrap();

        assert_ne!(get_hash(attr1), get_hash(attr2));
    }
}
