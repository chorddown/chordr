use crate::converter::ConverterTrait;
use crate::prelude::*;

use printpdf::*;
use std::fs::File;
//use rusttype::{point, Font, FontCollection, PositionedGlyph, Scale};

use crate::error::Result;
use crate::pdf::coordinates::Coordinates;
use crate::pdf::styles::Styles;

type BuildResult = Result<()>;

pub struct PdfBuilder {
    coordinates: Coordinates,
    styles: Styles,
    layer: PdfLayerReference,
    doc: PdfDocumentReference,
}

impl PdfBuilder {
    pub fn new(
        coordinates: Coordinates,
        styles: Styles,
        doc: PdfDocumentReference,
        layer: PdfLayerReference,
    ) -> Self {
        Self {
            coordinates,
            styles,
            layer,
            doc,
        }
    }
    pub fn build_for_node(mut self, node: &Node) -> Result<PdfDocumentReference> /*<'a>*/ {
        match self.draw_node(node) {
            Err(e) => Err(e),
            Ok(_) => Ok(self.doc),
        }
    }

    pub fn draw_node<'a>(&'a mut self, node: &'a Node) -> BuildResult /*<'a>*/ {
        //        let text = format!("{:?}", node);
        //        self.layer.use_text(text, 10, self.coordinates.x, self.coordinates.y, &self.style.font);

        match node {
            Node::ChordTextPair { chord, text } => {
                let column1 = self.draw_token(chord);
                let style = self.styles.chorus().unwrap().clone();
                self.coordinates.y -= style.line_height;
                let column2 = self.draw_token(text);
                self.coordinates.y += style.line_height;

                Ok(())
            }
            Node::ChordStandalone(chord) => {
                self.draw_token(chord)
                //                self.build_column(
                //                    self.build_tag_for_token(chord),
                //                    Tag::blank(),
                //                )
            }
            Node::Text(text) => self.draw_token(text),
            Node::Document(children) => self.build_tag_for_children(children),
            Node::Headline(token) => self.draw_token(token),
            Node::Quote(token) => self.draw_token(token),
            Node::Newline => {
                let style = self.styles.chorus().unwrap();
                self.coordinates.y -= style.line_height;

                Ok(())
                //                let inner = format!("{}\n", Tag::with_name("hr"));
                //
                //                Tag::raw(Content::Raw(inner))
            }
            Node::Section { head, children } => {
                if let Some(head) = head {
                    let inner = format!(
                        "{:?}{:?}",
                        self.draw_node(head),
                        self.build_tag_for_children(children)
                    );
                    Ok(())
                //                    gtb.set_tag_name("section")
                //                        .set_content(Content::Raw(inner))
                //                        .build()
                } else {
                    self.build_tag_for_children(children)

                    //                    gtb.set_tag_name("section")
                    //                        .set_content_tag(self.build_tag_for_children(children))
                    //                        .build()
                }
            }
        }
    }

    fn draw_token<'a>(&'a mut self, token: &'a Token) -> BuildResult /*<'a>*/ {
        //        let mut gtb = TagBuilder::new();

        match token {
            Token::Chord(text) => {
                //                let text = "Lorem ipsum";
                //                let text2 = "unicode: стуфхfцчшщъыьэюя";

                //                let font = self.doc.add_builtin_font(BuiltinFont::TimesBoldItalic).unwrap();
                //                let font = self.doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

                //        let font2 = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

                // text, font size, x from left edge, y from top edge, font
                //                let text = format!("{:?}", node);
                let style = self.styles.chord().unwrap();
                self.layer.use_text(
                    text,
                    style.font_size,
                    self.coordinates.x,
                    self.coordinates.y,
                    &style.font,
                );

                // For more complex layout of text, you can use functions
                // defined on the PdfLayerReference
                // Make sure to wrap your commands
                // in a `begin_text_section()` and `end_text_section()` wrapper
                //                self.layer.begin_text_section();
                //                {
                //                    // setup the general fonts.
                //                    // see the docs for these functions for details
                //                    self.layer.set_font(&self.stylesheets.chord().font, self.stylesheets.chord().font_size);
                //                    self.layer.set_text_cursor(self.coordinates.x, self.coordinates.y);
                //                    self.layer.set_line_height(33);
                //                    self.layer.set_word_spacing(3000);
                //                    self.layer.set_character_spacing(10);
                ////                    self.layer.set_text_rendering_mode(TextRenderingMode::Stroke);
                //
                ////                    // write two lines (one line break)
                ////                    self.layer.write_text(text.clone(), &font2);
                ////                    self.layer.add_line_break();
                ////                    self.layer.write_text(text2.clone(), &font2);
                ////                    self.layer.add_line_break();
                ////
                ////                    // write one line, but write text2 in superscript
                ////                    self.layer.write_text(text.clone(), &font2);
                ////                    self.layer.set_line_offset(10);
                ////                    self.layer.write_text(text2.clone(), &font2);
                //                }
                //                self.layer.end_text_section();

                //                self.coordinates.y += PdfBuilder::text_width(text, self.stylesheets.chord().font_size ).into();
                Ok(())
                //
                //                gtb
                //                    .set_tag_name("span")
                //                    .set_content_str(c)
                //                    .set_class_name("chordr-chord")
                //                    .set_attribute(Attribute::new("data-chord", c).unwrap())
                //                    .build()
            }
            Token::Literal(c) => {
                let style = self.styles.chord().unwrap();
                self.layer.use_text(
                    c,
                    style.font_size,
                    self.coordinates.x,
                    self.coordinates.y,
                    &style.font,
                );
                //                self.layer.use_text(c, self.stylesheets.verse.font_size, self.coordinates.x, self.coordinates.y, &self.stylesheets.verse.font);
                Ok(())

                //                gtb
                //                    .set_tag_name("span")
                //                    .set_content_str(c)
                //                    .build()
            }
            Token::Newline => {
                unreachable!();
                Ok(())
                //                let inner = format!("{}\n", Tag::with_name("br"));
                //
                //                Tag::raw(Content::Raw(inner))
            }
            Token::Quote(c) => {
                //                unreachable!();
                Ok(())
                //                gtb
                //                    .set_tag_name("blockquote")
                //                    .set_content_str(c)
                //                    .build()
            }
            Token::Headline { level, text: c } => {
                //                self.layer.use_text(c, self.stylesheets.headline.font_size, self.coordinates.x, self.coordinates.y, &self.stylesheets.headline.font);
                let style = self.styles.chord().unwrap();
                self.layer.use_text(
                    c,
                    style.font_size,
                    self.coordinates.x,
                    self.coordinates.y,
                    &style.font,
                );
                Ok(())
                //                gtb
                //                    .set_tag_name(&format!("h{}", level))
                //                    .set_content_str(c)
                //                    .build()
            }
        }
    }

    fn build_tag_for_children<'a, 'b>(&'a mut self, children: &'a Vec<Node>) -> BuildResult /*<'a>*/
    {
        let result = children
            .iter()
            .map(|n| self.draw_node(n))
            .find(BuildResult::is_err);
        match result {
            Some(error) => error,
            None => Ok(()),
        }
    }

    fn build_column(&mut self, column1: BuildResult, column2: BuildResult) -> BuildResult {
        Ok(())
    }

    fn text_width(text: &str, font_size: i64) -> Pt {
        //        let font_data = include_bytes!("../../../resources/fonts/Libre-Baskerville/LibreBaskerville-Regular.ttf");
        //        let collection = FontCollection::from_bytes(font_data as &[u8]).unwrap_or_else(|e| {
        //            panic!("error constructing a FontCollection from bytes: {}", e);
        //        });
        //        let font = collection
        //            .into_font() // only succeeds if collection consists of one font
        //            .unwrap_or_else(|e| {
        //                panic!("error turning FontCollection into a Font: {}", e);
        //            });
        //
        //
        //        // The font size to use
        //        let scale = Scale::uniform(font_size as f32);
        //
        //        // The text to render
        //
        //        // Use a dark red colour
        //
        //        let v_metrics = font.v_metrics(scale);
        //
        //        // layout the glyphs in a line with 20 pixels padding
        //        let glyphs: Vec<_> = font
        //            .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        //            .collect();
        //
        //        // work out the layout size
        //        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        //        let glyphs_width = {
        //            let min_x = glyphs
        //                .first()
        //                .map(|g| g.pixel_bounding_box().unwrap().min.x)
        //                .unwrap();
        //            let max_x = glyphs
        //                .last()
        //                .map(|g| g.pixel_bounding_box().unwrap().max.x)
        //                .unwrap();
        //            (max_x - min_x)
        //        };
        //
        //        Pt(glyphs_width as f64)
        Pt(1.0 as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::styles::{PageSize, Style, Styles};

    #[test]
    fn test_text_width() {
        let width = PdfBuilder::text_width("This is RustType rendered into a png!", 10);
        println!("{:?}", width)
    }

    #[test]
    fn test_build_pdf() {
        let text = "text lloo";

        let (doc, page1, layer1) =
            PdfDocument::new("PDF_Document_title", Mm(500.0), Mm(300.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let mut font_reader = std::io::Cursor::new(
            include_bytes!("../../../resources/fonts/RobotoMedium.ttf").as_ref(),
        );

        let font = doc.add_external_font(&mut font_reader).unwrap();

        // `use_text` is a wrapper around making a simple string
        current_layer.use_text(text, 48, Mm(10.0), Mm(200.0), &font);

        //
        //
        //        let content = include_str!("../../tests/resources/swing_low_sweet_chariot.chorddown");
        //        let token_lines = build_tokenizer().tokenize(content);
        //        let node = Parser::new().parse(token_lines_to_tokens(token_lines));
        //        let page_width = Mm(210.0);
        //        let page_height = Mm(297.0);
        //        let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", page_width, page_height, "Layer 1");
        ////        let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");
        //        let current_layer = doc.get_page(page1).get_layer(layer1);
        //
        ////        let font_times_roman = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
        ////        let font_helvetica = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        //        let data = include_bytes!("../../../resources/fonts/Libre-Baskerville/LibreBaskerville-Regular.ttf");
        //        let mut slice: &[u8] = data;
        ////        let font_libre_baskerville_regular = doc.add_external_font(&mut slice).unwrap();
        ////        let font = doc.add_external_font(File::open("../resources/fonts/Libre-Baskerville/LibreBaskerville-Regular.ttf").unwrap()).unwrap();
        ////        let font = doc.add_external_font(File::open("../resources/fonts/RobotoMedium.ttf").unwrap()).unwrap();
        ////        let font2 = doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
        //
        //
        //// text, font size, x from left edge, y from top edge, font
        ////        let stylesheets: Styles = Styles::new(
        ////            None,
        ////            None,
        ////            Some(Style { font, line_height: Mm(5.0), font_size: 10 }),
        ////            None,
        ////            PageSize::new(page_width, page_height),
        ////        );
        //
        ////        let stylesheets = Styles {
        ////            page: (page_width, page_height),
        ////            chord: Some(Style { font: font.clone(), line_height: Mm(5.0), font_size: 10 }),
        ////            chorus: Some(Style { font: font_helvetica.clone(), line_height: Mm(5.0), font_size: 10 }),
        ////            verse: Some(Style { font: font_helvetica.clone(), line_height: Mm(5.0), font_size: 10 }),
        ////            headline: Some(Style { font: font_helvetica.clone(), line_height: Mm(10.0), font_size: 20 }),
        ////        };
        //
        //        let mut font_reader = ::std::io::Cursor::new(include_bytes!("../../../resources/fonts/RobotoMedium.ttf").as_ref());
        //
        //        let font = doc.add_external_font(&mut font_reader).unwrap();
        //
        //        // `use_text` is a wrapper around making a simple string
        //        current_layer.use_text(text, 48, Mm(10.0), Mm(200.0), &font);

        //        current_layer.use_text(text, 10, Mm(0.0), page_height - Mm(20.0), &font);

        //        let builder = PdfBuilder::new(Coordinates::new(Mm(0.0), page_height - Mm(20.0)), stylesheets, doc, current_layer);
        //        let doc = builder.build_for_node(&node).unwrap();

        doc.save(&mut BufWriter::new(
            File::create("test_working.pdf").unwrap(),
        ))
        .unwrap();

        println!("{:?}", ::std::env::current_dir().unwrap());

        //        assert!(result.is_ok());
    }

    #[test]
    fn main_test() {
        use printpdf::*;
        use std::fs::File;
        use std::io::BufWriter;

        let (doc, page1, layer1) =
            PdfDocument::new("PDF_Document_title", Mm(500.0), Mm(300.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let text = "Lorem ipsum";
        let text2 = "dolor sit amet";

        let mut font_reader = std::io::Cursor::new(
            include_bytes!("../../../resources/fonts/RobotoMedium.ttf").as_ref(),
        );

        let font = doc.add_external_font(&mut font_reader).unwrap();

        // `use_text` is a wrapper around making a simple string
        current_layer.use_text(text, 48, Mm(10.0), Mm(200.0), &font);

        // text fill color = blue
        let blue = Rgb::new(13.0 / 256.0, 71.0 / 256.0, 161.0 / 256.0, None);
        let orange = Rgb::new(244.0 / 256.0, 67.0 / 256.0, 54.0 / 256.0, None);
        current_layer.set_fill_color(Color::Rgb(blue));
        current_layer.set_outline_color(Color::Rgb(orange));

        // For more complex layout of text, you can use functions
        // defined on the PdfLayerReference
        // Make sure to wrap your commands
        // in a `begin_text_section()` and `end_text_section()` wrapper
        current_layer.begin_text_section();

        // setup the general fonts.
        // see the docs for these functions for details
        current_layer.set_font(&font, 33);
        current_layer.set_text_cursor(Mm(10.0), Mm(100.0));
        current_layer.set_line_height(33);
        current_layer.set_word_spacing(3000);
        current_layer.set_character_spacing(10);

        // write two lines (one line break)
        current_layer.write_text(text, &font);
        current_layer.add_line_break();
        current_layer.write_text(text2, &font);
        current_layer.add_line_break();

        current_layer.set_text_rendering_mode(TextRenderingMode::FillStroke);
        current_layer.set_character_spacing(0);
        current_layer.set_text_matrix(TextMatrix::Rotate(10.0));

        // write one line, but write text2 in superscript
        current_layer.write_text(text, &font);
        current_layer.set_line_offset(10);
        current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
        current_layer.set_font(&font, 18);
        current_layer.write_text(text2, &font);

        current_layer.end_text_section();

        doc.save(&mut BufWriter::new(File::create("test_fonts.pdf").unwrap()))
            .unwrap();
    }
}
