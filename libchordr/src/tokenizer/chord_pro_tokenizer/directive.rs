use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Directive {
    /// Preamble directives
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#preamble-directives
    NewSong,

    /// Meta data
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#meta-data-directives
    Title(String),
    Subtitle(String),
    Artist(String),
    Composer(String),
    Lyricist(String),
    Copyright(String),
    Album(String),
    Year(String),
    Key(String),
    Time(String),
    Tempo(String),
    Duration(String),
    Capo(String),
    Meta(String),

    /// Formatting
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#formatting-directives
    Comment(String),
    CommentItalic(String),
    CommentBox(String),
    Image(String),

    /// Environment directives
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#environment-directives
    StartOfChorus(String),
    EndOfChorus,
    Chorus(String),
    StartOfVerse(String),
    EndOfVerse,
    StartOfTab(String),
    EndOfTab,
    StartOfGrid(String),
    EndOfGrid,

    /// Chord diagrams
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#chord-diagrams
    Define(String),
    Chord(String),

    /// Fonts, sizes and colours
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#fonts-sizes-and-colours
    Textfont(String),
    Textsize(String),
    Textcolour(String),
    Chordfont(String),
    Chordsize(String),
    Chordcolour(String),
    Tabfont(String),
    Tabsize(String),
    Tabcolour(String),

    /// Output related directives
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#output-related-directives
    NewPage,
    NewPhysicalPage,
    ColumnBreak,

    /// Custom extensions
    ///
    /// https://www.chordpro.org/chordpro/ChordPro-Directives.html#custom-extensions
    Custom(String),
}

impl Directive {
    pub fn from_keyword<S: Into<String>>(keyword: &str, data: S) -> Self {
        match keyword {
            // Preamble directives
            "new_song" | "ns" => Directive::NewSong,

            // Meta data
            "title" | "t" => Directive::Title(data.into()),
            "subtitle" | "st" => Directive::Subtitle(data.into()),
            "artist" => Directive::Artist(data.into()),
            "composer" => Directive::Composer(data.into()),
            "lyricist" => Directive::Lyricist(data.into()),
            "copyright" => Directive::Copyright(data.into()),
            "album" => Directive::Album(data.into()),
            "year" => Directive::Year(data.into()),
            "key" => Directive::Key(data.into()),
            "time" => Directive::Time(data.into()),
            "tempo" => Directive::Tempo(data.into()),
            "duration" => Directive::Duration(data.into()),
            "capo" => Directive::Capo(data.into()),
            "meta" => Directive::Meta(data.into()),

            // Formatting
            "comment" | "c" => Directive::Comment(data.into()),
            "comment_italic" | "ci" => Directive::CommentItalic(data.into()),
            "comment_box" | "cb" => Directive::CommentBox(data.into()),
            "image" => Directive::Image(data.into()),


            // Environment directives
            "start_of_chorus" | "soc" => Directive::StartOfChorus(data.into()),
            "end_of_chorus" | "eoc" => Directive::EndOfChorus,
            "chorus" => Directive::Chorus(data.into()),
            "start_of_verse" => Directive::StartOfVerse(data.into()),
            "end_of_verse" => Directive::EndOfVerse,
            "start_of_tab" | "sot" => Directive::StartOfTab(data.into()),
            "end_of_tab" | "eot" => Directive::EndOfTab,
            "start_of_grid" => Directive::StartOfGrid(data.into()),
            "end_of_grid" => Directive::EndOfGrid,

            // Chord diagrams
            "define" => Directive::Define(data.into()),
            "chord" => Directive::Chord(data.into()),

            // Fonts, sizes and colours
            "textfont" => Directive::Textfont(data.into()),
            "textsize" => Directive::Textsize(data.into()),
            "textcolour" => Directive::Textcolour(data.into()),
            "chordfont" => Directive::Chordfont(data.into()),
            "chordsize" => Directive::Chordsize(data.into()),
            "chordcolour" => Directive::Chordcolour(data.into()),
            "tabfont" => Directive::Tabfont(data.into()),
            "tabsize" => Directive::Tabsize(data.into()),
            "tabcolour" => Directive::Tabcolour(data.into()),

            // Output related directives
            "new_page" | "np" => Directive::NewPage,
            "new_physical_page" | "npp" => Directive::NewPhysicalPage,
            "column_break" /*| "cb" */ => Directive::ColumnBreak,

            // Custom extensions
            _ => Directive::Custom(data.into())
        }
    }

    fn from_str<S: Into<String>>(input: S) -> Self {
        let input_string = input.into();
        let parts: Vec<&str> = input_string.splitn(2, ':').collect();
        if parts.len() > 1 {
            Directive::from_keyword(parts[0].trim(), parts[1].trim())
        } else {
            Directive::from_keyword(parts[0].trim(), "")
        }
    }

    pub fn comment<S: Into<String>>(value: S) -> Self {
        Directive::Comment(value.into())
    }

    pub fn title<S: Into<String>>(value: S) -> Self {
        Directive::Title(value.into())
    }

    pub fn start_of_chorus<S: Into<String>>(value: S) -> Self {
        Directive::StartOfChorus(value.into())
    }
}

impl From<&str> for Directive {
    fn from(input: &str) -> Self {
        Directive::from_str(input)
    }
}

impl From<String> for Directive {
    fn from(input: String) -> Self {
        Directive::from_str(input)
    }
}
