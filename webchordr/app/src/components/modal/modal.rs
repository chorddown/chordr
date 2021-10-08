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
    props: ModalProps,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Close,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = ModalProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            visible: props.visible,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.visible = false;
                self.props.on_close.emit(())
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.visible = props.visible;
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        (if self.visible {
            let handle_close_click = self.link.callback(|_| Msg::Close);
            let class = self.props.class.clone();

            html! {
                <ModalSkeleton class=class>
                    <div class="modal-header">
                        <div class="modal-header-text">
                            {&self.props.header_text}
                        </div>
                        <button class="modal-close" onclick=handle_close_click>
                            <i class="im im-x-mark"></i>
                        </button>
                    </div>
                    <div class="modal-body">
                        {{ self.props.children.iter().collect::<Html>() }}
                    </div>
                </ModalSkeleton>
            }
        } else {
            html! {}
        }) as Html
    }
}
