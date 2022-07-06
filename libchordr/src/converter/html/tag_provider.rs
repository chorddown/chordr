use crate::html::content::Content;
use crate::html::tag::Tag;
use crate::html::tag_builder::TagBuilder;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::{Chords, NoteDisplay};
use crate::models::meta::MetaTrait;
use crate::parser::{Node, SectionType};
use crate::tokenizer::{Meta, Token};

pub struct TagProvider {}

impl TagProvider {
    pub fn new() -> Self {
        TagProvider {}
    }

    pub fn build_tag_for_node<'a>(
        &'a self,
        node: &'a Node,
        meta: &dyn MetaTrait,
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
        metadata_token: &Meta,
        song_metadata: &dyn MetaTrait,
        formatting: Formatting,
    ) -> Tag {
        let content = get_song_metadata_content(metadata_token, song_metadata, formatting);

        Tag::raw(format!(
            "{} {}",
            Tag::span(
                Content::from_string(format!("{}:", metadata_token.keyword())),
                Some(class_name_for_metadata_keyword(metadata_token)),
            ),
            Tag::span(
                Content::from_string(
                    content.unwrap_or_else(|| metadata_token.content().to_string())
                ),
                Some(class_name_for_metadata_value(metadata_token))
            ),
        ))
    }

    fn build_tag_for_section(
        &self,
        head: &Node,
        children: &[Node],
        section_type: &SectionType,
        meta: &dyn MetaTrait,
        formatting: Formatting,
    ) -> Tag {
        let mut gtb = TagBuilder::new().set_tag_name("section");
        if let Some(class_name) = class_name_for_type(section_type) {
            gtb = gtb.set_class_name(class_name);
        }

        let inner = format!(
            "{}{}",
            self.build_tag_for_node(head, meta, formatting),
            self.build_tag_for_children(children, meta, formatting)
        );

        gtb.set_content(Content::Raw(inner)).build()
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
            Token::Meta(_) => unreachable!(),
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
        meta: &dyn MetaTrait,
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
    metadata_token: &Meta,
    song_metadata: &dyn MetaTrait,
    formatting: Formatting,
) -> Option<String> {
    match metadata_token {
        Meta::Subtitle(_) => song_metadata.subtitle(),
        Meta::Artist(_) => song_metadata.artist(),
        Meta::Composer(_) => song_metadata.composer(),
        Meta::Lyricist(_) => song_metadata.lyricist(),
        Meta::Copyright(_) => song_metadata.copyright(),
        Meta::Album(_) => song_metadata.album(),
        Meta::Year(_) => song_metadata.year(),
        Meta::Key(_) => song_metadata.key().map(|c| c.note_format(formatting)),
        Meta::OriginalKey(_) => song_metadata
            .original_key()
            .map(|c| c.note_format(formatting)),
        Meta::Time(_) => song_metadata.time(),
        Meta::Tempo(_) => song_metadata.tempo(),
        Meta::Duration(_) => song_metadata.duration(),
        Meta::Capo(_) => song_metadata.capo(),
        Meta::OriginalTitle(_) => song_metadata.original_title(),
        Meta::AlternativeTitle(_) => song_metadata.alternative_title(),
        Meta::CCLISongId(_) => song_metadata.ccli_song_id(),
        Meta::BNotation(_) => Some(song_metadata.b_notation().to_string()),
    }
}

fn class_name_for_type(section_type: &SectionType) -> Option<&'static str> {
    match section_type {
        SectionType::Verse => None,
        SectionType::Chorus => Some("chorus"),
        SectionType::Bridge => Some("bridge"),
        SectionType::Reference => Some("reference"),
    }
}

const fn class_name_for_metadata_keyword(metadata_token: &Meta) -> &'static str {
    match metadata_token {
        Meta::Artist(_) => "meta-keyword -artist",
        Meta::Composer(_) => "meta-keyword -composer",
        Meta::Lyricist(_) => "meta-keyword -lyricist",
        Meta::Copyright(_) => "meta-keyword -copyright",
        Meta::Album(_) => "meta-keyword -album",
        Meta::Year(_) => "meta-keyword -year",
        Meta::Key(_) => "meta-keyword -key",
        Meta::OriginalKey(_) => "meta-keyword -original-key",
        Meta::Time(_) => "meta-keyword -time",
        Meta::Tempo(_) => "meta-keyword -tempo",
        Meta::Duration(_) => "meta-keyword -duration",
        Meta::Subtitle(_) => "meta-keyword -subtitle",
        Meta::Capo(_) => "meta-keyword -capo",
        Meta::OriginalTitle(_) => "meta-keyword -original-title",
        Meta::AlternativeTitle(_) => "meta-keyword -alternative-title",
        Meta::CCLISongId(_) => "meta-keyword -ccli-song-id",
        Meta::BNotation(_) => "meta-keyword -b-notation",
    }
}

const fn class_name_for_metadata_value(metadata_token: &Meta) -> &'static str {
    match metadata_token {
        Meta::Artist(_) => "meta-value -artist",
        Meta::Composer(_) => "meta-value -composer",
        Meta::Lyricist(_) => "meta-value -lyricist",
        Meta::Copyright(_) => "meta-value -copyright",
        Meta::Album(_) => "meta-value -album",
        Meta::Year(_) => "meta-value -year",
        Meta::Key(_) => "meta-value -key",
        Meta::OriginalKey(_) => "meta-value -original-key",
        Meta::Time(_) => "meta-value -time",
        Meta::Tempo(_) => "meta-value -tempo",
        Meta::Duration(_) => "meta-value -duration",
        Meta::Subtitle(_) => "meta-value -subtitle",
        Meta::Capo(_) => "meta-value -capo",
        Meta::OriginalTitle(_) => "meta-value -original-title",
        Meta::AlternativeTitle(_) => "meta-value -alternative-title",
        Meta::CCLISongId(_) => "meta-value -ccli-song-id",
        Meta::BNotation(_) => "meta-value -b-notation",
    }
}
