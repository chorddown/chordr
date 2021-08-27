use std::iter::Peekable;
use std::vec::IntoIter;

use crate::models::chord::Chords;
use crate::tokenizer::Token;

pub use super::meta_information::MetaInformation;
pub use super::node::Node;
pub use super::parser_result::ParserResult;
pub use super::section_type::SectionType;
pub use super::*;

pub struct NodeParser {
    b_notation: BNotation,
}

impl ParserTrait for NodeParser {
    type OkType = Node;

    fn parse(&mut self, tokens: Vec<Token>) -> Result<Self::OkType, Error> {
        let mut tokens_iterator = Parser::cleanup_tokens(tokens).into_iter().peekable();

        let mut elements = vec![];

        while let Some(token) = tokens_iterator.next() {
            elements.push(self.visit(token, &mut tokens_iterator)?);
        }

        Ok(Node::Document(elements))
    }
}

impl NodeParser {
    pub fn with_b_notation(b_notation: BNotation) -> Self {
        Self { b_notation }
    }

    fn visit(
        &mut self,
        token: Token,
        tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Node, Error> {
        log::trace!("Visit token: {:?}", token);
        match token {
            Token::Chord(_) => self.visit_chord(token, tokens),
            Token::Headline {
                level,
                text: _,
                modifier,
            } => {
                if level == 1 {}
                let head = Some(Box::new(Node::Headline(token)));

                if tokens.peek().is_some() {
                    // Collect children
                    let mut children = vec![];
                    while let Some(token) = tokens.peek() {
                        if token_is_start_of_section(token) {
                            break;
                        }
                        let result = self.visit(tokens.next().unwrap(), tokens)?;
                        children.push(result);
                    }

                    Ok(Node::Section {
                        head,
                        children,
                        section_type: modifier.into(),
                    })
                } else {
                    Ok(Node::Section {
                        head,
                        children: vec![],
                        section_type: modifier.into(),
                    })
                }
            }
            Token::Meta(meta) => Ok(Node::Meta(meta)),
            Token::Literal(_) => Ok(Node::Text(token)),
            Token::Quote(_) => Ok(Node::Quote(token)),
            Token::Newline => Ok(Node::Newline),
        }
    }

    fn visit_chord(
        &mut self,
        token: Token,
        tokens: &mut Peekable<IntoIter<Token>>,
    ) -> Result<Node, Error> {
        let chords_raw = if let Token::Chord(c) = token {
            c
        } else {
            unreachable!("Invalid Token given")
        };

        // TODO: Add relaxed parsing of chords like `[A ///]`
        let chord_result = Chords::try_from(&chords_raw, self.b_notation);

        let chords = match chord_result {
            Err(error) => {
                return match tokens.peek() {
                    Some(next) => Err(Error::chord_error(format!(
                        "{} (before token {:?})",
                        error, next
                    ))),
                    None => Err(error),
                };
            }

            Ok(chords) => chords,
        };

        if let Some(Token::Literal(_)) = tokens.peek() {
            // Consume the next token
            let text = tokens.next().unwrap();

            let last_in_line = matches!(tokens.peek(), Some(Token::Newline));

            return Ok(Node::ChordTextPair {
                chords,
                text,
                last_in_line,
            });
        }

        Ok(Node::ChordStandalone(chords))
    }
}

fn token_is_start_of_section(token: &Token) -> bool {
    match token {
        Token::Headline {
            level: _,
            text: _,
            modifier: _,
        } => true,
        Token::Quote(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{get_test_ast, get_test_tokens};

    use super::*;

    #[test]
    fn test_parse() {
        let mut parser = NodeParser::with_b_notation(BNotation::B);
        let result = parser.parse(get_test_tokens());

        assert!(result.is_ok());
        let ast = result.unwrap();

        assert_eq!(ast, get_test_ast());
    }
}
