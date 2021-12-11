use crate::error::Result;
use crate::models::chord::fmt::*;
use crate::models::chord::Chords;
use crate::models::metadata::MetadataTrait;
use crate::parser::Node;
use crate::tokenizer::Token;

use super::ConverterTrait;

pub struct ChorddownConverter {}

impl ConverterTrait for ChorddownConverter {
    fn convert(
        &self,
        node: &Node,
        meta: &dyn MetadataTrait,
        formatting: Formatting,
    ) -> Result<String> {
        let output = format!(
            "{}{}{}",
            self.build_title(meta),
            self.build_meta(meta, formatting),
            self.build_node(node, formatting)?
        );
        Ok(cleanup_output(&output))
    }
}

impl ChorddownConverter {
    fn build_node<'a>(&'a self, node: &'a Node, formatting: Formatting) -> Result<String> {
        match node {
            Node::ChordTextPair {
                chords,
                text,
                last_in_line: _,
            } => Ok(format!(
                "{}{}",
                self.build_string_for_chords(chords, formatting),
                self.build_token(text),
            )),
            Node::ChordStandalone(chord) => {
                Ok(self.build_column(self.build_string_for_chords(chord, formatting), ""))
            }
            Node::Text(text) => Ok(self.build_token(text)),
            Node::Document(children) => Ok(self.build_string_for_children(children, formatting)),
            Node::Headline(token) => Ok(self.build_token(token)),
            Node::Quote(token) => Ok(self.build_token(token)),
            Node::Meta(_) => {
                // Metadata must have already been rendered
                Ok(String::new())
            }
            Node::Newline => Ok("\n".to_owned()),
            Node::Section {
                head,
                children,
                section_type: _,
            } => {
                let inner = match head {
                    Some(head) => format!(
                        "{}{}",
                        self.build_node(head, formatting)?,
                        self.build_string_for_children(children, formatting)
                    ),
                    None => self.build_string_for_children(children, formatting),
                };

                Ok(format!("{}\n", inner))
            }
        }
    }

    fn build_token<'a>(&'a self, token: &'a Token) -> String {
        match token {
            Token::Literal(c) => c.to_owned(),
            Token::Quote(c) => format!("> {}\n", c.to_owned()),
            Token::Headline {
                level,
                text,
                modifier,
            } => {
                if *level == 1 {
                    String::new()
                } else {
                    format!("{}{} {}", "#".repeat(*level as usize), modifier, text)
                }
            }
            Token::Chord(_) => unreachable!(),
            Token::Newline => unreachable!(),
            Token::Metadata(_) => unreachable!(),
        }
    }

    fn build_title(&self, meta: &dyn MetadataTrait) -> String {
        match meta.title() {
            Some(t) => format!("# {}\n", t),
            None => String::new(),
        }
    }

    fn build_meta(&self, meta: &dyn MetadataTrait, formatting: Formatting) -> String {
        let mut buffer = String::new();
        if let Some(v) = meta.subtitle() {
            buffer.push_str("Subtitle: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.original_title() {
            buffer.push_str("Original Title: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.alternative_title() {
            buffer.push_str("Alternative Title: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.artist() {
            buffer.push_str("Artist: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.composer() {
            buffer.push_str("Composer: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.lyricist() {
            buffer.push_str("Lyricist: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.copyright() {
            buffer.push_str("Copyright: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.album() {
            buffer.push_str("Album: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.year() {
            buffer.push_str("Year: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.key() {
            buffer.push_str("Key: ");
            buffer.push_str(&v.note_format(formatting));
            buffer.push_str("\n")
        }
        if let Some(v) = meta.time() {
            buffer.push_str("Time: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.tempo() {
            buffer.push_str("Tempo: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.duration() {
            buffer.push_str("Duration: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.capo() {
            buffer.push_str("Capo: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        if let Some(v) = meta.ccli_song_id() {
            buffer.push_str("CCLI Song ID: ");
            buffer.push_str(&v);
            buffer.push_str("\n")
        }
        //        metadata.b_notation()  // -> BNotation;
        buffer.trim_end().to_string()
    }

    fn build_string_for_chords(&self, chords: &Chords, formatting: Formatting) -> String {
        format!("[{}]", chords.note_format(formatting))
    }

    fn build_string_for_children<'a>(
        &'a self,
        children: &'a [Node],
        formatting: Formatting,
    ) -> String {
        children
            .iter()
            .filter_map(|n| self.build_node(n, formatting).ok())
            .collect::<Vec<String>>()
            .join("")
    }

    fn build_column<S1: Into<String>, S2: Into<String>>(&self, chord: S1, lyric: S2) -> String {
        let chord_text = chord.into();
        let lyric_text = lyric.into();

        format!("{}{}", chord_text, lyric_text)
    }
}

fn cleanup_output(output: &str) -> String {
    format!("{}\n", remove_double_blank_lines(output).trim())
}

fn remove_double_blank_lines(input: &str) -> String {
    if input.contains("\n\n\n") {
        remove_double_blank_lines(&input.replace("\n\n\n", "\n\n"))
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
        let converter = ChorddownConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &Metadata::default(),
            Formatting::with_format(Format::Chorddown),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            //v-- The title is read from the parsed Meta Data. Here none was provided
            r"##! Chorus
Swing [D]low, sweet [G]chari[D]ot,
Comin’ for to carry me [A7]home.
Swing [D7]low, sweet [G]chari[D]ot,
Comin’ for to [A7]carry me [D]home.

## Verse 1
I [D]looked over Jordan, and [G]what did I [D]see,
Comin’ for to carry me [A7]home.
A [D]band of angels [G]comin’ after [D]me,
Comin’ for to [A7]carry me [D]home.

> Chorus
"
        );
    }

    #[test]
    fn test_convert_w_metadata() {
        let converter = ChorddownConverter {};
        let result = converter.convert(
            &get_test_ast(),
            &get_test_metadata(),
            Formatting::with_format(Format::Chorddown),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            //v-- The title is read from the parsed Meta Data
            r#"# Great new song
Subtitle: Originally known as 'Swing low sweet chariot'
Artist: Me
Composer: Wallace Willis
Lyricist: Wallace Willis
Year: 1865
Capo: 1

##! Chorus
Swing [D]low, sweet [G]chari[D]ot,
Comin’ for to carry me [A7]home.
Swing [D7]low, sweet [G]chari[D]ot,
Comin’ for to [A7]carry me [D]home.

## Verse 1
I [D]looked over Jordan, and [G]what did I [D]see,
Comin’ for to carry me [A7]home.
A [D]band of angels [G]comin’ after [D]me,
Comin’ for to [A7]carry me [D]home.

> Chorus
"#
        );
    }

    #[test]
    fn test_convert_w_inline_metadata() {
        let converter = ChorddownConverter {};
        let ast = get_test_ast_w_inline_metadata();
        let result = converter.convert(
            &ast,
            &get_test_metadata(),
            Formatting::with_format(Format::Chorddown),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            //v-- The title is read from the parsed Meta Data
            r#"# Great new song
Subtitle: Originally known as 'Swing low sweet chariot'
Artist: Me
Composer: Wallace Willis
Lyricist: Wallace Willis
Year: 1865
Capo: 1

##! Chorus
Swing [D]low, sweet [G]chari[D]ot.
"#
        );
    }

    #[test]
    fn test_convert_w_content_after_quote() {
        let converter = ChorddownConverter {};
        let ast = get_test_ast_with_quote();
        let result = converter.convert(
            &ast,
            &Metadata::default(),
            Formatting::with_format(Format::Chorddown),
        );

        assert!(result.is_ok());
        let source = result.unwrap();

        assert_eq!(
            source,
            r#"> Play slowly

##! Chorus
Swing [D]low, sweet [G]chari[D]ot.
"#
        );
    }
}
