use crate::helpers::Class;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub children: Children,
    pub class: Class,
}

pub struct ModalSkeleton {}

impl Component for ModalSkeleton {
    type Message = ();
    type Properties = ModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class.add("modal");

        html! {
            <div class="modal-outer">
                <div class={class}>
                    {{ ctx.props().children.iter().collect::<Html>() }}
                </div>
            </div>
        }
    }
}
