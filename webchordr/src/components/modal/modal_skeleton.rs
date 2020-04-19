use crate::helpers::Class;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub children: Children,
    pub class: Class,
}

pub struct ModalSkeleton {
    props: ModalProps,
}

impl Component for ModalSkeleton {
    type Message = ();
    type Properties = ModalProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let class = self.props.class.add("modal");

        html! {
            <div class="modal-outer">
                <div class=class>
                    {{ self.props.children.iter().collect::<Html>() }}
                </div>
            </div>
        }
    }
}
