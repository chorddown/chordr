use crate::components::nbsp::Nbsp;
use yew::prelude::*;

pub type OnSetlist = bool;

#[derive(Properties, PartialEq, Clone)]
pub struct SetlistProps {
    pub is_on_setlist: OnSetlist,
    pub on_click: Callback<OnSetlist>,
}

pub struct Setlist {}

impl Component for Setlist {
    type Message = ();
    type Properties = SetlistProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_on_set_list = ctx.props().is_on_setlist;
        let add_to_list = ctx.props().on_click.reform(|_| true);
        let remove_from_list = ctx.props().on_click.reform(|_| false);

        let (title, on_click, icon, class) = if is_on_set_list {
            (
                "Remove song from setlist",
                remove_from_list,
                "im im-check-square-o",
                "discreet -active",
            )
        } else {
            (
                "Add song to setlist",
                add_to_list,
                "im im-square-o",
                "discreet",
            )
        };

        html! {
            <div class="setlist-tool">
                <div title={title}>
                    <button class={class} onclick={on_click}>
                        <i class={icon}></i>
                        <Nbsp/>
                        <span>{"Setlist"}</span>
                    </button>
                    <span class="sr-only">{title}</span>
                </div>
            </div>
        }
    }
}
