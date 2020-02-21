use super::ConverterTrait;
use crate::error::Result;
use crate::models::chord::fmt::Formatting;
use crate::models::meta::MetaTrait;
use crate::parser::Node;
use crate::tokenizer::Token;

pub struct SongBeamerConverter {}

impl ConverterTrait for SongBeamerConverter {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn MetaTrait,
        _formatting: Formatting,
    ) -> Result<String> {
        let output = format!(
            "{}\n{}\n{}",
            self.build_std_meta(meta),
            self.build_meta(meta),
            self.build_node(node)?
        );

        // TODO: Encode for Windows usage
        // use encoding::{Encoding, EncoderTrap};
        // use encoding::all::ISO_8859_1;
        // println!("{:?}", ISO_8859_1.encode(output.as_str(), EncoderTrap::Ignore));

        Ok(cleanup_output(&output))
    }
}

impl SongBeamerConverter {
    fn build_node<'a>(&'a self, node: &'a Node) -> Result<String> {
        match node {
            Node::ChordTextPair { chords: _, text } => Ok(self.build_text(text)),
            Node::Text(text) => Ok(self.build_text(text)),
            Node::Document(children) => Ok(self.build_tag_for_children(children)),
            Node::Newline => Ok("\n".to_owned()),
            Node::Section {
                head: _,
                children,
                section_type: _,
            } => Ok(format!("---\n{}\n", self.build_tag_for_children(children))),

            Node::Headline(_) => Ok(String::new()),
            Node::ChordStandalone(_) => Ok(String::new()),
            Node::Quote(_) => Ok(String::new()),
            Node::Meta(_) => {
                // Meta data must have been already output
                Ok(String::new())
            }
        }
    }

    fn build_text<'a>(&'a self, token: &'a Token) -> String {
        if let Token::Literal(c) = token {
            c.to_owned()
        } else {
            unreachable!("Invalid Token given")
        }
    }

    fn build_std_meta(&self, _meta: &dyn MetaTrait) -> String {
        r"#LangCount=1
#Editor=Chordr
#Version=3"
            .to_owned()
    }

    fn build_meta(&self, meta: &dyn MetaTrait) -> String {
        let mut buffer: Vec<String> = vec![];

        if let Some(v) = meta.title() {
            buffer.push(format!("#OTitle={}", v))
        }
        // if let Some(v) = meta.subtitle() {
        //     buffer.push(format!("#Subtitle={}", v))
        // }
        // if let Some(v) = meta.artist() {
        //     buffer.push(format!("#Artist={}", v))
        // }
        if let Some(v) = meta.composer() {
            buffer.push(format!("#Melody={}", v))
        }
        if let Some(v) = meta.lyricist() {
            buffer.push(format!("#Author={}", v))
        }
        if let Some(v) = meta.copyright() {
            buffer.push(format!("#(c)={}", v))
        }
        // if let Some(v) = meta.album() {
        //     buffer.push(format!("#Album={}", v))
        // }
        // if let Some(v) = meta.year() {
        //     buffer.push(format!("#Year={}", v))
        // }
        // if let Some(v) = meta.key() {
        //     buffer.push(format!("#Key={}", v))
        // }
        // if let Some(v) = meta.time() {
        //     buffer.push(format!("#Time={}", v))
        // }
        // if let Some(v) = meta.tempo() {
        //     buffer.push(format!("#Tempo={}", v))
        // }
        // if let Some(v) = meta.duration() {
        //     buffer.push(format!("#Duration={}", v))
        // }
        // if let Some(v) = meta.capo() {
        //     buffer.push(format!("#Capo={}", v))
        // }
        //        meta.b_notation()  // -> BNotation;
        buffer.join("\n")
    }

    fn build_tag_for_children<'a, 'b>(&'a self, children: &'a Vec<Node>) -> String {
        children
            .iter()
            .filter_map(|n| self.build_node(n).ok())
            .collect::<Vec<String>>()
            .join("")
    }
}

fn cleanup_output(output: &str) -> String {
    format!(
        "{}\n",
        remove_blank_slides(&remove_blank_lines(output)).trim_end()
    )
}

fn remove_blank_lines(input: &str) -> String {
    if input.contains("\n\n") {
        remove_blank_lines(&input.replace("\n\n", "\n"))
    } else {
        input.to_owned()
    }
}

fn remove_blank_slides(input: &str) -> String {
    if input.contains("---\n---\n") {
        remove_blank_slides(&input.replace("---\n---\n", "---\n"))
    } else {
        input.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::Format;
    use crate::parser::MetaInformation;
    use crate::test_helpers::get_test_ast;
    use crate::test_helpers::get_test_metadata;
    use crate::tokenizer::Modifier;

    #[test]
    fn test_convert() {
        let converter = SongBeamerConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &MetaInformation::default(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"#LangCount=1
#Editor=Chordr
#Version=3
---
Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.
---
I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home.
"#
        );
    }

    #[test]
    fn test_convert_w_metadata() {
        let converter = SongBeamerConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &get_test_metadata(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"#LangCount=1
#Editor=Chordr
#Version=3
#OTitle=Great new song
#Melody=Wallace Willis
#Author=Wallace Willis
---
Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.
---
I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home.
"#
        );
    }

    #[test]
    fn test_convert_w_inline_metadata() {
        let converter = SongBeamerConverter {};
        let ast = Node::Document(vec![
            Node::section(
                1,
                "Swing Low Sweet Chariot",
                Modifier::None,
                vec![Node::newline()],
            ),
            Node::meta("Artist: The Fantastic Corns").unwrap(),
            Node::newline(),
            Node::meta("Composer: Daniel Corn").unwrap(),
            Node::newline(),
            Node::section(
                2,
                "Chorus",
                Modifier::Chorus,
                vec![
                    Node::newline(),
                    Node::text("Swing "),
                    Node::chord_text_pair("D", "low, sweet ").unwrap(),
                    Node::chord_text_pair("G", "chari").unwrap(),
                    Node::chord_text_pair("D", "ot.").unwrap(),
                ],
            ),
        ]);
        let result = converter.convert(
            &ast,
            &get_test_metadata(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"#LangCount=1
#Editor=Chordr
#Version=3
#OTitle=Great new song
#Melody=Wallace Willis
#Author=Wallace Willis
---
Swing low, sweet chariot.
"#
        );
    }

    #[test]
    fn test_convert_w_quote() {
        let converter = SongBeamerConverter {};
        let ast = Node::Document(vec![
            Node::section(
                1,
                "Swing Low Sweet Chariot",
                Modifier::None,
                vec![Node::newline()],
            ),
            Node::quote("Play slowly"),
            Node::newline(),
            Node::section(
                2,
                "Chorus",
                Modifier::Chorus,
                vec![
                    Node::newline(),
                    Node::text("Swing "),
                    Node::chord_text_pair("D", "low, sweet ").unwrap(),
                    Node::chord_text_pair("G", "chari").unwrap(),
                    Node::chord_text_pair("D", "ot.").unwrap(),
                ],
            ),
        ]);
        let result = converter.convert(
            &ast,
            &MetaInformation::default(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"#LangCount=1
#Editor=Chordr
#Version=3
---
Swing low, sweet chariot.
"#
        );
    }
}
