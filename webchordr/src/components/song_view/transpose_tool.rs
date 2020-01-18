use log::error;
use log::info;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TransposeToolProps {
    #[props(required)]
    pub transpose_semitone: isize,
    #[props(required)]
    pub show_input_field: bool,
    #[props(required)]
    pub on_click_up: Callback<()>,
    #[props(required)]
    pub on_click_down: Callback<()>,
    #[props(required)]
    pub on_set: Callback<isize>,
}

pub enum Msg {
    InputChange(ChangeData),
}

#[allow(dead_code)]
pub struct TransposeTool {
    /// State from the parent
    props: TransposeToolProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for TransposeTool {
    type Message = Msg;
    type Properties = TransposeToolProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputChange(ChangeData::Value(v)) => {
                info!("{:?}", v);

                match v.parse::<isize>() {
                    Ok(v) => {
                        self.props.on_set.emit(v);
                    }
                    Err(_) => {
                        error!("Invalid change data {:?}", v);
                    }
                };
            }
            Msg::InputChange(change_data) => error!("Invalid change data {:?}", change_data),
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let transpose_semitone = self.props.transpose_semitone;
        let transpose_up = self.props.on_click_up.reform(|_| ());
        let transpose_down = self.props.on_click_down.reform(|_| ());
        let show_input_field = self.props.show_input_field;

        let number_output = if show_input_field {
            let onchange = self.link.callback(|e: ChangeData| Msg::InputChange(e));
            html! {<input type="number" min="-11" max="11" onchange=onchange value=transpose_semitone/>}
        } else {
            html! {<span class="value">{transpose_semitone}</span>}
        };

        let disable_down = transpose_semitone < -11;
        let disable_up = transpose_semitone > 11;

        let inner = html! {
            <>
                <span class="icon">{"♬"}</span>
                <button class="discreet" disabled=disable_down onclick=transpose_down><i class="im im-angle-left"></i></button>
                {number_output}
                <button class="discreet" disabled=disable_up onclick=transpose_up><i class="im im-angle-right"></i></button>
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
