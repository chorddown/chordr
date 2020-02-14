use self::setlist_tool::Setlist;
use self::transpose_tool::TransposeTool;
use libchordr::prelude::Format as LibchordrFormat;
use libchordr::prelude::*;
use log::error;
use log::info;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::components::song_view::semitone_notation_tool::SemitoneNotationTool;

mod setlist_tool;
mod transpose_tool;
mod semitone_notation_tool;

#[derive(Properties, PartialEq)]
pub struct SongViewProps {
    #[props(required)]
    pub song: Song,
    #[props(required)]
    pub enable_setlists: bool,
    #[props(required)]
    pub is_on_setlist: bool,
    #[props(required)]
    pub on_setlist_add: Callback<Song>,
    #[props(required)]
    pub on_setlist_remove: Callback<Song>,

    pub transpose_semitone: Option<isize>,
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

    transpose_semitone: isize,
    formatting: Formatting,
}

impl Component for SongView {
    type Message = Msg;
    type Properties = SongViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let transpose_semitone = props.transpose_semitone.unwrap_or(0);
        let formatting = Formatting::with_format(LibchordrFormat::HTML);

        Self {
            link,
            props,
            transpose_semitone,
            formatting,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TransposeUp => {
                self.transpose_semitone += 1;
                info!("up to {:?}", self.transpose_semitone);
            }
            Msg::TransposeDown => {
                self.transpose_semitone -= 1;
                info!("down to {:?}", self.transpose_semitone);
            }
            Msg::TransposeSet(v) => {
                self.transpose_semitone = v;
                info!("set to {:?}", self.transpose_semitone);
            }
            Msg::SetlistChange(flag) => {
                let song = &self.props.song;
                info!("Set Song {} on set-list: {:?}", song.id(), flag);
                if flag {
                    self.props.on_setlist_add.emit(song.clone())
                } else {
                    self.props.on_setlist_remove.emit(song.clone())
                }
            }
            Msg::SemitoneNotationChange(s) => {
                self.formatting = Formatting { semitone_notation: s, ..self.formatting };
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let song_changed = self.props.song.id() != props.song.id();
        if song_changed || self.props != props {
            if song_changed {
                self.transpose_semitone = 0;
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
            self.transpose_semitone,
            self.props.is_on_setlist
        );

        let detail = self.convert_song_to_html_node();
        let transpose_up = self.link.callback(|_| Msg::TransposeUp);
        let transpose_down = self.link.callback(|_| Msg::TransposeDown);
        let transpose_set = self.link.callback(|v| Msg::TransposeSet(v));
        let setlist_set = self.link.callback(|b| Msg::SetlistChange(b));
        let semitone_notation_set = self.link.callback(|s| Msg::SemitoneNotationChange(s));

        let setlist_tool = if self.props.enable_setlists {
            html! {
                <Setlist
                    on_click=setlist_set
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
                        transpose_semitone=self.transpose_semitone
                        on_click_up=transpose_up
                        on_click_down=transpose_down
                        on_set=transpose_set
                    />
                    {setlist_tool}
                    <SemitoneNotationTool
                        semitone_notation=self.formatting.semitone_notation
                        on_change=semitone_notation_set
                    />
                </div>
            </div>
        }
    }
}

impl SongView {
    fn convert_song_to_html_string(&self) -> String {
        let props = &self.props;

        let converter_result = if self.transpose_semitone != 0 {
            transpose_and_convert_to_format(
                &props.song.src(),
                self.transpose_semitone,
                props.song.meta(),
                self.formatting,
            )
        } else {
            convert_to_format(&props.song.src(), props.song.meta(), self.formatting)
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
}
