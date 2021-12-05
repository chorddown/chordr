use tag_provider::TagProvider;

use crate::converter::ConverterTrait;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::NoteDisplay;
use crate::models::song_meta_trait::SongMetaTrait;
use crate::prelude::*;

mod tag_provider;

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_node(
        &self,
        node: &Node,
        meta: &dyn SongMetaTrait,
        tag_builder: &TagProvider,
        formatting: Formatting,
    ) -> Result<String> {
        let tag = tag_builder.build_tag_for_node(node, meta, formatting);

        Ok(format!(
            r#"<div id="chordr">
{}
{}
</div>"#,
            tag,
            self.format_meta(meta, formatting)
        ))
    }

    fn format_meta(&self, meta: &dyn SongMetaTrait, formatting: Formatting) -> String {
        let none_text = "None";

        format!(
            r"<!--
Meta

Title:              {}
Original Title:     {}
Alternative Title:  {}
Subtitle:           {}
Artist:             {}
Composer:           {}
Lyricist:           {}
Copyright:          {}
Album:              {}
Year:               {}
Key:                {}
Original Key:       {}
Time:               {}
Tempo:              {}
Duration:           {}
Capo:               {}
CCLI Song #:        {}
-->",
            meta.title().unwrap_or_else(|| none_text.to_owned()),
            meta.original_title()
                .unwrap_or_else(|| none_text.to_owned()),
            meta.alternative_title()
                .unwrap_or_else(|| none_text.to_owned()),
            meta.subtitle().unwrap_or_else(|| none_text.to_owned()),
            meta.artist().unwrap_or_else(|| none_text.to_owned()),
            meta.composer().unwrap_or_else(|| none_text.to_owned()),
            meta.lyricist().unwrap_or_else(|| none_text.to_owned()),
            meta.copyright().unwrap_or_else(|| none_text.to_owned()),
            meta.album().unwrap_or_else(|| none_text.to_owned()),
            meta.year().unwrap_or_else(|| none_text.to_owned()),
            meta.key()
                .map_or_else(|| none_text.to_owned(), |c| c.note_format(formatting)),
            meta.original_key()
                .map_or_else(|| none_text.to_owned(), |c| c.note_format(formatting)),
            meta.time().unwrap_or_else(|| none_text.to_owned()),
            meta.tempo().unwrap_or_else(|| none_text.to_owned()),
            meta.duration().unwrap_or_else(|| none_text.to_owned()),
            meta.capo().unwrap_or_else(|| none_text.to_owned()),
            meta.ccli_song_id().unwrap_or_else(|| none_text.to_owned()),
        )
    }
}

impl ConverterTrait for HtmlConverter {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn SongMetaTrait,
        formatting: Formatting,
    ) -> Result<String> {
        let tag_builder = TagProvider::new();

        self.html_for_node(node, meta, &tag_builder, formatting)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::get_test_tokens;

    use super::*;

    #[test]
    fn test_convert() {
        let content = include_str!("../../../tests/resources/swing_low_sweet_chariot.html");
        let result = Parser::new().parse(get_test_tokens());
        assert!(result.is_ok());
        let parser_result = result.unwrap();

        let converter = HtmlConverter {};
        let result = converter.convert(
            parser_result.node_as_ref(),
            parser_result.meta_as_ref(),
            Formatting::with_format(Format::HTML),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.trim())
    }
}
