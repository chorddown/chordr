use crate::prelude::*;
use crate::error::Result;
use crate::converter::html::HtmlConverter;

mod html;

pub trait ConverterTrait {
    fn convert(&self, node: &Node, format: Format) -> Result<String>;
}

pub struct Converter {}

impl Converter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_converter(format: Format) -> impl ConverterTrait {
        match format {
            Format::HTML => HtmlConverter {}
        }
    }
}

impl ConverterTrait for Converter {
    fn convert(&self, node: &Node, format: Format) -> Result<String> {
        Converter::get_converter(format).convert(node, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_tokens;

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../tests/resources/swing_low_sweet_chariot.html");
        let node = Parser::new().parse(token_lines_to_tokens(get_test_tokens()));
        let result = Converter::new().convert(&node, Format::HTML);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content)
    }
}
