use crate::components::modal::modal_skeleton::ModalSkeleton;
use crate::helpers::Class;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub children: Children,
    pub visible: bool,

    #[prop_or_default]
    pub header_text: String,
    #[prop_or_default]
    pub class: Class,
    #[prop_or_default]
    pub on_close: Callback<()>,
}

pub struct Modal {
    visible: bool,
}

pub enum Msg {
    Close,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = ModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            visible: ctx.props().visible,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                self.visible = false;
                ctx.props().on_close.emit(())
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.visible = ctx.props().visible;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        (if self.visible {
            let handle_close_click = ctx.link().callback(|_| Msg::Close);
            let class = ctx.props().class.clone();

            html! {
                <ModalSkeleton {class}>
                    <div class="modal-header">
                        <div class="modal-header-text">
                            {&ctx.props().header_text}
                        </div>
                        <button class="modal-close" onclick={handle_close_click}>
                            <i class="im im-x-mark"></i>
                        </button>
                    </div>
                    <div class="modal-body">
                        {{ ctx.props().children.iter().collect::<Html>() }}
                    </div>
                </ModalSkeleton>
            }
        } else {
            html! {}
        }) as Html
    }
}
