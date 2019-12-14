#[derive(Clone, Debug)]
pub struct Meta {
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
}

impl Default for Meta {
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
        }
    }
}
