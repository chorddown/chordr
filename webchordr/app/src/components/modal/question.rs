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
}

pub enum Msg {
    Choose1,
    Choose2,
}

impl Component for Question {
    type Message = Msg;
    type Properties = QuestionProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            visible: ctx.props().visible,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Choose1 => ctx.props().on_answer_1.emit(()),
            Msg::Choose2 => ctx.props().on_answer_2.emit(()),
        }
        self.visible = false;
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.visible = ctx.props().visible;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = &ctx.props();

        let on_answer_1 = ctx.link().callback(|_| Msg::Choose1);
        let on_answer_2 = ctx.link().callback(|_| Msg::Choose2);

        let answer_1_class = props.answer_1_class.or("model-answer answer-1");
        let answer_2_class = props.answer_2_class.or("model-answer answer-2");
        let class = props.class.or("");

        (if self.visible {
            html! {
                <ModalSkeleton class={class}>
                    <div class="modal-header modal-question">
                        <div class="modal-header-text">
                            {&props.question_text}
                        </div>
                    </div>
                    { ctx.props().children.iter().collect::<Html>() }
                    <div class="modal-buttons button-group">
                        <button class={answer_1_class} onclick={on_answer_1}>
                            {&props.answer_1_text}
                        </button>
                        <button class={answer_2_class} onclick={on_answer_2}>
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
