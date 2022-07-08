use yew::{html, Component, Context, Html};

pub(crate) struct StartScreen {}

impl Component for StartScreen {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        (html! {
            <div class="start-screen">
                <div class="logo">
                    <img src="/assets/images/logo-512-light.png" alt="chordr" />
                </div>
            </div>
        }) as Html
    }
}
