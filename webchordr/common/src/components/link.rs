use std::marker::PhantomData;

use crate::route::{AppRoute, LinkTo};
use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<Q = ()>
where
    Q: Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub class: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: AppRoute,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub role: Option<String>,
    #[prop_or_default]
    pub data_key: Option<String>,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

/// A wrapper around `<a>` tag to be used with [`Router`](crate::Router)
pub struct Link<Q = ()>
where
    Q: Clone + PartialEq + Serialize + 'static,
{
    _query: PhantomData<Q>,
}

pub enum Msg {
    OnClick,
}

impl<Q> Component for Link<Q>
where
    Q: Clone + PartialEq + Serialize + 'static,
{
    type Message = Msg;
    type Properties = LinkProps<Q>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _query: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                let LinkProps { to, query, .. } = ctx.props();
                let history = ctx.link().history().expect_throw("failed to read history");
                match query {
                    None => {
                        history.push(to.clone());
                    }
                    Some(data) => {
                        history
                            .push_with_query(to.clone(), data.clone())
                            .expect_throw("failed push history with query");
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let LinkProps {
            class,
            title,
            to,
            data_key,
            children,
            disabled,
            role,
            ..
        } = ctx.props().clone();
        let onclick = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::OnClick
        });

        let href: AttrValue = LinkTo::from(to).to_string().into();

        html! {
            <a {class}
                {title}
                {href}
                {role}
                {onclick}
                {disabled}
                data-key={data_key}
            >
                { children }
            </a>
        }
    }
}
