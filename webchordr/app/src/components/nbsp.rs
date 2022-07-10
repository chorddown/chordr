use yew::prelude::*;

#[function_component(Nbsp)]
pub fn nbsp() -> Html {
    (html! { "\u{00a0}" }) as Html
}
