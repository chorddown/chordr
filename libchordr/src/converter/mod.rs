use crate::prelude::*;
use crate::error::Result;
use crate::converter::html::HtmlConverter;
use crate::tokenizer::TokenLine;

mod html;

pub trait ConverterTrait {
    fn convert(&self, token_lines: &Vec<TokenLine>, format: Format) -> Result<String>;
}

pub struct Converter {}

impl Converter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_converter(format: Format) -> Box<dyn ConverterTrait> {
        match format {
            Format::HTML => Box::new(HtmlConverter {})
        }
    }
}

impl ConverterTrait for Converter {
    fn convert(&self, token_lines: &Vec<TokenLine>, format: Format) -> Result<String> {
        Converter::get_converter(format).convert(token_lines, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_tokens;

    #[test]
    fn test_tokenize_long() {
        let content = include_str!("../../tests/resources/swing_low_sweet_chariot.html");
        let converter = Converter {};
        let result = converter.convert(&get_test_tokens(), Format::HTML);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.trim())
    }
}
