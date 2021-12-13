use libchordr::models::metadata::SemitoneNotation;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SemitoneNotationToolProps {
    pub semitone_notation: SemitoneNotation,
    pub on_change: Callback<SemitoneNotation>,
}

pub struct SemitoneNotationTool {
    /// State from the parent
    props: SemitoneNotationToolProps,
}

impl Component for SemitoneNotationTool {
    type Message = ();
    type Properties = SemitoneNotationToolProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let mut sharp_class = vec!["discreet"];
        let mut flat_class = vec!["discreet"];
        if self.props.semitone_notation == SemitoneNotation::Sharp {
            sharp_class.push("-active");
        } else {
            flat_class.push("-active");
        }

        let select_sharp = self.props.on_change.reform(|_| SemitoneNotation::Sharp);
        let select_flat = self.props.on_change.reform(|_| SemitoneNotation::Flat);

        html! {
            <div class="semitone-notation-tool">
                <div title="Select the semitone notation">
                    <button class=sharp_class onclick=select_sharp>{SemitoneNotation::Sharp}</button>
                    <button class=flat_class onclick=select_flat>{SemitoneNotation::Flat}</button>
                </div>
            </div>
        }
    }
}
