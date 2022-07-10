use crate::error::Result;
use crate::models::chord::fmt::Formatting;
use crate::models::chord::NoteDisplay;
use crate::models::meta::MetaTrait;
use crate::models::structure::{ReferenceResolver, Section, SectionProvider};
use crate::parser::Node;
use crate::tokenizer::Token;

use super::ConverterTrait;

/// Converter to build song files for [SongBeamer](https://www.songbeamer.de/)
///
/// For an unofficial specification see https://gitlab.com/openlp/wiki/-/wikis/Development/SongBeamer_-_Song_Data_Format
pub struct SongBeamerConverter {}

const BOM: &str = "\u{feff}";

impl ConverterTrait for SongBeamerConverter {
    fn convert(&self, node: &Node, meta: &dyn MetaTrait, formatting: Formatting) -> Result<String> {
        let output = format!(
            "{}\n{}\n{}",
            self.build_std_meta(node),
            self.build_meta(meta, formatting),
            self.build_node(node).unwrap_or_default()
        );

        Ok(format!("{}{}", BOM, cleanup_output(&output)))
    }
}

impl SongBeamerConverter {
    fn build_node(&self, node: &Node) -> Option<String> {
        match node {
            Node::ChordTextPair {
                chords: _,
                text,
                last_in_line: _,
            } => Some(self.build_text(text)),
            Node::Text(text) => Some(self.build_text(text)),
            Node::Document(children) => self.build_content_for_children(children),
            Node::Newline => Some("\n".to_owned()),
            Node::Section {
                head: _,
                children,
                section_type: _,
            } => {
                let head =
                    SectionProvider::get_section(node).map_or("".to_string(), |s| s.title + "\n");
                let content = self.build_content_for_children(children);
                content.map(|content| format!("---\n{}{}\n", head, content.trim()))
            }

            Node::Headline(_) => None,
            Node::ChordStandalone(_) => None,
            Node::Quote(_) => None,
            Node::Meta(_) => {
                // Meta data must have been already output
                None
            }
        }
    }

    fn build_text(&self, token: &Token) -> String {
        if let Token::Literal(c) = token {
            c.to_owned()
        } else {
            unreachable!("Invalid Token given")
        }
    }

    fn build_std_meta(&self, node: &Node) -> String {
        format!(
            r"#LangCount=1
#Editor=Chordr
#Version=3
#VerseOrder={}",
            self.build_verse_order(node)
        )
    }

    fn build_meta(&self, meta: &dyn MetaTrait, formatting: Formatting) -> String {
        let mut buffer: Vec<String> = vec![];

        if let Some(v) = meta.title() {
            buffer.push(format!("#Title={}", v))
        }
        if let Some(v) = meta.original_title() {
            buffer.push(format!("#OTitle={}", v))
        }
        if let Some(v) = meta.subtitle() {
            buffer.push(format!("#Subtitle={}", v))
        }
        if let Some(v) = meta.artist() {
            buffer.push(format!("#Artist={}", v))
        }
        if let Some(v) = meta.composer() {
            buffer.push(format!("#Melody={}", v))
        }
        if let Some(v) = meta.lyricist() {
            buffer.push(format!("#Author={}", v))
        }
        if let Some(v) = meta.copyright() {
            buffer.push(format!("#(c)={}", v))
        }
        if let Some(v) = meta.album() {
            buffer.push(format!("#Album={}", v))
        }
        if let Some(v) = meta.year() {
            buffer.push(format!("#Year={}", v))
        }
        if let Some(v) = meta.key() {
            buffer.push(format!("#Key={}", v.note_format(formatting)))
        }
        if let Some(v) = meta.time() {
            buffer.push(format!("#Time={}", v))
        }
        if let Some(v) = meta.tempo() {
            buffer.push(format!("#Tempo={}", v))
        }
        if let Some(v) = meta.duration() {
            buffer.push(format!("#Duration={}", v))
        }
        if let Some(v) = meta.capo() {
            buffer.push(format!("#Capo={}", v))
        }
        if let Some(v) = meta.ccli_song_id() {
            buffer.push(format!("#CCLI={}", v))
        }
        //        meta.b_notation()  // -> BNotation;
        buffer.join("\n")
    }

    fn build_content_for_children(&self, children: &[Node]) -> Option<String> {
        let trimmed = children
            .iter()
            .filter_map(|n| self.build_node(n))
            .collect::<String>()
            .trim()
            .to_string();

        if !trimmed.is_empty() {
            Some(trimmed)
        } else {
            None
        }
    }

    fn build_verse_order(&self, node: &Node) -> String {
        let reference_resolver = ReferenceResolver::new();
        let sections = node.get_sections();
        let get_section_title = |section: &Section| {
            if section.is_reference {
                let resolve_result = reference_resolver.resolve_reference(section, &sections);
                if let Some(referenced_section) = resolve_result {
                    return referenced_section.title.clone();
                }
            }

            section.title.clone()
        };

        sections
            .iter()
            .map(get_section_title)
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn cleanup_output(output: &str) -> String {
    remove_blank_lines(output)
}

fn remove_blank_lines(input: &str) -> String {
    if input.contains("\n\n") {
        remove_blank_lines(&input.replace("\n\n", "\n"))
    } else {
        input.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Format;
    use crate::parser::MetaInformation;
    use crate::test_helpers::get_test_metadata;
    use crate::test_helpers::{
        get_test_ast, get_test_ast_w_inline_metadata, get_test_ast_with_quote,
    };

    use super::*;

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
            BOM.to_owned()
                + r#"#LangCount=1
#Editor=Chordr
#Version=3
#VerseOrder=Chorus,Verse 1,Chorus
---
Chorus
Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.
---
Verse 1
I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home."#
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
            BOM.to_owned()
                + r#"#LangCount=1
#Editor=Chordr
#Version=3
#VerseOrder=Chorus,Verse 1,Chorus
#Title=Great new song
#Subtitle=Originally known as 'Swing low sweet chariot'
#Artist=Me
#Melody=Wallace Willis
#Author=Wallace Willis
#Year=1865
#Capo=1
---
Chorus
Swing low, sweet chariot,
Comin’ for to carry me home.
Swing low, sweet chariot,
Comin’ for to carry me home.
---
Verse 1
I looked over Jordan, and what did I see,
Comin’ for to carry me home.
A band of angels comin’ after me,
Comin’ for to carry me home."#
        );
    }

    #[test]
    fn test_convert_w_inline_metadata() {
        let converter = SongBeamerConverter {};
        let ast = get_test_ast_w_inline_metadata();
        let result = converter.convert(
            &ast,
            &get_test_metadata(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            BOM.to_owned()
                + r#"#LangCount=1
#Editor=Chordr
#Version=3
#VerseOrder=Chorus
#Title=Great new song
#Subtitle=Originally known as 'Swing low sweet chariot'
#Artist=Me
#Melody=Wallace Willis
#Author=Wallace Willis
#Year=1865
#Capo=1
---
Chorus
Swing low, sweet chariot."#,
        );
    }

    #[test]
    fn test_convert_w_quote() {
        let converter = SongBeamerConverter {};
        let ast = get_test_ast_with_quote();
        let result = converter.convert(
            &ast,
            &MetaInformation::default(),
            Formatting::with_format(Format::SongBeamer),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            BOM.to_owned()
                + r#"#LangCount=1
#Editor=Chordr
#Version=3
#VerseOrder=Chorus,Chorus
---
Chorus
Swing low, sweet chariot."#
        );
    }
}
