use crate::tokenizer::Token;
use crate::converter::html::escape::Escape;

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
}
