use crate::components::modal::modal_skeleton::ModalSkeleton;
use crate::helpers::Class;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct QuestionProps {
    pub question_text: String,
    pub answer_1_text: String,
    pub answer_2_text: String,

    #[prop_or_default]
    pub class: Class,
    #[prop_or_default]
    pub answer_1_class: Class,
    #[prop_or_default]
    pub answer_2_class: Class,

    pub on_answer_1: Callback<()>,
    pub on_answer_2: Callback<()>,

    pub visible: bool,

    #[prop_or_default]
    pub children: Children,
}

pub struct Question {
    visible: bool,
    props: QuestionProps,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Choose1,
    Choose2,
}

impl Component for Question {
    type Message = Msg;
    type Properties = QuestionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            visible: props.visible,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Choose1 => self.props.on_answer_1.emit(()),
            Msg::Choose2 => self.props.on_answer_2.emit(()),
        }
        self.visible = false;
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
        let props = &self.props;

        let on_answer_1 = self.link.callback(|_| Msg::Choose1);
        let on_answer_2 = self.link.callback(|_| Msg::Choose2);

        let answer_1_class = props.answer_1_class.or("model-answer answer-1");
        let answer_2_class = props.answer_2_class.or("model-answer answer-2");
        let class = props.class.or("");

        (if self.visible {
            html! {
                <ModalSkeleton class=class>
                    <div class="modal-header modal-question">
                        <div class="modal-header-text">
                            {&props.question_text}
                        </div>
                    </div>
                    {{ self.props.children.iter().collect::<Html>() }}
                    <div class="modal-buttons button-group">
                        <button class=answer_1_class onclick=on_answer_1>
                            {&props.answer_1_text}
                        </button>
                        <button class=answer_2_class onclick=on_answer_2>
                            {&props.answer_2_text}
                        </button>
                    </div>
                </ModalSkeleton>
            }
        } else {
            html! {}
        }) as Html
    }
}
