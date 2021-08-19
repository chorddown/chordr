use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum TagName {
    Div,
    Span,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Section,
    Blockquote,
    Hr,
    None,
}

impl TagName {
    pub fn is_none(&self) -> bool {
        if let TagName::None = self {
            true
        } else {
            false
        }
    }

    pub const fn headline_level(level: u8) -> TagName {
        match level {
            1 => TagName::H1,
            2 => TagName::H2,
            3 => TagName::H3,
            4 => TagName::H4,
            5 => TagName::H5,
            _ => TagName::H6,
        }
    }

    #[inline(always)]
    pub fn as_html_tag_name(&self) -> &'static str {
        match self {
            TagName::Div => "div",
            TagName::Span => "span",
            TagName::Section => "section",
            TagName::Blockquote => "blockquote",
            TagName::H1 => "h1",
            TagName::H2 => "h2",
            TagName::H3 => "h3",
            TagName::H4 => "h4",
            TagName::H5 => "h5",
            TagName::H6 => "h6",
            TagName::None => "",
            TagName::Hr => "hr",
        }
    }
}

impl Display for TagName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let TagName::None = self {
            Ok(())
        } else {
            f.write_str(self.as_html_tag_name())
        }
    }
}
