use crate::prelude::*;
use crate::converter::ConverterTrait;
use crate::tokenizer::{Token, Directive};

pub struct HtmlConverter {}

impl HtmlConverter {
    fn html_for_token(&self, tag_builder: &TagBuilder, token: &Token) -> String {
        format!(
            "{}{}{}",
            tag_builder.opening_tag_for_token(token),
            tag_builder.content_for_token(token),
            tag_builder.closing_tag_for_token(token),
        )
    }
}


impl ConverterTrait for HtmlConverter {
    fn convert(&self, tokens: &Vec<Token>, format: Format) -> Result<String> {
        let mut buffer = String::new();
        let tag_builder = TagBuilder::new();
        for token in tokens {
            buffer.push_str(HtmlConverter::html_for_token(self, &tag_builder, token).as_str());
        }

        Ok(buffer)
    }
}

struct TagBuilder {}

impl TagBuilder {
    fn new() -> Self {
        TagBuilder {}
    }
    fn css_class_for_directive(&self, directive: &Directive) -> String {
        match directive {
            Directive::Title(_) => "chordr-directive-title".to_owned(),
            Directive::Subtitle(_) => "chordr-directive-subtitle".to_owned(),
            Directive::Artist(_) => "chordr-directive-artist".to_owned(),
            Directive::Composer(_) => "chordr-directive-composer".to_owned(),
            Directive::Lyricist(_) => "chordr-directive-lyricist".to_owned(),
            Directive::Copyright(_) => "chordr-directive-copyright".to_owned(),
            Directive::Album(_) => "chordr-directive-album".to_owned(),
            Directive::Year(_) => "chordr-directive-year".to_owned(),
            Directive::Key(_) => "chordr-directive-key".to_owned(),
            Directive::Time(_) => "chordr-directive-time".to_owned(),
            Directive::Tempo(_) => "chordr-directive-tempo".to_owned(),
            Directive::Duration(_) => "chordr-directive-duration".to_owned(),
            Directive::Capo(_) => "chordr-directive-capo".to_owned(),
            Directive::Meta(_) => "chordr-directive-meta".to_owned(),
            Directive::Comment(_) => "chordr-comment".to_owned(),
            Directive::CommentItalic(_) => "chordr-comment -italic".to_owned(),
            Directive::CommentBox(_) => "chordr-comment -box".to_owned(),
            Directive::Image(_) => "chordr-image".to_owned(),
            Directive::StartOfChorus(c) | Directive::Chorus(c) => format!("chordr-chorus {}", self.escape_quotes(c)),
            Directive::StartOfVerse(c) => format!("chordr-verse {}", self.escape_quotes(c)),
            Directive::StartOfTab(c) => format!("chordr-tab {}", self.escape_quotes(c)),
            Directive::StartOfGrid(c) => format!("chordr-grid {}", self.escape_quotes(c)),
            Directive::Custom(ref c) => self.escape_quotes(c),
            _ => "".to_owned()
        }
    }

    fn content_for_directive(&self, directive: &Directive) -> String {
        let es = "".to_owned();
        let class = self.css_class_for_directive(directive);

        match directive {
            // Preamble directives
            Directive::NewSong => es,

            // Meta data
            Directive::Title(ref c) =>
                format!("<h1 class=\"{class}\">{content}</h1>", class = class, content = self.escape_node(c)),
            Directive::Subtitle(ref c) =>
                format!("<h2 class=\"{class}\">{content}</h2>", class = class, content = self.escape_node(c)),
            Directive::Artist(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Composer(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Lyricist(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Copyright(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Album(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Year(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Key(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Time(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Tempo(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Duration(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Capo(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Meta(ref c) =>
                format!("<tag>{}</tag>", self.escape_node(c)),

            // Formatting
            Directive::Comment(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::CommentItalic(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::CommentBox(ref c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = self.escape_node(c)),
            Directive::Image(ref c) =>
                format!("<img class=\"{class}\" {content} />", class = class, content = self.escape_quotes(c)),

            // Environment directives
            Directive::StartOfChorus(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::EndOfChorus =>
                format!("<tag></tag>"),
            Directive::Chorus(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::StartOfVerse(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::EndOfVerse =>
                format!("<tag></tag>"),
            Directive::StartOfTab(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::EndOfTab =>
                format!("<tag></tag>"),
            Directive::StartOfGrid(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::EndOfGrid =>
                format!("<tag></tag>"),

            // Chord diagrams
            Directive::Define(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Chord(ref c) =>
                format!("<tag>{}</tag>", c),

            // Fonts, sizes and colours
            Directive::Textfont(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Textsize(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Textcolour(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Chordfont(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Chordsize(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Chordcolour(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Tabfont(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Tabsize(ref c) =>
                format!("<tag>{}</tag>", c),
            Directive::Tabcolour(ref c) =>
                format!("<tag>{}</tag>", c),

            // Output related directives
            Directive::NewPage => es,
            Directive::NewPhysicalPage => es,
            Directive::ColumnBreak => es,

            // Custom extensions
            Directive::Custom(ref c) =>
                format!("<tag>{}</tag>", c),
        }
    }
    fn opening_tag_for_token(&self, token: &Token) -> String {
        match token {
            Token::Chord(ref c) => format!("<span data-chord={} class=\"chordr-chord\">", c),
            Token::Directive(_) => "".to_owned(),
            Token::Literal(_) => "<span>".to_owned(),
            Token::Newline => "".to_owned(),
            Token::Comment(_) => "<!-- ".to_owned(),
        }
    }
    fn content_for_token(&self, token: &Token) -> String {
        match token {
            Token::Directive(ref d) => self.content_for_directive(d),
            Token::Newline => "<br/>\n".to_owned(),
            Token::Chord(c) => self.escape_node(c),
            Token::Literal(c) => self.escape_node(c),
            Token::Comment(c) => self.escape_node(c),
        }
    }

    fn closing_tag_for_token(&self, token: &Token) -> &str {
        match token {
            Token::Chord(_) => "</span>",
            Token::Directive(_) => "",
            Token::Literal(_) => "</span>",
            Token::Newline => "",
            Token::Comment(_) => "-->",
        }
    }

    fn escape_node(&self, input: &String) -> String {
        format!("{}", input)
    }

    fn escape_quotes(&self, input: &String) -> String {
        format!("{}", input)
    }
}
