use crate::html::attribute::{Attribute, AttributeCollection};
use crate::html::content::Content;
use crate::html::tag::Tag;

#[derive(Debug)]
pub struct TagBuilder<'a> {
    tag_name: &'a str,
    content: Content,
    attributes: AttributeCollection,
}

impl<'a> TagBuilder<'a> {
    pub fn new() -> Self {
        Self {
            tag_name: "",
            content: Content::None,
            attributes: Default::default(),
        }
    }

    pub fn set_content_str<S: Into<String>>(self, content: S) -> Self {
        self.set_content(Content::Some(content.into()))
    }

    pub fn set_content_tag(self, content: Tag) -> Self {
        self.set_content(Content::Tag(Box::new(content)))
    }

    pub fn set_content(mut self, content: Content) -> Self {
        self.content = content;

        self
    }

    pub fn set_tag_name(mut self, tag_name: &'a str) -> Self {
        self.tag_name = tag_name;

        self
    }

    pub fn set_class_name(self, class_name: &'a str) -> Self {
        self.set_attribute(Attribute::class_name(class_name))
    }

    pub fn set_id(self, id: &'a str) -> Self {
        self.set_attribute(Attribute::id(id))
    }

    pub fn set_attribute(mut self, attribute: Attribute /*<'a>*/) -> Self {
        self.attributes.replace(attribute);

        self
    }

    pub fn build(self) -> Tag {
        if self.tag_name.is_empty() && (!self.attributes.is_empty()) {
            println!("{:?}", self);
            panic!("Can not build a Fragment tag with attributes and content")
        }

        if self.attributes.is_empty() {
            Tag::new(self.tag_name, self.content, None)
        } else {
            Tag::new(self.tag_name, self.content, Some(self.attributes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_a() {
        let gtb = TagBuilder::new()
            .set_tag_name("a")
            .set_attribute(Attribute::new("href", "https://cundd.net").unwrap())
            .set_content_str("Homepage link");

        assert_eq!(
            "<a href='https://cundd.net'>Homepage link</a>",
            &gtb.build().to_string()
        );
    }

    #[test]
    fn test_escape_class_name() {
        let gtb = TagBuilder::new()
            .set_tag_name("div")
            .set_content_str("Some content")
            .set_class_name("some-nice-class' try to hack you");

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class&#39; try to hack you'>Some content</div>"
        );
    }

    #[test]
    fn test_escape_attribute() {
        let gtb = TagBuilder::new()
            .set_tag_name("a")
            .set_attribute(Attribute::new("href", "https://cundd.net' try to hack you").unwrap())
            .set_content_str("Homepage link");

        assert_eq!(
            &gtb.build().to_string(),
            "<a href='https://cundd.net&#39; try to hack you'>Homepage link</a>"
        );
    }

    #[test]
    fn test_class_name_attribute_twice() {
        let gtb = TagBuilder::new()
            .set_tag_name("div")
            .set_content_str("Some content")
            .set_class_name("some-nice-class")
            .set_attribute(Attribute::new("class", "another-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='another-nice-class'>Some content</div>"
        );
    }

    #[test]
    fn test_attribute_twice() {
        let gtb = TagBuilder::new()
            .set_tag_name("div")
            .set_content_str("Some content")
            .set_attribute(Attribute::new("id", "some-nice-id").unwrap())
            .set_attribute(Attribute::new("id", "another-nice-id").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div id='another-nice-id'>Some content</div>"
        );
    }

    #[test]
    fn test_different_attributes() {
        let gtb = TagBuilder::new()
            .set_tag_name("div")
            .set_content_str("Some content")
            .set_attribute(Attribute::new("id", "a-nice-id").unwrap())
            .set_attribute(Attribute::new("class", "some-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class' id='a-nice-id'>Some content</div>"
        );

        let gtb = TagBuilder::new()
            .set_tag_name("div")
            .set_content_str("Some content")
            .set_attribute(Attribute::new("id", "a-nice-id-that-is-longer").unwrap())
            .set_attribute(Attribute::new("class", "some-nice-class").unwrap());

        assert_eq!(
            &gtb.build().to_string(),
            "<div class='some-nice-class' id='a-nice-id-that-is-longer'>Some content</div>"
        );
    }

    #[test]
    fn test_reset() {
        let gtb = TagBuilder::new()
            .set_tag_name("span")
            .set_content_str("My content")
            .set_attribute(Attribute::new("class", "some-nice-class").unwrap());
        assert_eq!(
            &gtb.build().to_string(),
            "<span class='some-nice-class'>My content</span>"
        );

        let gtb = TagBuilder::new();
        assert_eq!(&gtb.build().to_string(), "</>");
    }
}
