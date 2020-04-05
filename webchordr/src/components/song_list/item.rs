use crate::helpers::Class;
use libchordr::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

#[derive(Properties, PartialEq, Clone)]
pub struct SongListItemProps<S: SongData + Clone> {
    pub song: S,
    pub key: String,

    #[prop_or_default]
    pub class: Class,
}

#[allow(dead_code)]
pub struct Item<S: SongData + PartialEq + 'static + Clone> {
    /// State from the parent
    props: SongListItemProps<S>,
    /// Utility object
    link: ComponentLink<Self>,
}

impl<S: SongData + PartialEq + 'static + Clone> Component for Item<S> {
    type Message = ();
    type Properties = SongListItemProps<S>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // info!("{:?} vs {:?}", self.props, props);
        // let new_name = props.song.title();
        // let old_name = self.props.song.title();
        // js!(console.log("%c" + @{old_name} + " -> " + @{new_name}, "color:Orange"));
        // self.props = props;
        // return true;
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> VNode {
        let title = &self.props.song.title();
        let key = &self.props.key;
        let href = format!("#/song/{}", self.props.song.id());
        let class = self.props.class.or("song-item");

        html! { <a role="button" class=class data-key=key href=href>{title}</a> }
    }
}
