use crate::tokenizer::{Token, Directive};
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
            Directive::StartOfChorus(c) | Directive::Chorus(c) => format!("chordr-chorus {}", escape_html(c)),
            Directive::StartOfVerse(c) => format!("chordr-verse {}", escape_html(c)),
            Directive::StartOfTab(c) => format!("chordr-tab {}", escape_html(c)),
            Directive::StartOfGrid(c) => format!("chordr-grid {}", escape_html(c)),
            Directive::Custom(c) => escape_html(c),
            _ => "".to_owned()
        }
    }

    fn tag_for_directive(&self, directive: &Directive) -> String {
        let es = "".to_owned();
        let class = self.css_class_for_directive(directive);

        match directive {
            // Preamble directives
            Directive::NewSong => es,

            // Meta data
            Directive::Title(c) =>
                format!("<h1 class=\"{class}\">{content}</h1>", class = class, content = escape_html(c)),
            Directive::Subtitle(c) =>
                format!("<h2 class=\"{class}\">{content}</h2>", class = class, content = escape_html(c)),
            Directive::Artist(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Composer(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Lyricist(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Copyright(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Album(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Year(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Key(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Time(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Tempo(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Duration(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Capo(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Meta(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),

            // Formatting
            Directive::Comment(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::CommentItalic(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::CommentBox(c) =>
                format!("<p class=\"{class}\">{content}</p>", class = class, content = escape_html(c)),
            Directive::Image(c) =>
                format!("<img class=\"{class}\" {content} />", class = class, content = escape_html(c)),

            // Environment directives
            Directive::StartOfChorus(c) => format!("<h3 class=\"{class}\">{content}</h3>", class = class, content = escape_html(c)),
            Directive::EndOfChorus => es,
            Directive::Chorus(c) => format!("<h3 class=\"{class}\"><!-- Insert chorus -->{content}</h3>", class = class, content = escape_html(c)),
            Directive::StartOfVerse(c) => format!("<h3 class=\"{class}\">{content}</h3>", class = class, content = escape_html(c)),
            Directive::EndOfVerse => es,
            Directive::StartOfTab(c) => format!("<h3 class=\"{class}\">{content}</h3>", class = class, content = escape_html(c)),
            Directive::EndOfTab => es,
            Directive::StartOfGrid(c) => format!("<h3 class=\"{class}\">{content}</h3>", class = class, content = escape_html(c)),
            Directive::EndOfGrid => es,

            // Chord diagrams
            Directive::Define(c) => format!("<!-- Define {} -->", escape_html(c)),
            Directive::Chord(c) => format!("<!-- Chord {} -->", escape_html(c)),

            // Fonts, sizes and colours
            Directive::Textfont(c) => format!("<!-- Textfont: {} -->", escape_html(c)),
            Directive::Textsize(c) => format!("<!-- Textsize: {} -->", escape_html(c)),
            Directive::Textcolour(c) => format!("<!-- Textcolour: {} -->", escape_html(c)),
            Directive::Chordfont(c) => format!("<!-- Chordfont: {} -->", escape_html(c)),
            Directive::Chordsize(c) => format!("<!-- Chordsize: {} -->", escape_html(c)),
            Directive::Chordcolour(c) => format!("<!-- Chordcolour: {} -->", escape_html(c)),
            Directive::Tabfont(c) => format!("<!-- Tabfont: {} -->", escape_html(c)),
            Directive::Tabsize(c) => format!("<!-- Tabsize: {} -->", escape_html(c)),
            Directive::Tabcolour(c) => format!("<!-- Tabcolour: {} -->", escape_html(c)),

            // Output related directives
            Directive::NewPage => es,
            Directive::NewPhysicalPage => es,
            Directive::ColumnBreak => es,

            // Custom extensions
            Directive::Custom(c) => format!("<!-- Custom: {} -->", escape_html(c))
        }
    }

    fn opening_tag_for_token(&self, token: &Token) -> String {
        match token {
            Token::Chord(c) => format!("<span data-chord={} class=\"chordr-chord\">", c),
            Token::Directive(_) => "".to_owned(),
            Token::Literal(_) => "<span>".to_owned(),
            Token::Newline => "".to_owned(),
            Token::Comment(_) => "<!-- ".to_owned(),
        }
    }

    fn content_for_token(&self, token: &Token) -> String {
        match token {
            Token::Directive(ref d) => self.tag_for_directive(d),
            Token::Newline => "<br/>\n".to_owned(),
            Token::Chord(c) => escape_html(c),
            Token::Literal(c) => escape_html(c),
            Token::Comment(c) => escape_html(c),
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
}
