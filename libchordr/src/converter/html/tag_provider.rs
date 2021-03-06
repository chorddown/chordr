use crate::html::attribute::Attribute;
use crate::html::tag::{Content, Tag};
use crate::html::tag_builder::TagBuilder;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::{Chords, NoteDisplay};
use crate::parser::{Node, SectionType};
use crate::tokenizer::Token;

pub struct TagProvider {}

impl TagProvider {
    pub fn new() -> Self {
        TagProvider {}
    }

    pub fn build_tag_for_node<'a>(&'a self, node: &'a Node, formatting: Formatting) -> Tag {
        let mut gtb = TagBuilder::new();

        match node {
            Node::ChordTextPair {
                chords,
                text,
                last_in_line,
            } => self.build_column(
                self.build_tag_for_chords(chords, formatting),
                self.build_tag_for_chord_text_token(text, formatting, *last_in_line),
            ),
            Node::ChordStandalone(chord) => {
                self.build_column(self.build_tag_for_chords(chord, formatting), Tag::blank())
            }
            Node::Text(text) => {
                self.build_column(Tag::blank(), self.build_tag_for_token(text, formatting))
            }
            Node::Document(children) => gtb
                .set_tag_name("div")
                .set_id("chordr-song")
                .set_content_tag(self.build_tag_for_children(children, formatting))
                .build(),
            Node::Headline(token) => self.build_tag_for_token(token, formatting),
            Node::Quote(token) => self.build_tag_for_token(token, formatting),
            Node::Meta(m) => {
                let inner = format!(
                    "{} {}",
                    gtb.set_tag_name("span")
                        .set_class_name("meta-keyword")
                        .set_content_str(&format!("{}:", m.keyword()))
                        .build(),
                    gtb.set_tag_name("span")
                        .set_class_name("meta-value")
                        .set_content_str(m.content())
                        .build()
                );

                gtb.set_content(Content::Raw(inner)).build()
            }
            Node::Newline => {
                let inner = format!("{}\n", Tag::with_name("hr"));

                Tag::raw(Content::Raw(inner))
            }
            Node::Section {
                head,
                children,
                section_type,
            } => {
                gtb.set_tag_name("section");
                if let Some(class_name) = class_name_for_type(section_type) {
                    gtb.set_class_name(class_name);
                }

                if let Some(head) = head {
                    self.build_tag_for_children(children, formatting)
                        .to_string();
                    let inner = format!(
                        "{}{}",
                        self.build_tag_for_node(head, formatting),
                        self.build_tag_for_children(children, formatting)
                    );

                    gtb.set_content(Content::Raw(inner)).build()
                } else {
                    gtb.set_content_tag(self.build_tag_for_children(children, formatting))
                        .build()
                }
            }
        }
    }

    fn build_tag_for_token<'a>(&'a self, token: &'a Token, _formatting: Formatting) -> Tag {
        let mut gtb = TagBuilder::new();

        match token {
            Token::Literal(c) => gtb.set_tag_name("span").set_content_str(c).build(),
            Token::Quote(c) => gtb.set_tag_name("blockquote").set_content_str(c).build(),
            Token::Headline {
                level,
                text: c,
                modifier: _,
            } => gtb
                .set_tag_name(&format!("h{}", level))
                .set_content_str(c)
                .build(),
            Token::Chord(_) => unreachable!(),
            Token::Meta(_) => unreachable!(),
            Token::Newline => unreachable!(),
        }
    }
    fn build_tag_for_chord_text_token<'a>(
        &'a self,
        token: &'a Token,
        _formatting: Formatting,
        last_in_line: bool,
    ) -> Tag {
        let mut gtb = TagBuilder::new();

        if let Token::Literal(c) = token {
            let span = gtb.set_tag_name("span").set_content_str(c);
            if last_in_line {
                span.set_class_name("-last-in-line");
            }

            span.build()
        } else {
            unreachable!()
        }
        // match token {
        //     Token::Literal(c) => gtb.set_tag_name("span").set_content_str(c).set_class_name(if eol { "-last-of-line" } else { "" }).build(),
        //     _ => unreachable!()
        // }
    }

    fn build_tag_for_chords(&self, chords: &Chords, formatting: Formatting) -> Tag {
        let mut gtb = TagBuilder::new();
        gtb.set_tag_name("span")
            .set_content_str(chords.note_format(formatting))
            .set_class_name("chordr-chord")
            .set_attribute(Attribute::new("data-chord", &chords.note_format(formatting)).unwrap())
            .build()
    }

    fn build_tag_for_children<'a>(&'a self, children: &'a [Node], formatting: Formatting) -> Tag {
        let mut gtb = TagBuilder::new();
        if children.is_empty() {
            return Tag::blank();
        }

        let inner = children
            .iter()
            .map(|n| self.build_tag_for_node(n, formatting).to_string())
            .collect::<Vec<String>>()
            .join("");

        gtb.set_content(Content::Raw(inner)).build()
    }

    fn build_column(&self, chord: Tag, lyric: Tag) -> Tag {
        let chord_text = if chord.is_blank() {
            "&nbsp;".to_owned()
        } else {
            chord.to_string()
        };
        let lyric_text = if lyric.is_blank() {
            "".to_owned()
        } else {
            lyric.to_string()
        };

        let lyric_text_class = match lyric.content() {
            Content::Some(s) if s.ends_with(char::is_alphanumeric) => "text-row -word-split",
            Content::Some(_) => "text-row -word-boundary",
            _ => "text-row",
        };

        let string = format!(
            "<div class='chord-row'>{}</div><div class='{}'>{}</div>",
            chord_text, lyric_text_class, lyric_text
        );

        TagBuilder::new()
            .set_tag_name("div")
            .set_class_name("col")
            .set_content(Content::Raw(string))
            .build()
    }
}

fn class_name_for_type(section_type: &SectionType) -> Option<&'static str> {
    match section_type {
        SectionType::Chorus => Some("chorus"),
        SectionType::Unknown => None,
        SectionType::Bridge => Some("bridge"),
    }
}
