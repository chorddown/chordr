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
    pub show_input_field: Option<()>,
}

pub enum Msg {
    InputChange(ChangeData),
    TransposeUp,
    TransposeDown,
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
            Msg::InputChange(ChangeData::Value(v)) => {
                info!("{:?}", v);

                self.transpose_semitone = match v.parse::<isize>() {
                    Ok(v) => v,
                    Err(_) => {
                        error!("Invalid change data {:?}", v);
                        0
                    }
                };
            }
            Msg::InputChange(change_data) => error!("Invalid change data {:?}", change_data),
            Msg::TransposeUp => {
                self.transpose_semitone += 1;
                info!("up to {:?}", self.transpose_semitone);
            }
            Msg::TransposeDown => {
                self.transpose_semitone -= 1;
                info!("down to {:?}", self.transpose_semitone);
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
        let transpose_up = self.link.callback(|_| Msg::TransposeUp);
        let transpose_down = self.link.callback(|_| Msg::TransposeDown);
        let show_input_field = self.props.show_input_field.is_some();

        let number_output = if show_input_field {
            let onchange = self.link.callback(|e: ChangeData| Msg::InputChange(e));
            html! {<input type="number" min="-11" max="11" onchange=onchange value=self.transpose_semitone/>}
        } else {
            html! {<span class="value">{self.transpose_semitone}</span>}
        };

        let inner = html! {
            <>
                <span class="icon">{"♬"}</span>
                <button class="discreet" onclick=transpose_down><i class="im im-angle-left"></i></button>
                {number_output}
                <button class="discreet" onclick=transpose_up><i class="im im-angle-right"></i></button>
                <span class="sr-only">{"Transpose song"}</span>
            </>
        };

        (if show_input_field {
            html! {
                <div class="transpose-tool">
                    <label title="Transpose song">
                        {inner}
                    </label>
                </div>
            }
        } else {
            html! {
                <div class="transpose-tool">
                    <div title="Transpose song">
                        {inner}
                    </div>
                </div>
            }
        }) as Html
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
