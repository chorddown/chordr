use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

use libchordr::prelude::Format as LibchordrFormat;
use stdweb::web::Node;
use self::transpose_tool::TransposeTool;

use log::error;
use log::info;

mod transpose_tool;

#[derive(Properties, PartialEq)]
pub struct SongViewProps {
    #[props(required)]
    pub song: Song,

    pub transpose_semitone: Option<isize>,
    pub show_input_field: Option<()>,
}

pub enum Msg {
    TransposeUp,
    TransposeDown,
    TransposeSet(isize),
}

#[allow(dead_code)]
pub struct SongView {
    /// State from the parent
    props: SongViewProps,
    /// Utility object
    link: ComponentLink<Self>,

    transpose_semitone: isize,
}

impl Component for SongView {
    type Message = Msg;
    type Properties = SongViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let transpose_semitone = props.transpose_semitone.unwrap_or(0);

        Self { link, props, transpose_semitone }
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
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.song.id() != props.song.id() {
            self.props = props;
            self.transpose_semitone = 0;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        info!("View song {} (transpose {})", self.props.song.id(), self.transpose_semitone);

        let detail = self.convert_song_to_html_node();
        let transpose_up = self.link.callback(|_| Msg::TransposeUp);
        let transpose_down = self.link.callback(|_| Msg::TransposeDown);
        let transpose_set = self.link.callback(|v| Msg::TransposeSet(v));

        html! {
            <div>
                {detail}
                <TransposeTool
                    show_input_field=false
                    transpose_semitone=self.transpose_semitone
                    on_click_up=transpose_up
                    on_click_down=transpose_down
                    on_set=transpose_set
                />
            </div>
        }
    }
}

impl SongView {
    fn convert_song_to_html_string(&self) -> String {
        let props = &self.props;

        let converter_result = if self.transpose_semitone != 0 {
            transpose_and_convert_to_format(&props.song.src(), self.transpose_semitone, props.song.meta(), LibchordrFormat::HTML)
        } else {
            convert_to_format(&props.song.src(), props.song.meta(), LibchordrFormat::HTML)
        };

        match converter_result {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                String::new()
            }
        }
    }
}

impl SongView {
    fn convert_song_to_html_node(&self) -> VNode {
        let html = self.convert_song_to_html_string();
        if let Ok(node) = Node::from_html(&html) {
            VNode::VRef(node)
        } else {
            html! {}
        }
    }
}
