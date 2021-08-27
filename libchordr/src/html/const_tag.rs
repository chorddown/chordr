use std::fmt::{Display, Error, Formatter};

use crate::error::Result;
use crate::html::content::Content;
use crate::html::tag_name::TagName;

#[derive(Clone, Debug)]
pub struct ConstTag {
    blank: bool,
    tag_name: TagName,
    content: Content,
    class_name: Option<&'static str>,
    data_chord: Option<String>,
}

impl ConstTag {
    pub(crate) const fn headline(
        level: u8,
        content: Content,
        class_name: Option<&'static str>,
    ) -> ConstTag {
        Self {
            blank: false,
            tag_name: TagName::headline_level(level),
            content,
            class_name,
            data_chord: None,
        }
    }

    pub(crate) const fn div(content: Content, class_name: Option<&'static str>) -> Self {
        Self {
            blank: false,
            tag_name: TagName::Div,
            content,
            class_name,
            data_chord: None,
        }
    }

    pub(crate) const fn span(content: Content, class_name: Option<&'static str>) -> Self {
        Self {
            blank: false,
            tag_name: TagName::Span,
            content,
            class_name,
            data_chord: None,
        }
    }

    pub(crate) const fn span_with_chord(
        content: Content,
        class_name: Option<&'static str>,
        data_chord: String,
    ) -> Self {
        Self {
            blank: false,
            tag_name: TagName::Span,
            content,
            class_name,
            data_chord: Some(data_chord),
        }
    }

    pub(crate) const fn hr() -> Self {
        Self {
            blank: false,
            tag_name: TagName::Hr,
            content: Content::None,
            class_name: None,
            data_chord: None,
        }
    }

    #[allow(unused)]
    pub(crate) const fn section(content: Content, class_name: Option<&'static str>) -> Self {
        Self {
            blank: false,
            tag_name: TagName::Section,
            content,
            class_name,
            data_chord: None,
        }
    }

    pub(crate) const fn blockquote(content: Content, class_name: Option<&'static str>) -> Self {
        Self {
            blank: false,
            tag_name: TagName::Blockquote,
            content,
            class_name,
            data_chord: None,
        }
    }

    #[allow(unused)]
    pub(crate) const fn new(tag_name: TagName, content: Content) -> Self {
        Self {
            tag_name,
            content,
            blank: false,
            class_name: None,
            data_chord: None,
        }
    }

    pub(crate) const fn raw(content: String) -> Self {
        Self {
            tag_name: TagName::None,
            content: Content::Raw(content),
            blank: false,
            class_name: None,
            data_chord: None,
        }
    }

    /// Build a new Tag instance that will render as an empty string
    pub(crate) const fn blank() -> Self {
        Self {
            tag_name: TagName::None,
            content: Content::None,
            blank: true,
            class_name: None,
            data_chord: None,
        }
    }

    pub(crate) const fn content(&self) -> &Content {
        &self.content
    }

    pub(crate) const fn is_self_closing(&self) -> bool {
        matches!(self.tag_name, TagName::Hr)
    }

    pub(crate) const fn tag_name(&self) -> &TagName {
        &self.tag_name
    }

    pub(crate) const fn is_blank(&self) -> bool {
        self.blank
    }

    pub fn is_raw_wrapper(&self) -> bool {
        self.tag_name.is_none() && !self.content.is_empty()
    }
}

impl Display for ConstTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.is_blank() {
            return Ok(());
        }

        if self.is_raw_wrapper() {
            write!(f, "{}", self.content)
        } else if self.is_self_closing() {
            f.write_str("<")?;
            f.write_str(self.tag_name.as_html_tag_name())?;
            f.write_str("/>")
        } else {
            let tag_name_string = self.tag_name.as_html_tag_name();
            f.write_str("<")?;
            f.write_str(tag_name_string)?;

            if let Some(class_name) = self.class_name {
                f.write_str(" class='")?;
                f.write_str(class_name)?;
                f.write_str("'")?;
            }

            if let Some(data_chord) = &self.data_chord {
                f.write_str(" data-chord='")?;
                f.write_str(&data_chord)?;
                f.write_str("'")?;
            }

            f.write_str(">")?;
            f.write_str(&self.content.to_string())?;
            f.write_str("</")?;
            f.write_str(tag_name_string)?;
            f.write_str(">")?;
            Ok(())
        }
    }
}
