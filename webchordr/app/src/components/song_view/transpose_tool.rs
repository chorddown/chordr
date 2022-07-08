use log::error;
use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TransposeToolProps {
    pub transpose_semitone: isize,
    pub show_input_field: bool,

    pub on_click_up: Callback<()>,
    pub on_click_down: Callback<()>,
    pub on_set: Callback<isize>,
}

pub enum Msg {
    InputChange(String),
}

pub struct TransposeTool {}

impl Component for TransposeTool {
    type Message = Msg;
    type Properties = TransposeToolProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChange(v) => {
                info!("{:?}", v);

                match v.parse::<isize>() {
                    Ok(v) => {
                        ctx.props().on_set.emit(v);
                    }
                    Err(_) => {
                        error!("Invalid change data {:?}", v);
                    }
                };
            } // Msg::InputChange(change_data) => error!("Invalid change data {:?}", change_data),
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let transpose_semitone = ctx.props().transpose_semitone;
        let transpose_up = ctx.props().on_click_up.reform(|_| ());
        let transpose_down = ctx.props().on_click_down.reform(|_| ());
        let show_input_field = ctx.props().show_input_field;

        let number_output = if show_input_field {
            let onchange = ctx.link().batch_callback(|e: Event| {
                let input = e.target_dyn_into::<HtmlInputElement>();

                input.map(|input| Msg::InputChange(input.value()))
            });

            // let onchange = ctx.link().callback(Msg::InputChange);
            html! {<input type="number" min="-11" max="11" onchange={onchange} value={transpose_semitone.to_string()}/>}
        } else {
            html! {<span class="value">{transpose_semitone}</span>}
        };

        let disable_down = transpose_semitone < -11;
        let disable_up = transpose_semitone > 11;

        let inner = html! {
            <>
                <span class="icon">{"♬"}</span>
                <button class="discreet" disabled={disable_down} onclick={transpose_down}><i class="im im-angle-left"></i></button>
                {number_output}
                <button class="discreet" disabled={disable_up} onclick={transpose_up}><i class="im im-angle-right"></i></button>
                <span class="sr-only">{"Transpose song"}</span>
            </>
        };

        (if show_input_field {
            html! {
                <div class="transpose-tool">
                    <label title="Transpose song">
                        {inner}
                    </label>
                </div>
            }
        } else {
            html! {
                <div class="transpose-tool">
                    <div title="Transpose song">
                        {inner}
                    </div>
                </div>
            }
        }) as Html
    }
}
