use printpdf::{IndirectFontRef, Mm};

#[derive(Clone)]
pub struct Style {
    pub(super) font: IndirectFontRef,
    pub(super) line_height: Mm,
    pub(super) font_size: i64,
}

impl Style {
    pub fn new(font: IndirectFontRef, line_height: Mm, font_size: i64) -> Self {
        Self {
            font,
            line_height,
            font_size,
        }
    }
}

#[derive(Clone)]
pub struct PageSize {
    width: Mm,
    height: Mm,
}

impl PageSize {
    pub fn new(width: Mm, height: Mm) -> Self {
        Self { width, height }
    }
}

pub struct Styles {
    chorus: Option<Style>,
    verse: Option<Style>,
    chord: Option<Style>,
    headline: Option<Style>,
    page: PageSize,
}

impl Styles {
    pub fn new(
        chorus: Option<Style>,
        verse: Option<Style>,
        chord: Option<Style>,
        headline: Option<Style>,
        page: PageSize,
    ) -> Self {
        Self {
            chorus,
            verse,
            chord,
            headline,
            page,
        }
    }
    pub fn chorus(&self) -> Option<&Style> {
        self.chorus.as_ref()
    }
    pub fn verse(&self) -> Option<&Style> {
        self.verse.as_ref()
    }
    pub fn chord(&self) -> Option<&Style> {
        self.chord.as_ref()
    }
    pub fn headline(&self) -> Option<&Style> {
        self.headline.as_ref()
    }
    pub fn page(&self) -> PageSize {
        self.page.clone()
    }
}
