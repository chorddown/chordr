use yew::{html, Html, ShouldRender};
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        (html! {
            <div class="start-screen">
                <div class="logo">
                    <img src="/assets/images/logo-512-light.png" alt="chordr" />
                </div>
            </div>
        }) as Html
    }
}
