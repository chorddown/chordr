use crate::html::content::Content;
use crate::html::tag::Tag;
use crate::html::tag_builder::TagBuilder;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::{Chords, NoteDisplay};
use crate::models::metadata::MetadataTrait;
use crate::parser::{Node, SectionType};
use crate::tokenizer::{RawMetadata, Token};

pub struct TagProvider {}

impl TagProvider {
    pub fn new() -> Self {
        TagProvider {}
    }

    pub fn build_tag_for_node<'a>(
        &'a self,
        node: &'a Node,
        meta: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Tag {
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
            Node::Document(children) => TagBuilder::new()
                .set_tag_name("div")
                .set_id("chordr-song")
                .set_content_tag(self.build_tag_for_children(children, meta, formatting))
                .build(),
            Node::Headline(token) => self.build_tag_for_token(token, formatting),
            Node::Quote(token) => self.build_tag_for_token(token, formatting),
            Node::Meta(m) => self.build_tag_for_meta(m, meta, formatting),
            Node::Newline => Tag::raw(format!("{}\n", Tag::hr())),
            Node::Section {
                head,
                children,
                section_type,
            } => self.build_tag_for_section(head, children, section_type, meta, formatting),
        }
    }

    fn build_tag_for_meta(
        &self,
        metadata_token: &RawMetadata,
        song_metadata: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Tag {
        let content = get_song_metadata_content(metadata_token, song_metadata, formatting);

        Tag::raw(format!(
            "{} {}",
            Tag::span(
                Content::from_string(format!("{}:", metadata_token.label())),
                Some("metadata-keyword"),
            ),
            Tag::span(
                Content::from_string(
                    content.unwrap_or_else(|| metadata_token.content().to_string())
                ),
                Some("metadata-value")
            ),
        ))
    }

    fn build_tag_for_section(
        &self,
        head: &Option<Box<Node>>,
        children: &[Node],
        section_type: &SectionType,
        meta: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Tag {
        let mut gtb = TagBuilder::new().set_tag_name("section");
        if let Some(class_name) = class_name_for_type(section_type) {
            gtb = gtb.set_class_name(class_name);
        }

        if let Some(head) = head {
            let inner = format!(
                "{}{}",
                self.build_tag_for_node(head, meta, formatting),
                self.build_tag_for_children(children, meta, formatting)
            );

            gtb.set_content(Content::Raw(inner)).build()
        } else {
            gtb.set_content_tag(self.build_tag_for_children(children, meta, formatting))
                .build()
        }
    }

    fn build_tag_for_token<'a>(&'a self, token: &'a Token, _formatting: Formatting) -> Tag {
        match token {
            Token::Literal(c) => Tag::span(Content::from_string(c), None),
            Token::Quote(c) => Tag::blockquote(Content::from_string(c), None),
            Token::Headline {
                level,
                text: c,
                modifier: _,
            } => Tag::headline(*level, Content::from_string(c), None),
            Token::Chord(_) => unreachable!(),
            Token::Metadata(_) => unreachable!(),
            Token::Newline => unreachable!(),
        }
    }

    fn build_tag_for_chord_text_token(
        &self,
        token: &Token,
        _formatting: Formatting,
        last_in_line: bool,
    ) -> Tag {
        if let Token::Literal(c) = token {
            if last_in_line {
                Tag::span(Content::from_string(c), Some("-last-in-line"))
            } else {
                Tag::span(Content::from_string(c), None)
            }
        } else {
            unreachable!()
        }
    }

    fn build_tag_for_chords(&self, chords: &Chords, formatting: Formatting) -> Tag {
        let formatted_chords = chords.note_format(formatting);

        Tag::span_with_chord(
            Content::from_string(formatted_chords.clone()),
            Some("chordr-chord"),
            formatted_chords,
        )
    }

    fn build_tag_for_children<'a>(
        &'a self,
        children: &'a [Node],
        meta: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Tag {
        if children.is_empty() {
            Tag::blank()
        } else {
            Tag::raw(
                children
                    .iter()
                    .map(|n| self.build_tag_for_node(n, meta, formatting).to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            )
        }
    }

    fn build_column(&self, chord: Tag, lyric: Tag) -> Tag {
        let chord_text = if chord.is_blank() {
            "&nbsp;".to_owned()
        } else {
            chord.to_string()
        };

        let lyric_text_class = match lyric.content() {
            Content::Some(s) if s.ends_with(char::is_alphanumeric) => "text-row -word-split",
            Content::Some(_) => "text-row -word-boundary",
            _ => "text-row",
        };

        let html = format!(
            "<div class='chord-row'>{}</div><div class='{}'>{}</div>",
            chord_text, lyric_text_class, lyric
        );

        Tag::div(Content::Raw(html), Some("col"))
    }
}

fn get_song_metadata_content(
    metadata_token: &RawMetadata,
    song_metadata: &dyn MetadataTrait,
    formatting: Formatting,
) -> Option<String> {
    let result = match metadata_token {
        RawMetadata::Subtitle(_) => song_metadata.subtitle(),
        RawMetadata::Artist(_) => song_metadata.artist(),
        RawMetadata::Composer(_) => song_metadata.composer(),
        RawMetadata::Lyricist(_) => song_metadata.lyricist(),
        RawMetadata::Copyright(_) => song_metadata.copyright(),
        RawMetadata::Album(_) => song_metadata.album(),
        RawMetadata::Year(_) => song_metadata.year(),
        RawMetadata::Key(_) => {
            return song_metadata.key().map(|c| c.note_format(formatting));
        }
        RawMetadata::OriginalKey(_) => {
            return song_metadata
                .original_key()
                .map(|c| c.note_format(formatting));
        }
        RawMetadata::Time(_) => song_metadata.time(),
        RawMetadata::Tempo(_) => song_metadata.tempo(),
        RawMetadata::Duration(_) => song_metadata.duration(),
        RawMetadata::Capo(_) => song_metadata.capo(),
        RawMetadata::OriginalTitle(_) => song_metadata.original_title(),
        RawMetadata::AlternativeTitle(_) => song_metadata.alternative_title(),
        RawMetadata::CCLISongId(_) => song_metadata.ccli_song_id(),
        RawMetadata::BNotation(_) => return Some(song_metadata.b_notation().to_string()),
    };

    result.map(ToOwned::to_owned)
}

fn class_name_for_type(section_type: &SectionType) -> Option<&'static str> {
    match section_type {
        SectionType::Chorus => Some("chorus"),
        SectionType::Unknown => None,
        SectionType::Bridge => Some("bridge"),
    }
}
