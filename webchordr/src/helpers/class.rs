use std::fmt::{Display, Error, Formatter};
use yew::html::IntoPropValue;
use yew::Classes;

#[derive(Clone, Debug, PartialEq)]
pub struct Class(String);

impl Class {
    pub fn new<S: Into<String>>(class: S) -> Self {
        Self(class.into())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn or(&self, default: &str) -> Self {
        if self.is_empty() {
            Self::from(default)
        } else {
            self.clone()
        }
    }

    /// Return a new class with the given string appended
    pub fn add(&self, class: &str) -> Self {
        Self(format!("{} {}", self.0, class))
    }

    /// Return a new class with the given string appended
    pub fn append(&mut self, class: &str) {
        self.0.push(' ');
        self.0.push_str(class);
    }
}

impl Default for Class {
    fn default() -> Self {
        Self("".to_owned())
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

// -------------------------
// Create instances of Class
impl From<&str> for Class {
    fn from(string: &str) -> Self {
        Self(string.to_owned())
    }
}

impl From<String> for Class {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl From<&String> for Class {
    fn from(string: &String) -> Self {
        Self(string.clone())
    }
}

// --------------------------------------------------
// Transform an instance of Class into something else
impl From<Class> for Classes {
    fn from(c: Class) -> Self {
        Classes::from(c.0)
    }
}

impl IntoPropValue<Class> for &str {
    fn into_prop_value(self) -> Class {
        Class::from(self)
    }
}
