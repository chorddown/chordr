use crate::tokenizer::Token;
use crate::parser::Node;
use crate::html::tag_builder::TagBuilder;
use crate::html::tag::{Tag, Content};
use crate::html::attribute::Attribute;

pub struct TagProvider {}

impl TagProvider {
    pub fn new() -> Self {
        TagProvider {}
    }

    pub fn build_tag_for_node<'a>(&'a self, node: &'a Node) -> Tag/*<'a>*/ {
        let mut gtb = TagBuilder::new();

        match node {
            Node::ChordTextPair { chord, text } => self.build_column(
                self.build_tag_for_token(chord),
                self.build_tag_for_token(text),
            ),
            Node::ChordStandalone(chord) => self.build_column(
                self.build_tag_for_token(chord),
                Tag::blank(),
            ),
            Node::Text(text) => self.build_column(
                Tag::blank(),
                self.build_tag_for_token(text),
            ),
            Node::Document(children) => gtb.set_tag_name("div").set_id("chordr").set_content_tag(self.build_tag_for_children(children)).build(),
            Node::Headline(token) => self.build_tag_for_token(token),
            Node::Quote(token) => self.build_tag_for_token(token),
            Node::Newline => {
                let inner = format!("{}\n", Tag::with_name("hr"));

                Tag::raw(Content::Raw(inner))
            }
            Node::Section { head, children } => {
                if let Some(head) = head {
                    let inner = format!(
                        "{}{}",
                        self.build_tag_for_node(head),
                        self.build_tag_for_children(children)
                    );

                    gtb.set_tag_name("section")
                        .set_content(Content::Raw(inner))
                        .build()
                } else {
                    gtb.set_tag_name("section")
                        .set_content_tag(self.build_tag_for_children(children))
                        .build()
                }
            }
        }
    }

    fn build_tag_for_token<'a>(&'a self, token: &'a Token) -> Tag/*<'a>*/ {
        let mut gtb = TagBuilder::new();

        match token {
            Token::Chord(c) => {
                gtb
                    .set_tag_name("span")
                    .set_content_str(c)
                    .set_class_name("chordr-chord")
                    .set_attribute(Attribute::new("data-chord", c).unwrap())
                    .build()
            }
            Token::Literal(c) => {
                gtb
                    .set_tag_name("span")
                    .set_content_str(c)
                    .build()
            }
            Token::Newline => {
                let inner = format!("{}\n", Tag::with_name("br"));

                Tag::raw(Content::Raw(inner))
            }
            Token::Quote(c) => {
                gtb
                    .set_tag_name("blockquote")
                    .set_content_str(c)
                    .build()
            }
            Token::Headline { level, text: c } => {
                gtb
                    .set_tag_name(&format!("h{}", level))
                    .set_content_str(c)
                    .build()
            }
        }
    }

    fn build_tag_for_children<'a, 'b>(&'a self, children: &'a Vec<Node>) -> Tag/*<'a>*/ {
        let mut gtb = TagBuilder::new();
        let inner = children
            .iter()
            .map(|n| self.build_tag_for_node(n).to_string())
            .collect::<Vec<String>>()
            .join("");

        gtb.set_content(Content::Raw(inner))
            .build()
    }

    fn build_column(&self, row1: Tag, row2: Tag) -> Tag {
        let row1text = if row1.is_blank() { "&nbsp;".to_owned() } else { row1.to_string() };
        let row2text = if row2.is_blank() { "&nbsp;".to_owned() } else { row2.to_string() };

        let string = format!("<div class='chord-row'>{}</div><div class='text-row'>{}</div>", row1text, row2text);

        TagBuilder::new()
            .set_tag_name("div")
            .set_class_name("col")
            .set_content(Content::Raw(string))
            .build()
    }
}
