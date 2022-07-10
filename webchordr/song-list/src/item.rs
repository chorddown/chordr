use libchordr::prelude::*;
use std::marker::PhantomData;
use webchordr_common::components::link::Link;
use webchordr_common::helpers::Class;
use webchordr_common::route::AppRoute;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Component;

#[derive(Properties, PartialEq, Clone)]
pub struct SongListItemProps<S: SongData + Clone + PartialEq> {
    pub song: S,
    pub data_key: String,

    #[prop_or_default]
    pub sortable: bool,

    #[prop_or_default]
    pub draggable: bool,

    #[prop_or_default]
    pub highlight: bool,

    #[prop_or_default]
    pub class: Class,
}

#[allow(dead_code)]
pub struct Item<S: SongData + PartialEq + 'static + Clone> {
    _ph: PhantomData<S>,
}

impl<S: SongData + PartialEq + 'static + Clone> Item<S> {
    fn get_class(&self, ctx: &Context<Self>) -> Class {
        let base_class = ctx.props().class.or("song-item");
        let class = if ctx.props().highlight {
            base_class.add("-highlight")
        } else {
            base_class
        };

        let class = if ctx.props().sortable {
            class.add("-sortable")
        } else {
            class
        };

        if ctx.props().draggable {
            class.add("-draggable")
        } else {
            class
        }
    }
}

impl<S: SongData + PartialEq + 'static + Clone> Component for Item<S> {
    type Message = ();
    type Properties = SongListItemProps<S>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _ph: PhantomData::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let title = &ctx.props().song.title();
        let key = &ctx.props().data_key;
        let id = ctx.props().song.id().to_string();
        let class = self.get_class(ctx);
        let draggable = if ctx.props().draggable {
            "true"
        } else {
            "false"
        };

        let to = AppRoute::Song {
            id: ctx.props().song.id(),
        };
        let link = html! { <Link role="button" class="discreet" data_key={key.clone()} to={to}>{title}</Link> };
        // let link = html! { <a role="button" class="discreet" data-key={key.clone()} href={href}>{title}</a> };

        (if ctx.props().sortable {
            html! { <div class={class} data-song-id={id} draggable={draggable}>{link}<span class="sortable-handle">{"::"}</span></div> }
        } else {
            html! { <div class={class} data-song-id={id} draggable={draggable}>{link}</div> }
        }) as Html
    }
}
