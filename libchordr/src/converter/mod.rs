use self::chorddown::ChorddownConverter;
use self::html::HtmlConverter;
#[cfg(feature = "pdf")]
use self::pdf::PdfConverter;
use crate::converter::songbeamer::SongBeamerConverter;
use crate::error::Result;
use crate::models::chord::fmt::Formatting;
use crate::models::song_meta_trait::SongMetaTrait;
use crate::prelude::*;

mod chorddown;
mod html;
#[cfg(feature = "pdf")]
mod pdf;
mod songbeamer;

pub trait ConverterTrait {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn SongMetaTrait,
        formatting: Formatting,
    ) -> Result<String>;
}

pub struct Converter {}

impl Converter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_converter(format: Format) -> Box<dyn ConverterTrait> {
        match format {
            Format::HTML => Box::new(HtmlConverter {}),
            Format::Chorddown => Box::new(ChorddownConverter {}),
            Format::SongBeamer => Box::new(SongBeamerConverter {}),
            #[cfg(feature = "pdf")]
            Format::PDF => Box::new(PdfConverter {}),
        }
    }
}

impl ConverterTrait for Converter {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn SongMetaTrait,
        formatting: Formatting,
    ) -> Result<String> {
        Converter::get_converter(formatting.format).convert(node, meta, formatting)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::get_test_tokens;

    #[test]
    fn test_convert() {
        let content = include_str!("../../tests/resources/swing_low_sweet_chariot.html");
        let result = Parser::new().parse(token_lines_to_tokens(get_test_tokens()));
        assert!(result.is_ok());
        let parser_result = result.unwrap();
        let result = Converter::new().convert(
            parser_result.node_as_ref(),
            parser_result.meta_as_ref(),
            Formatting::with_format(Format::HTML),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content.trim())
    }
}
