use webchordr_common::prelude::*;
use yew::prelude::*;

#[function_component(HomeTool)]
pub fn home_tool() -> Html {
    (html! {
        <div class="home-tool">
            <Link role="button" class="home discreet" to={AppRoute::Index} title="Go to home screen">
                <i class="im im-home"></i>
            </Link>
        </div>
    }) as Html
}
