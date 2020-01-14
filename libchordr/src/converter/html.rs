mod tag_provider;

use crate::converter::ConverterTrait;
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
    ) -> Result<String> {
        Ok(format!(
            r#"<div id="chordr">
{}
{}
</div>"#,
            tag_builder.build_tag_for_node(node),
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
    fn convert(&self, node: &Node, meta: &dyn SongMetaTrait, _format: Format) -> Result<String> {
        let tag_builder = TagProvider::new();

        self.html_for_node(node, meta, &tag_builder)
    }
}
