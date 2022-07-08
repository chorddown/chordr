use webchordr_common::components::link::Link;
use webchordr_common::route::AppRoute;
use yew::prelude::*;

pub struct HomeTool {}

impl Component for HomeTool {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="home-tool">
                <Link role="button" class="home discreet" to={AppRoute::Index} title="Go to home screen">
                    <i class="im im-home"></i>
                </Link>
            </div>
        }
    }
}
