use std::convert::From;


#[derive(Debug, PartialEq)]
pub enum Meta {
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

impl Meta {
    pub fn from_keyword<S: Into<String>>(keyword: &str, data: S) -> Self {
        match keyword {
            // Preamble directives
            "new_song" | "ns" => Meta::NewSong,

            // Meta data
            "title" | "t" => Meta::Title(data.into()),
            "subtitle" | "st" => Meta::Subtitle(data.into()),
            "artist" => Meta::Artist(data.into()),
            "composer" => Meta::Composer(data.into()),
            "lyricist" => Meta::Lyricist(data.into()),
            "copyright" => Meta::Copyright(data.into()),
            "album" => Meta::Album(data.into()),
            "year" => Meta::Year(data.into()),
            "key" => Meta::Key(data.into()),
            "time" => Meta::Time(data.into()),
            "tempo" => Meta::Tempo(data.into()),
            "duration" => Meta::Duration(data.into()),
            "capo" => Meta::Capo(data.into()),
            "meta" => Meta::Meta(data.into()),

            // Formatting
            "comment" | "c" => Meta::Comment(data.into()),
            "comment_italic" | "ci" => Meta::CommentItalic(data.into()),
            "comment_box" | "cb" => Meta::CommentBox(data.into()),
            "image" => Meta::Image(data.into()),


            // Environment directives
            "start_of_chorus" | "soc" => Meta::StartOfChorus(data.into()),
            "end_of_chorus" | "eoc" => Meta::EndOfChorus,
            "chorus" => Meta::Chorus(data.into()),
            "start_of_verse" => Meta::StartOfVerse(data.into()),
            "end_of_verse" => Meta::EndOfVerse,
            "start_of_tab" | "sot" => Meta::StartOfTab(data.into()),
            "end_of_tab" | "eot" => Meta::EndOfTab,
            "start_of_grid" => Meta::StartOfGrid(data.into()),
            "end_of_grid" => Meta::EndOfGrid,

            // Chord diagrams
            "define" => Meta::Define(data.into()),
            "chord" => Meta::Chord(data.into()),

            // Fonts, sizes and colours
            "textfont" => Meta::Textfont(data.into()),
            "textsize" => Meta::Textsize(data.into()),
            "textcolour" => Meta::Textcolour(data.into()),
            "chordfont" => Meta::Chordfont(data.into()),
            "chordsize" => Meta::Chordsize(data.into()),
            "chordcolour" => Meta::Chordcolour(data.into()),
            "tabfont" => Meta::Tabfont(data.into()),
            "tabsize" => Meta::Tabsize(data.into()),
            "tabcolour" => Meta::Tabcolour(data.into()),

            // Output related directives
            "new_page" | "np" => Meta::NewPage,
            "new_physical_page" | "npp" => Meta::NewPhysicalPage,
            "column_break" /*| "cb" */ => Meta::ColumnBreak,

            // Custom extensions
            _ => Meta::Custom(data.into())
        }
    }

    fn from_str<S: Into<String>>(input: S) -> Self {
        let input_string = input.into();
        let parts: Vec<&str> = input_string.splitn(2, ':').collect();
        if parts.len() > 1 {
            Meta::from_keyword(parts[0].trim(), parts[1].trim())
        } else {
            Meta::from_keyword(parts[0].trim(), "")
        }
    }

    pub fn comment<S: Into<String>>(value: S) -> Self {
        Meta::Comment(value.into())
    }

    pub fn title<S: Into<String>>(value: S) -> Self {
        Meta::Title(value.into())
    }

    pub fn start_of_chorus<S: Into<String>>(value: S) -> Self {
        Meta::StartOfChorus(value.into())
    }
}

impl From<&str> for Meta {
    fn from(input: &str) -> Self {
        Meta::from_str(input)
    }
}

impl From<String> for Meta {
    fn from(input: String) -> Self {
        Meta::from_str(input)
    }
}
