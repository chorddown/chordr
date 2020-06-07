use std::fmt::{Display, Error, Formatter};
use yew::Classes;

#[derive(Clone, Debug, PartialEq)]
pub struct Class(String);

impl Class {
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
        Classes::from(c.0.as_str())
    }
}

impl yew::virtual_dom::Transformer<&str, Class> for Class {
    fn transform(from: &str) -> Class {
        Class::from(from)
    }
}

impl yew::virtual_dom::Transformer<&str, Class> for yew::virtual_dom::vcomp::VComp {
    fn transform(from: &str) -> Class {
        Class::from(from)
    }
}

//impl From<Class> for yew::virtual_dom::Transformer<&str, helpers::class::Class>` is not implemented for `yew::virtual_dom::vcomp::VComp
