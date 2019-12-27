use crate::html::attribute::Attribute;
use crate::html::tag::{Content, Tag};
use crate::html::validate_xml_identifier;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TagBuilder<'a> {
    tag_name: &'a str,
    content: Content,
    attributes: HashSet<Attribute /*<'a>*/>,
}

impl<'a> TagBuilder<'a> {
    pub fn new() -> Self {
        Self {
            tag_name: "",
            content: Content::None,
            attributes: HashSet::new(),
        }
    }

    pub fn set_content_str(&mut self, content: &'a str) -> &mut Self {
        self.set_content(Content::Some(content.to_owned()))
    }

    pub fn set_content_tag(&mut self, content: Tag) -> &mut Self {
        self.set_content(Content::Tag(Box::new(content)))
    }

    pub fn set_content(&mut self, content: Content) -> &mut Self {
        self.content = content;

        self
    }

    pub fn set_tag_name(&mut self, tag_name: &'a str) -> &mut Self {
        self.tag_name = validate_xml_identifier(tag_name).unwrap();

        self
    }

    pub fn set_class_name(&mut self, class_name: &'a str) -> &mut Self {
        let attribute = Attribute::new("class", class_name).unwrap();
        self.set_attribute(attribute);

        self
    }

    pub fn set_id(&mut self, id: &'a str) -> &mut Self {
        let attribute = Attribute::new("id", id).unwrap();
        self.set_attribute(attribute);

        self
    }

    pub fn set_attribute(&mut self, attribute: Attribute /*<'a>*/) -> &mut Self {
        self.attributes.replace(attribute);

        self
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) -> &mut Self {
        self.tag_name = "";
        self.content = Content::None;
        self.attributes.clear();

        self
    }

    pub fn build(&self) -> Tag {
        if self.tag_name.is_empty() && (!self.attributes.is_empty()) {
            println!("{:?}", self);
            panic!("Can not build a Fragment tag with attributes and content")
        }
        if self.attributes.is_empty() {
            Tag::new(self.tag_name.to_owned().clone(), self.content.clone(), None)
        } else {
            Tag::new(
                self.tag_name.to_owned().clone(),
                self.content.clone(),
                Some(self.attributes.clone()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_a() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("a");
        gtb.set_attribute(Attribute::new("href", "https://cundd.net").unwrap());
        gtb.set_content_str("Homepage link");

        assert_eq!(
            "<a href='https://cundd.net'>Homepage link</a>",
            &gtb.build().to_string()
        );
    }

    #[test]
    fn test_escape_class_name() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("div");
        gtb.set_content_str("Some content");
        gtb.set_class_name("some-nice-class' try to hack you");

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class&#39; try to hack you'>Some content</div>"
        );
    }

    #[test]
    fn test_escape_attribute() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("a");
        gtb.set_attribute(Attribute::new("href", "https://cundd.net' try to hack you").unwrap());
        gtb.set_content_str("Homepage link");

        assert_eq!(
            &gtb.build().to_string(),
            "<a href='https://cundd.net&#39; try to hack you'>Homepage link</a>"
        );
    }

    #[test]
    fn test_class_name_attribute_twice() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("div");
        gtb.set_content_str("Some content");
        gtb.set_class_name("some-nice-class");
        gtb.set_attribute(Attribute::new("class", "another-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='another-nice-class'>Some content</div>"
        );
    }

    #[test]
    fn test_attribute_twice() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("div");
        gtb.set_content_str("Some content");
        gtb.set_attribute(Attribute::new("id", "some-nice-id").unwrap());
        gtb.set_attribute(Attribute::new("id", "another-nice-id").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div id='another-nice-id'>Some content</div>"
        );
    }

    #[test]
    fn test_different_attributes() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("div");
        gtb.set_content_str("Some content");
        gtb.set_attribute(Attribute::new("id", "a-nice-id").unwrap());
        gtb.set_attribute(Attribute::new("class", "some-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class' id='a-nice-id'>Some content</div>"
        );

        gtb.reset();
        gtb.set_tag_name("div");
        gtb.set_content_str("Some content");
        gtb.set_attribute(Attribute::new("id", "a-nice-id-that-is-longer").unwrap());
        gtb.set_attribute(Attribute::new("class", "some-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class' id='a-nice-id-that-is-longer'>Some content</div>"
        );
    }

    #[test]
    fn test_reset() {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("span");
        gtb.set_content_str("My content");
        gtb.set_attribute(Attribute::new("class", "some-nice-class").unwrap());
        assert_eq!(
            &gtb.build().to_string(),
            "<span class='some-nice-class'>My content</span>"
        );

        gtb.reset();
        assert_eq!(&gtb.build().to_string(), "</>");
    }
}
