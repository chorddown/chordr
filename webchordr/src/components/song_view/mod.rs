use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

use libchordr::prelude::Format as LibchordrFormat;
use stdweb::web::Node;

use log::error;
use log::info;

#[derive(Properties, PartialEq)]
pub struct SongViewProps {
    #[props(required)]
    pub song: Song,

    pub transpose_semitone: Option<isize>,
}

pub enum Msg {
    Click(ChangeData)
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
            Msg::Click(ChangeData::Value(v)) => {
                info!("{:?}", v);

                self.transpose_semitone = match v.parse::<isize>() {
                    Ok(v) => v,
                    Err(_) => {
                        error!("Invalid change data {:?}", v);
                        0
                    }
                };
                true // Indicate that the Component should re-render
            }
            Msg::Click(change_data) => {
                error!("Invalid change data {:?}", change_data);
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.song.id() != props.song.id() {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        info!("View song {}", self.props.song.id());

        let detail = self.convert_song_to_html_node();
        let transpose_tool = self.render_transpose_tool();

        html! {
            <div>
                {detail}
                {transpose_tool}
            </div>
        }
    }
}


impl SongView {
    fn render_transpose_tool(&self) -> Html {
        let onchange = self.link.callback(|e: ChangeData| Msg::Click(e));

        html! {
            <div class="transpose-tool">
                <label title="Transpose song">
                    <span class="icon">{"♬"}</span>
                    <input type="number" min="-11" max="11" onchange=onchange value=self.transpose_semitone/>
                    <span class="sr-only">{"Transpose song"}</span>
                </label>
            </div>
        }
    }
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
