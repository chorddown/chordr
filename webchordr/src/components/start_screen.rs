use yew::virtual_dom::VNode;
use yew::{html, ShouldRender};
use yew::{Component, ComponentLink};

pub(crate) struct StartScreen {}

impl Component for StartScreen {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="start-screen">
                <div class="logo">
                    <img src="assets/images/logo-512-light.png" alt="chordr" />
                </div>
            </div>
        }
    }
}
