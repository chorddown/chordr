use crate::components::nbsp::Nbsp;
use yew::prelude::*;
use stdweb::web::Date;

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
        let uri = format!("/?{}", Date::now());

        html! {
            <div class="reload-section">
                <div class="reload-button-container">
                    <a class="reload-button" href=uri title="Reload the song catalog">
                        <i class="im im-sync"></i>
                        <Nbsp/>
                        <span>{"Reload the catalog"}</span>
                    </a>
                </div>
                <div class="reload-section-version">
                    {"V"}{env!("CARGO_PKG_VERSION")}
                </div>
            </div>
        }
    }
}
