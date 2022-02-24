use yew::prelude::*;

pub struct HomeTool {}

impl Component for HomeTool {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="home-tool">
                <a role="button" class="home discreet" href="/#" title="Go to home screen">
                    <i class="im im-home"></i>
                </a>
            </div>
        }
    }
}
