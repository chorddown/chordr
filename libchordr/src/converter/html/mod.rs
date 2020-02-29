mod tag_provider;

use crate::converter::ConverterTrait;
use crate::models::chord::fmt::Formatting;
use crate::models::song_meta_trait::SongMetaTrait;
use crate::prelude::*;
use tag_provider::TagProvider;

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_node(
        &self,
        node: &Node,
        meta: &dyn SongMetaTrait,
        tag_builder: &TagProvider,
        formatting: Formatting,
    ) -> Result<String> {
        let tag = tag_builder.build_tag_for_node(node, formatting);

        Ok(format!(
            r#"<div id="chordr">
{}
{}
</div>"#,
            tag,
            self.format_meta(meta)
        ))
    }

    fn format_meta(&self, meta: &dyn SongMetaTrait) -> String {
        let none_text = "None".to_owned();

        format!(
            r"<!--
Meta

Title:      {}
Subtitle:   {}
Artist:     {}
Composer:   {}
Lyricist:   {}
Copyright:  {}
Album:      {}
Year:       {}
Key:        {}
Time:       {}
Tempo:      {}
Duration:   {}
Capo:       {}
-->",
            meta.title().unwrap_or(none_text.clone()),
            meta.subtitle().unwrap_or(none_text.clone()),
            meta.artist().unwrap_or(none_text.clone()),
            meta.composer().unwrap_or(none_text.clone()),
            meta.lyricist().unwrap_or(none_text.clone()),
            meta.copyright().unwrap_or(none_text.clone()),
            meta.album().unwrap_or(none_text.clone()),
            meta.year().unwrap_or(none_text.clone()),
            meta.key().unwrap_or(none_text.clone()),
            meta.time().unwrap_or(none_text.clone()),
            meta.tempo().unwrap_or(none_text.clone()),
            meta.duration().unwrap_or(none_text.clone()),
            meta.capo().unwrap_or(none_text.clone()),
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
    use super::*;
    use crate::test_helpers::get_test_tokens;

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
