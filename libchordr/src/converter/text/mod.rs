use std::ops::Add;

use crate::error::Result;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::NoteDisplay;
use crate::models::metadata::MetadataTrait;
use crate::parser::Node;
use crate::tokenizer::Token;

use super::ConverterTrait;

pub struct TextConverter {}

impl ConverterTrait for TextConverter {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Result<String> {
        let output = format!(
            "{}\n{}",
            self.build_meta(meta, formatting),
            self.build_node(node)?
        );

        Ok(cleanup_output(&output))
    }
}

impl TextConverter {
    fn build_node<'a>(&'a self, node: &'a Node) -> Result<String> {
        match node {
            Node::ChordTextPair {
                chords: _,
                text,
                last_in_line: _,
            } => Ok(self.build_text(text)),
            Node::Text(text) => Ok(self.build_text(text)),
            Node::Document(children) => Ok(self.build_tag_for_children(children)),
            Node::Newline => Ok("\n".to_owned()),
            Node::Section {
                head: _,
                children,
                section_type: _,
            } => Ok(self.build_tag_for_children(children)),

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

    fn build_meta(&self, metadata: &dyn MetadataTrait, formatting: Formatting) -> String {
        let mut buffer: Vec<String> = vec![];
        if let Some(v) = metadata.title() {
            buffer.push(v.to_owned())
        }
        if let Some(v) = metadata.subtitle() {
            buffer.push(format!("Subtitle: {}", v))
        }
        if let Some(v) = metadata.original_title() {
            buffer.push(format!("Original Title: {}", v))
        }
        if let Some(v) = metadata.alternative_title() {
            buffer.push(format!("Alternative Title: {}", v))
        }
        if let Some(v) = metadata.artist() {
            buffer.push(format!("Artist: {}", v))
        }
        if let Some(v) = metadata.composer() {
            buffer.push(format!("Composer: {}", v))
        }
        if let Some(v) = metadata.lyricist() {
            buffer.push(format!("Lyricist: {}", v))
        }
        if let Some(v) = metadata.copyright() {
            buffer.push(format!("Copyright: {}", v))
        }
        if let Some(v) = metadata.album() {
            buffer.push(format!("Album: {}", v))
        }
        if let Some(v) = metadata.year() {
            buffer.push(format!("Year: {}", v))
        }
        if let Some(v) = metadata.key() {
            buffer.push(format!("Key: {}", v.note_format(formatting)))
        }
        if let Some(v) = metadata.time() {
            buffer.push(format!("Time: {}", v))
        }
        if let Some(v) = metadata.tempo() {
            buffer.push(format!("Tempo: {}", v))
        }
        if let Some(v) = metadata.duration() {
            buffer.push(format!("Duration: {}", v))
        }
        if let Some(v) = metadata.capo() {
            buffer.push(format!("Capo: {}", v))
        }
        if let Some(v) = metadata.ccli_song_id() {
            buffer.push(format!("CCLI Song ID: {}", v))
        }
        //        metadata.b_notation()  // -> BNotation;
        buffer.join("\n")
    }

    fn build_tag_for_children<'a>(&'a self, children: &'a [Node]) -> String {
        let output: String = children
            .iter()
            .filter_map(|n| self.build_node(n).ok())
            .collect::<Vec<String>>()
            .join("");

        output.add("\n")
    }
}

fn cleanup_output(output: &str) -> String {
    format!("{}\n", &remove_blank_lines(output).trim())
}

fn remove_blank_lines(input: &str) -> String {
    if input.contains("\n\n\n") {
        remove_blank_lines(&input.replace("\n\n\n", "\n\n"))
    } else {
        input.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Format;
    use crate::parser::Metadata;
    use crate::test_helpers::get_test_metadata;
    use crate::test_helpers::{
        get_test_ast, get_test_ast_w_inline_metadata, get_test_ast_with_quote,
    };

    use super::*;

    #[test]
    fn test_convert() {
        let converter = TextConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &Metadata::default(),
            Formatting::new_with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.

I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home.
"#
        );
    }

    #[test]
    fn test_convert_w_metadata() {
        let converter = TextConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &get_test_metadata(),
            Formatting::new_with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"Great new song
Subtitle: Originally known as 'Swing low sweet chariot'
Artist: Me
Composer: Wallace Willis
Lyricist: Wallace Willis
Year: 1865
Capo: 1

Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.

I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home.
"#
        );
    }

    #[test]
    fn test_convert_w_inline_metadata() {
        let converter = TextConverter {};
        let ast = get_test_ast_w_inline_metadata();
        let result = converter.convert(
            &ast,
            &get_test_metadata(),
            Formatting::new_with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"Great new song
Subtitle: Originally known as 'Swing low sweet chariot'
Artist: Me
Composer: Wallace Willis
Lyricist: Wallace Willis
Year: 1865
Capo: 1

Swing low, sweet chariot.
"#
        );
    }

    #[test]
    fn test_convert_w_quote() {
        let converter = TextConverter {};
        let ast = get_test_ast_with_quote();
        let result = converter.convert(
            &ast,
            &Metadata::default(),
            Formatting::new_with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"Swing low, sweet chariot.
"#
        );
    }
}
