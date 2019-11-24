mod tag_provider;

use crate::prelude::*;
use crate::converter::ConverterTrait;
use tag_provider::TagProvider;

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_node(&self, node: &Node, tag_builder: &TagProvider) -> Result<String> {
        Ok(tag_builder.build_tag_for_node(node).to_string())
    }
}


impl ConverterTrait for HtmlConverter {
    fn convert(&self, node: &Node, _format: Format) -> Result<String> {
        let tag_builder = TagProvider::new();

        self.html_for_node(node, &tag_builder)
    }
}
