use crate::tokenizer::Token;
use crate::converter::html::escape::Escape;
use crate::parser::Node;

pub struct TagBuilder {}

fn escape_html(input: &String) -> String {
    format!("{}", Escape(input))
}

impl TagBuilder {
    pub fn new() -> Self {
        TagBuilder {}
    }

    pub fn build_tag_for_token(&self, token: &Token) -> String {
        format!(
            "{}{}{}",
            self.opening_tag_for_token(token),
            self.content_for_token(token),
            self.closing_tag_for_token(token),
        )
    }

    pub fn build_tag_for_node(&self, node: &Node) -> String {
        match node {
            Node::Newline => "<hr/>\n".to_owned(),
            Node::ChordTextPair { chord, text } => {
                self.build_column(
                    self.build_tag_for_token(chord),
                    self.build_tag_for_token(text),
                )
            }
            Node::ChordStandalone(chord) => {
                self.build_column(
                    self.build_tag_for_token(chord),
                    "",
                )
            }
            Node::Text(text) => {
                self.build_column(
                    "",
                    self.build_tag_for_token(text),
                )
            }
            Node::Document(children) => {
                self.build_tag_for_children(children)
            }
            Node::Headline(token) => {
                self.build_tag_for_token(token)
            }
            Node::Quote(token) => {
                self.build_tag_for_token(token)
            }
            Node::Section { head, children } => {
                let head_content = if let Some(head) = head {
                    self.build_tag_for_node(head)
                } else {
                    "".into()
                };

                format!(
                    "<section>{}{}</section>",
                    head_content,
                    self.build_tag_for_children(children)
                )
            }
        }

//        format!(
//            "{}{}{}",
//            self.opening_tag_for_node(node),
//            self.content_for_node(node),
//            self.closing_tag_for_node(node),
//        )
    }

    fn build_tag_for_children(&self, children: &Vec<Node>) -> String {
        children.iter().map(|n| self.build_tag_for_node(n)).collect::<Vec<String>>().join("")
    }

    fn build_column<S1: Into<String>, S2: Into<String>>(&self, row1: S1, row2: S2) -> String {
        let row1string = row1.into();
        let row1text = if row1string.is_empty() { "&nbsp;".to_owned() } else { row1string };

        let row2string = row2.into();
        let row2text = if row2string.is_empty() { "&nbsp;".to_owned() } else { row2string };

        format!(
            "<div class='col'><table><tr><td>{}</td></tr><tr><td>{}</td></tr></table></div>",
            row1text,
            row2text
        )
    }

    fn opening_tag_for_token(&self, token: &Token) -> String {
        match token {
            Token::Chord(c) => format!("<span data-chord={} class=\"chordr-chord\">", c),
            Token::Literal(_) => "<span>".to_owned(),
            Token::Newline => "".to_owned(),
            Token::Quote(_) => "<blockquote>".to_owned(),
            Token::Headline { level, text: _ } => format!("<h{}>", level)
        }
    }

    fn content_for_token(&self, token: &Token) -> String {
        match token {
            Token::Newline => "<br/>".to_owned(),
            Token::Chord(c) => escape_html(c),
            Token::Literal(c) => escape_html(c),
            Token::Quote(c) => escape_html(c),
            Token::Headline { level: _, text: c } => escape_html(c),
        }
    }

    fn closing_tag_for_token(&self, token: &Token) -> String {
        match token {
            Token::Chord(_) => "</span>".to_owned(),
            Token::Literal(_) => "</span>".to_owned(),
            Token::Newline => "".to_owned(),
            Token::Quote(_) => "</blockquote>".to_owned(),
            Token::Headline { level, text: _ } => format!("</h{}>", level)
        }
    }

//    fn opening_tag_for_node(&self, node: &Node) -> String {
//        match node {
//            Node::Newline => "".to_owned(),
//            Node::ChordTextPair { chord, text } => {
//                format!(
//                    "<table><tr><td>{}</td></tr><tr><td>{}</td></tr></table>",
//                    self.build_tag_for_token(chord),
//                    self.build_tag_for_token(text)
//                )
//            }
//            Node::ChordStandalone(chord) => {
//                format!(
//                    "<table><tr><td>{}</td></tr><tr><td></td></tr></table>",
//                    self.build_tag_for_token(chord),
//                )
//            }
//            Node::Text(text) => {
//                format!(
//                    "<table><tr><td></td></tr><tr><td>{}</td></tr></table>",
//                    self.build_tag_for_token(text)
//                )
//            }
//            Node::Document(_) => { unreachable!() }
//            Node::Headline(_) => { unreachable!() }
//            Node::Quote(_) => { unreachable!() }
//            Node::Section { head: _, children: _ } => { unreachable!() }
//        }
//    }

//    fn content_for_node(&self, node: &Node) -> String {
//        format!("")
////        match node {
////            Token::Newline => "<br/>".to_owned(),
////            Token::Chord(c) => escape_html(c),
////            Token::Literal(c) => escape_html(c),
////            Token::Quote(c) => escape_html(c),
////            Token::Headline { level: _, text: c } => escape_html(c),
////        }
//    }

//    fn closing_tag_for_node(&self, node: &Node) -> String {
//        format!("")
////        match node {
////            Token::Chord(_) => "</span>".to_owned(),
////            Token::Literal(_) => "</span>".to_owned(),
////            Token::Newline => "".to_owned(),
////            Token::Quote(_) => "</blockquote>".to_owned(),
////            Token::Headline { level, text: _ } => format!("</h{}>", level)
////        }
//    }
}
