use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Nbsp;

impl Component for Nbsp {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> VNode {
        html! { "\u{00a0}" }
    }
}
