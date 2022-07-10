use libchordr::models::meta::SemitoneNotation;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SemitoneNotationToolProps {
    pub semitone_notation: SemitoneNotation,
    pub on_change: Callback<SemitoneNotation>,
}

pub struct SemitoneNotationTool {}

impl Component for SemitoneNotationTool {
    type Message = ();
    type Properties = SemitoneNotationToolProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut sharp_class = vec!["discreet"];
        let mut flat_class = vec!["discreet"];
        if ctx.props().semitone_notation == SemitoneNotation::Sharp {
            sharp_class.push("-active");
        } else {
            flat_class.push("-active");
        }

        let select_sharp = ctx.props().on_change.reform(|_| SemitoneNotation::Sharp);
        let select_flat = ctx.props().on_change.reform(|_| SemitoneNotation::Flat);

        html! {
            <div class="semitone-notation-tool">
                <div title="Select the semitone notation">
                    <button class={sharp_class} onclick={select_sharp}>{SemitoneNotation::Sharp}</button>
                    <button class={flat_class} onclick={select_flat}>{SemitoneNotation::Flat}</button>
                </div>
            </div>
        }
    }
}
