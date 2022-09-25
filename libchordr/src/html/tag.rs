use std::fmt::{Display, Error, Formatter};

use crate::error::Result;
use crate::html::attribute::AttributeCollection;
use crate::html::const_tag::ConstTag;
use crate::html::content::Content;
use crate::html::dynamic_tag::DynamicTag;
use crate::html::tag_name::TagName;

#[derive(Clone, Debug)]
pub enum Tag {
    Dynamic(DynamicTag),
    Const(ConstTag),
}

impl Tag {
    pub fn new<S: Into<String>>(
        tag_name: S,
        content: Content,
        attributes: Option<AttributeCollection>,
    ) -> Self {
        Self::Dynamic(DynamicTag::new(tag_name, content, attributes))
    }

    pub const fn raw(content: String) -> Self {
        Self::Const(ConstTag::raw(content))
    }

    /// Build a new Tag instance that will render as an empty string
    pub const fn blank() -> Self {
        Self::Const(ConstTag::blank())
    }

    pub(crate) const fn headline(
        level: u8,
        content: Content,
        class_name: Option<&'static str>,
    ) -> Self {
        Self::Const(ConstTag::headline(level, content, class_name))
    }

    pub(crate) const fn div(content: Content, class_name: Option<&'static str>) -> Self {
        Self::Const(ConstTag::div(content, class_name))
    }

    pub(crate) const fn span(content: Content, class_name: Option<&'static str>) -> Self {
        Self::Const(ConstTag::span(content, class_name))
    }

    pub(crate) const fn span_with_chord(
        content: Content,
        class_name: Option<&'static str>,
        data_chord: String,
    ) -> Self {
        Self::Const(ConstTag::span_with_chord(content, class_name, data_chord))
    }

    pub(crate) const fn hr() -> Self {
        Self::Const(ConstTag::hr())
    }

    pub(crate) const fn blockquote(content: Content, class_name: Option<&'static str>) -> Self {
        Self::Const(ConstTag::blockquote(content, class_name))
    }

    pub fn content(&self) -> &Content {
        match self {
            Tag::Dynamic(t) => t.content(),
            Tag::Const(t) => t.content(),
        }
    }

    #[allow(unused)]
    pub fn is_self_closing(&self) -> bool {
        match self {
            Tag::Dynamic(t) => t.is_self_closing(),
            Tag::Const(_) => false,
        }
    }

    pub fn is_blank(&self) -> bool {
        match self {
            Tag::Dynamic(t) => t.is_blank(),
            Tag::Const(t) => t.is_blank(),
        }
    }

    #[allow(unused)]
    pub fn has_attributes(&self) -> bool {
        match self {
            Tag::Dynamic(t) => t.has_attributes(),
            Tag::Const(_) => false,
        }
    }

    #[allow(unused)]
    pub fn tag_name(&self) -> Option<String> {
        match self {
            Tag::Dynamic(t) => t.tag_name(),
            Tag::Const(t) => {
                if let TagName::None = t.tag_name() {
                    None
                } else {
                    Some(t.tag_name().to_string())
                }
            }
        }
    }

    #[allow(unused)]
    pub fn is_raw_wrapper(&self) -> bool {
        match self {
            Tag::Dynamic(t) => t.is_raw_wrapper(),
            Tag::Const(t) => t.is_raw_wrapper(),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Tag::Dynamic(t) => f.write_str(&t.to_string()),
            Tag::Const(t) => f.write_str(&t.to_string()),
        }
    }
}

impl From<ConstTag> for Tag {
    fn from(t: ConstTag) -> Self {
        Self::Const(t)
    }
}

impl From<DynamicTag> for Tag {
    fn from(t: DynamicTag) -> Self {
        Self::Dynamic(t)
    }
}
