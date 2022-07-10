use yew::prelude::*;

#[function_component(StartScreen)]
pub(crate) fn start_screen() -> Html {
    (html! {
        <div class="start-screen">
            <div class="logo">
                <img src="/assets/images/logo-512-light.png" alt="chordr" />
            </div>
        </div>
    }) as Html
}
