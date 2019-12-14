mod node;
mod meta;
mod parser_result;

use std::iter::Peekable;
use std::vec::IntoIter;
use crate::tokenizer::Token;
pub use self::node::Node;
pub use self::meta::Meta;
pub use self::parser_result::ParserResult;


pub struct Parser {
    meta: Meta
}

impl Parser {
    pub fn new() -> Self {
        Self { meta: Meta::default() }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> ParserResult {
        let mut tokens_iterator = tokens.into_iter().peekable();

        let mut elements = vec![];

        while let Some(token) = tokens_iterator.next() {
            elements.push(self.visit(token, &mut tokens_iterator));
        }

        ParserResult::new(Node::Document(elements), self.meta.clone())
    }

    fn visit(&mut self, token: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Node {
        match token {
            Token::Chord(_) => {
                if let Some(next) = tokens.peek() {
                    if let Token::Literal(_) = next {
                        return Node::ChordTextPair {
                            chord: token,
                            text: tokens.next().unwrap(),
                        };
                    }
                }
                return Node::ChordStandalone(token);
            }
            Token::Headline { level, ref text } => {
                if level == 1 {
                    self.meta.title = Some(text.clone())
                }
                let head = Some(Box::new(Node::Headline(token)));

                if tokens.peek().is_some() {
                    // Collect children
                    let mut children = vec![];
                    while let Some(token) = tokens.peek() {
                        if token_is_start_of_section(token) {
                            break;
                        }
                        children.push(self.visit(tokens.next().unwrap(), tokens));
                    }

                    Node::Section { head, children }
                } else {
                    Node::Section {
                        head,
                        children: vec![],
                    }
                }
            }

            Token::Literal(_) => Node::Text(token),
            Token::Quote(_) => Node::Quote(token),
            Token::Newline => Node::Newline,
        }
    }
}

fn token_is_start_of_section(token: &Token) -> bool {
    match token {
        Token::Headline { level: _, text: _ } => true,
        Token::Quote(_) => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = Parser::new();
        let result = parser.parse(vec![
            Token::headline(1, "Swing Low Sweet Chariot"),
            Token::newline(),
            Token::headline(2, "Chorus"),
            Token::literal("Swing "),
            Token::chord("D"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::literal("Comin’ for to carry me "),
            Token::chord("A7"),
            Token::literal("home."),
            Token::literal("Swing "),
            Token::chord("D7"),
            Token::headline(2, "Verse"),
            Token::chord("E"),
            Token::literal("low, sweet "),
            Token::chord("G"),
            Token::literal("chari"),
            Token::chord("D"),
            Token::literal("ot,"),
            Token::chord("E"),
            Token::chord("A"),
            Token::newline(),
            Token::chord("B"),
            Token::chord("H"),
        ]);

        assert_eq!(Some("Swing Low Sweet Chariot".to_string()), result.meta().title);

        let ast = result.node();

        let expected_ast = Node::Document(vec![
            Node::section(1, "Swing Low Sweet Chariot", vec![
                Node::newline()
            ]),
            Node::section(2, "Chorus", vec![
                Node::text("Swing "),
                Node::chord_text_pair("D", "low, sweet "),
                Node::chord_text_pair("G", "chari"),
                Node::chord_text_pair("D", "ot,"),
                Node::text("Comin’ for to carry me "),
                Node::chord_text_pair("A7", "home."),
                Node::text("Swing "),
                Node::chord_standalone("D7"),
            ]),
            Node::section(2, "Verse", vec![
                Node::chord_text_pair("E", "low, sweet "),
                Node::chord_text_pair("G", "chari"),
                Node::chord_text_pair("D", "ot,"),
                Node::chord_standalone("E"),
                Node::chord_standalone("A"),
                Node::newline(),
                Node::chord_standalone("B"),
                Node::chord_standalone("H"),
            ]),
        ]);

        assert_eq!(expected_ast, ast);
    }
}
