mod escape;
mod tag_builder;

use crate::prelude::*;
use crate::converter::ConverterTrait;
use crate::tokenizer::{Token, TokenLine};
use tag_builder::TagBuilder;

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_token(&self, tag_builder: &TagBuilder, token: &Token) -> String {
        tag_builder.build_tag_for_token(token)
    }

    fn html_for_token_line(&self, tag_builder: &TagBuilder, token_line: &TokenLine) -> String {
        let mut output = "<div class=\"chordr-line\">".to_owned();
        for token in token_line {
            output.push_str(&self.html_for_token(tag_builder, token));
            output.push_str("\n")
        }

        output.push_str("</div>");

        output
    }
}


impl ConverterTrait for HtmlConverter {
    fn convert(&self, token_lines: &Vec<TokenLine>, _format: Format) -> Result<String> {
        let mut buffer = String::new();
        let tag_builder = TagBuilder::new();
        for token_line in token_lines {
            buffer.push_str(&HtmlConverter::html_for_token_line(self, &tag_builder, token_line));
        }

        Ok(buffer)
    }
}
