use tag_provider::TagProvider;

use crate::converter::ConverterTrait;
use crate::metadata::MetadataIterItemValue;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::NoteDisplay;
use crate::models::metadata::Metadata;
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
        let metadata = Metadata::from(metadata);
        let mut buffer = String::new();
        for item in metadata {
            let label = item.keyword.label().to_string() + ":";
            let value = match item.value {
                MetadataIterItemValue::Chord(c) => c.note_format(formatting),
                MetadataIterItemValue::String(s) => s,
                MetadataIterItemValue::BNotation(b) => b.to_string(),
                MetadataIterItemValue::None => "None".to_string(),
            };
            buffer.push_str(&format!("{:<19} {}\n", label, value))
        }

        format!(
            r"<!--
Meta

{}-->",
            buffer
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
