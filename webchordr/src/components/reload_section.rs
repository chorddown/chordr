use yew::prelude::*;
use crate::components::nbsp::Nbsp;

pub struct ReloadSection {}

impl Component for ReloadSection {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="reload-button-container">
                <a class="reload-button" href="/" title="Reload the song catalog">
                    <i class="im im-sync"></i>
                    <Nbsp/>
                    <span>{"Reload the catalog"}</span>
                </a>
            </div>
        }
    }
}
