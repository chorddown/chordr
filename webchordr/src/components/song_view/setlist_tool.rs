use yew::prelude::*;
use crate::components::nbsp::Nbsp;

pub type OnSetlist = bool;

#[derive(Properties, PartialEq)]
pub struct SetlistProps {
    #[props(required)]
    pub is_on_setlist: OnSetlist,
    #[props(required)]
    pub on_click: Callback<OnSetlist>,
}

pub struct Setlist {
    props: SetlistProps,
}

impl Component for Setlist {
    type Message = ();
    type Properties = SetlistProps;

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
        let is_on_set_list = self.props.is_on_setlist;
        let add_to_list = self.props.on_click.reform(|_| true);
        let remove_from_list = self.props.on_click.reform(|_| false);

        let (title, on_click, icon, class) = if is_on_set_list {
            (
                "Remove song from setlist",
                remove_from_list,
                "im im-check-square-o",
                "discreet -active"
            )
        } else {
            (
                "Add song to setlist",
                add_to_list,
                "im im-square-o",
                "discreet"
            )
        };

        html! {
            <div class="setlist-tool">
                <div title=title>
                    <button class=class onclick=on_click>
                        <i class=icon></i>
                        <Nbsp/>
                        <span>{"Setlist"}</span>
                    </button>
                    <span class="sr-only">{title}</span>
                </div>
            </div>
        }
    }
}
