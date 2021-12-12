use tag_provider::TagProvider;

use crate::converter::ConverterTrait;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::NoteDisplay;
use crate::prelude::*;

mod tag_provider;

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_node(
        &self,
        node: &Node,
        metadata: &dyn MetadataTrait,
        tag_builder: &TagProvider,
        formatting: Formatting,
    ) -> Result<String> {
        let tag = tag_builder.build_tag_for_node(node, metadata, formatting);

        Ok(format!(
            r#"<div id="chordr">
{}
{}
</div>"#,
            tag,
            self.format_meta(metadata, formatting)
        ))
    }

    fn format_meta(&self, metadata: &dyn MetadataTrait, formatting: Formatting) -> String {
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
            metadata.title().unwrap_or_else(|| none_text),
            metadata.original_title().unwrap_or_else(|| none_text),
            metadata.alternative_title().unwrap_or_else(|| none_text),
            metadata.subtitle().unwrap_or_else(|| none_text),
            metadata.artist().unwrap_or_else(|| none_text),
            metadata.composer().unwrap_or_else(|| none_text),
            metadata.lyricist().unwrap_or_else(|| none_text),
            metadata.copyright().unwrap_or_else(|| none_text),
            metadata.album().unwrap_or_else(|| none_text),
            metadata.year().unwrap_or_else(|| none_text),
            metadata
                .key()
                .map_or_else(|| none_text.to_owned(), |c| c.note_format(formatting)),
            metadata
                .original_key()
                .map_or_else(|| none_text.to_owned(), |c| c.note_format(formatting)),
            metadata.time().unwrap_or_else(|| none_text),
            metadata.tempo().unwrap_or_else(|| none_text),
            metadata.duration().unwrap_or_else(|| none_text),
            metadata.capo().unwrap_or_else(|| none_text),
            metadata.ccli_song_id().unwrap_or_else(|| none_text),
        )
    }
}

impl ConverterTrait for HtmlConverter {
    fn convert(
        &self,
        node: &Node,
        metadata: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Result<String> {
        let tag_builder = TagProvider::new();

        self.html_for_node(node, metadata, &tag_builder, formatting)
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
            parser_result.node(),
            parser_result.metadata(),
            Formatting::new_with_format(Format::HTML),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.trim())
    }
}
