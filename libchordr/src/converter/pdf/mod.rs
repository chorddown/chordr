use std::fs::File;
use std::io::BufWriter;

use printpdf::*;

use crate::converter::ConverterTrait;
use crate::error::Result;
use crate::metadata::metadata_trait::SongMetaTrait;
use crate::models::chord::fmt::Formatting;
use crate::pdf::coordinates::Coordinates;
use crate::pdf::pdf_builder::PdfBuilder;
use crate::pdf::styles::{Style, Styles};
use crate::prelude::*;

type BuildResult = Result<()>;

pub struct PdfConverter {}

impl PdfConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PdfConverter {
    fn default() -> Self {
        Self {}
    }
}

impl ConverterTrait for PdfConverter {
    fn convert(
        &self,
        node: &Node,
        metadata: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Result<String> {
        //        let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
        ////        let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");
        //        let current_layer = doc.get_page(page1).get_layer(layer1);
        //
        //        let font = doc.add_builtin_font(BuiltinFont::TimesBoldItalic).unwrap();
        ////        let font = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
        ////        let font2 = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
        //
        //
        //// text, font size, x from left edge, y from top edge, font
        //        let stylesheets = Styles {
        //            page
        //            chorus: Style::new(font.clone(), Mm(10.0), 10),
        //            verse: Style::new(font.clone(), Mm(10.0), 10),
        //            headline: Style::new(font.clone(), Mm(10.0), 18),
        //        };
        //
        //        let mut builder = PdfBuilder::new(Coordinates::new(Mm(0.0), Mm(0.0)), stylesheets, doc, current_layer);
        //
        //        let doc = builder.build_for_node(node).unwrap();
        //
        //
        //        doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
        //
        //        println!("{:?}", ::std::env::current_dir());
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Metadata;
    use crate::test_helpers::get_test_tokens;

    use super::*;

    #[test]
    fn test_convert() {
        let node = Parser::new().parse(get_test_tokens());
        let result = PdfConverter::new().convert(
            node.unwrap().node(),
            &Metadata::default(),
            Formatting::new_with_format(Format::PDF),
        );

        assert!(result.is_ok());
    }
}
