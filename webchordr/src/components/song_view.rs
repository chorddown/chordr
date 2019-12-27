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
}

pub enum Msg {}

#[allow(dead_code)]
pub struct SongView {
    /// State from the parent
    props: SongViewProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for SongView {
    type Message = Msg;
    type Properties = SongViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.song.id() != props.song.id() {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> VNode {
        info!("View song {}", self.props.song.id());
        let html = match convert_to_format(&self.props.song.src(), LibchordrFormat::HTML) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return html! {};
            }
        };

        if let Ok(node) = Node::from_html(&html) {
            VNode::VRef(node)
        } else {
            html! {}
        }
    }
}
