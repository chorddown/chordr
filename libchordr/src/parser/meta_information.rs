use crate::models::meta::b_notation::BNotation;
use crate::models::meta::MetaTrait;
use crate::tokenizer::Meta;

/// Meta Information for a parsed song
#[derive(Clone, Debug)]
pub struct MetaInformation {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub artist: Option<String>,
    pub composer: Option<String>,
    pub lyricist: Option<String>,
    pub copyright: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub key: Option<String>,
    pub time: Option<String>,
    pub tempo: Option<String>,
    pub duration: Option<String>,
    pub capo: Option<String>,
    pub original_title: Option<String>,
    pub alternative_title: Option<String>,
    pub ccli_song_id: Option<String>,
    pub b_notation: BNotation,
}

impl MetaInformation {
    /// Copy the Meta content into the appropriate field
    pub(crate) fn assign_from_token(&mut self, t: &Meta) {
        match t {
            Meta::Subtitle(content) => self.subtitle = Some(content.clone()),
            Meta::Artist(content) => self.artist = Some(content.clone()),
            Meta::Composer(content) => self.composer = Some(content.clone()),
            Meta::Lyricist(content) => self.lyricist = Some(content.clone()),
            Meta::Copyright(content) => self.copyright = Some(content.clone()),
            Meta::Album(content) => self.album = Some(content.clone()),
            Meta::Year(content) => self.year = Some(content.clone()),
            Meta::Key(content) => self.key = Some(content.clone()),
            Meta::Time(content) => self.time = Some(content.clone()),
            Meta::Tempo(content) => self.tempo = Some(content.clone()),
            Meta::Duration(content) => self.duration = Some(content.clone()),
            Meta::Capo(content) => self.capo = Some(content.clone()),
            Meta::OriginalTitle(content) => self.original_title = Some(content.clone()),
            Meta::AlternativeTitle(content) => self.alternative_title = Some(content.clone()),
            Meta::CCLISongId(content) => self.ccli_song_id = Some(content.clone()),
            Meta::BNotation(notation) => self.b_notation = *notation,
        }
    }
}

impl MetaTrait for MetaInformation {
    fn title(&self) -> Option<String> {
        self.title.as_ref().cloned()
    }

    fn subtitle(&self) -> Option<String> {
        self.subtitle.as_ref().cloned()
    }

    fn artist(&self) -> Option<String> {
        self.artist.as_ref().cloned()
    }

    fn composer(&self) -> Option<String> {
        self.composer.as_ref().cloned()
    }

    fn lyricist(&self) -> Option<String> {
        self.lyricist.as_ref().cloned()
    }

    fn copyright(&self) -> Option<String> {
        self.copyright.as_ref().cloned()
    }

    fn album(&self) -> Option<String> {
        self.album.as_ref().cloned()
    }

    fn year(&self) -> Option<String> {
        self.year.as_ref().cloned()
    }

    fn key(&self) -> Option<String> {
        self.key.as_ref().cloned()
    }

    fn time(&self) -> Option<String> {
        self.time.as_ref().cloned()
    }

    fn tempo(&self) -> Option<String> {
        self.tempo.as_ref().cloned()
    }

    fn duration(&self) -> Option<String> {
        self.duration.as_ref().cloned()
    }

    fn capo(&self) -> Option<String> {
        self.capo.as_ref().cloned()
    }

    fn original_title(&self) -> Option<String> {
        self.original_title.as_ref().cloned()
    }

    fn alternative_title(&self) -> Option<String> {
        self.alternative_title.as_ref().cloned()
    }

    fn ccli_song_id(&self) -> Option<String> {
        self.ccli_song_id.as_ref().cloned()
    }

    fn b_notation(&self) -> BNotation {
        self.b_notation
    }
}

impl Default for MetaInformation {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            artist: None,
            composer: None,
            lyricist: None,
            copyright: None,
            album: None,
            year: None,
            key: None,
            time: None,
            tempo: None,
            duration: None,
            capo: None,
            original_title: None,
            alternative_title: None,
            ccli_song_id: None,
            b_notation: Default::default(),
        }
    }
}
