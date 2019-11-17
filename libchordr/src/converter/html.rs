mod escape;
mod tag_builder;

use crate::prelude::*;
use crate::converter::ConverterTrait;
use tag_builder::TagBuilder;

pub struct HtmlConverter {}

impl HtmlConverter {
//    fn html_for_token(&self, tag_builder: &TagBuilder, token: &Token) -> String {
//        tag_builder.build_tag_for_token(token)
//    }

//    fn html_for_token_line(&self, tag_builder: &TagBuilder, token_line: &TokenLine) -> String {
//        let mut output = "<div class=\"chordr-line\">".to_owned();
//        for token in token_line {
//            output.push_str(&self.html_for_token(tag_builder, token));
//        }
//
//        output.push_str("</div>\n");
//
//        output
//    }

    fn html_for_node(&self, node: &Node, tag_builder: &TagBuilder) -> Result<String> {
        Ok(tag_builder.build_tag_for_node(node))
    }
}


impl ConverterTrait for HtmlConverter {
    fn convert(&self, node: &Node, _format: Format) -> Result<String> {
        let tag_builder = TagBuilder::new();

        self.html_for_node(node, &tag_builder)
    }
}
