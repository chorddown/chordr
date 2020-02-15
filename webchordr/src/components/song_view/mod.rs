use self::setlist_tool::Setlist;
use self::transpose_tool::TransposeTool;
use libchordr::prelude::*;
use log::error;
use log::info;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::components::song_view::semitone_notation_tool::SemitoneNotationTool;
use libchordr::models::song_settings::SongSettings;

mod setlist_tool;
mod transpose_tool;
mod semitone_notation_tool;

#[derive(Properties, PartialEq)]
pub struct SongViewProps {
    #[props(required)]
    pub song: Song,
    #[props(required)]
    pub song_settings: SongSettings,
    #[props(required)]
    pub enable_setlists: bool,
    #[props(required)]
    pub is_on_setlist: bool,
    #[props(required)]
    pub on_setlist_add: Callback<SetlistEntry>,
    #[props(required)]
    pub on_setlist_remove: Callback<SongId>,
    #[props(required)]
    pub on_settings_change: Callback<(SongId, SongSettings)>,

    /// Display the Transpose tool with an input field
    pub show_input_field: Option<()>,
}

pub enum Msg {
    TransposeUp,
    TransposeDown,
    TransposeSet(isize),
    SetlistChange(bool),
    SemitoneNotationChange(SemitoneNotation),
}

#[allow(dead_code)]
pub struct SongView {
    /// State from the parent
    props: SongViewProps,
    /// Utility object
    link: ComponentLink<Self>,

    song_settings: SongSettings,
}

impl Component for SongView {
    type Message = Msg;
    type Properties = SongViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let song_settings = props.song_settings.clone();

        Self { link, props, song_settings }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TransposeUp => self.change_transpose(self.song_settings.transpose_semitone() + 1),
            Msg::TransposeDown => self.change_transpose(self.song_settings.transpose_semitone() - 1),
            Msg::TransposeSet(v) => self.change_transpose(v),
            Msg::SemitoneNotationChange(s) => self.change_semitone_notation(s),
            Msg::SetlistChange(flag) => {
                let song = &self.props.song;
                info!("Set Song {} on setlist: {:?}", song.id(), flag);
                if flag {
                    self.props.on_setlist_add.emit(
                        SetlistEntry::from_song_with_settings(
                            song,
                            self.song_settings.clone(),
                        )
                    )
                } else {
                    self.props.on_setlist_remove.emit(song.id())
                }
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let song_changed = self.props.song.id() != props.song.id();
        if song_changed || self.props != props {
            if song_changed {
                self.song_settings = props.song_settings.clone();
            }
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        info!(
            "View song {} (transpose {}, in setlist: {})",
            self.props.song.id(),
            self.song_settings.transpose_semitone(),
            self.props.is_on_setlist
        );

        let semitone_notation = self.song_settings.semitone_notation();
        let transpose_semitone = self.song_settings.transpose_semitone();

        let detail = self.convert_song_to_html_node();
        let transpose_up = self.link.callback(|_| Msg::TransposeUp);
        let transpose_down = self.link.callback(|_| Msg::TransposeDown);
        let transpose_set = self.link.callback(|v| Msg::TransposeSet(v));
        let setlist_change = self.link.callback(|b| Msg::SetlistChange(b));
        let semitone_notation_set = self.link.callback(|s| Msg::SemitoneNotationChange(s));

        let setlist_tool = if self.props.enable_setlists {
            html! {
                <Setlist
                    on_click=setlist_change
                    is_on_setlist=self.props.is_on_setlist
                />
            }
        } else {
            html! {}
        };
        html! {
            <div>
                {detail}
                <div class="song-tools">
                    <TransposeTool
                        show_input_field=false
                        transpose_semitone=transpose_semitone
                        on_click_up=transpose_up
                        on_click_down=transpose_down
                        on_set=transpose_set
                    />
                    {setlist_tool}
                    <SemitoneNotationTool
                        semitone_notation=semitone_notation
                        on_change=semitone_notation_set
                    />
                </div>
            </div>
        }
    }
}


impl SongView {
    fn send_change(&self) {
        self.props.on_settings_change.emit((
            self.props.song.id(),
            self.song_settings.clone()
        ))
    }

    fn convert_song_to_html_string(&self) -> String {
        let props = &self.props;
        let transpose_semitone = self.song_settings.transpose_semitone();
        let formatting = self.song_settings.formatting();

        let converter_result = if transpose_semitone != 0 {
            transpose_and_convert_to_format(
                &props.song.src(),
                transpose_semitone,
                props.song.meta(),
                formatting,
            )
        } else {
            convert_to_format(&props.song.src(), props.song.meta(), formatting)
        };

        match converter_result {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                String::new()
            }
        }
    }

    fn convert_song_to_html_node(&self) -> VNode {
        let html = self.convert_song_to_html_string();
        if let Ok(node) = Node::from_html(&html) {
            VNode::VRef(node)
        } else {
            html! {}
        }
    }

    fn change_transpose(&mut self, transpose_semitone: isize) {
        self.song_settings = SongSettings::new(transpose_semitone, self.song_settings.formatting());

        info!("Change transpose semitone to {}", transpose_semitone);
        self.send_change();
    }

    fn change_semitone_notation(&mut self, s: SemitoneNotation) -> () {
        let formatting = Formatting { semitone_notation: s, ..self.song_settings.formatting() };
        let transpose_semitone = self.song_settings.transpose_semitone();

        info!("Change formatting to {:?}", formatting);
        self.song_settings = SongSettings::new(
            transpose_semitone,
            formatting,
        );
        self.send_change();
    }
}
