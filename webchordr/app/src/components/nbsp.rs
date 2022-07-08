use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Nbsp;

impl Component for Nbsp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        html! { "\u{00a0}" }
    }
}
