use yew::prelude::*;
use yew::Callback;

use libchordr::prelude::Setlist;
use webchordr_common::helpers::Class;

#[derive(Properties, Clone)]
pub struct ItemProps {
    pub setlist: Setlist,
    pub highlight: bool,
    pub on_load_click: Callback<Setlist>,
    pub on_delete_click: Callback<Setlist>,
}

pub struct Item {
    props: ItemProps,
}

impl Component for Item {
    type Message = ();
    type Properties = ItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.setlist != props.setlist || self.props.highlight != props.highlight {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let setlist = self.props.setlist.clone();
        let name = setlist.name();
        let key = setlist.id();
        let base_class = Class::new("setlist-list-item");
        let class = if self.props.highlight {
            base_class.add("-highlight")
        } else {
            base_class
        };

        let text = if !name.is_empty() {
            name.to_string()
        } else {
            format!("key {}", key)
        };

        let setlist_for_loading = setlist.clone();
        let on_load_click_prop = self
            .props
            .on_load_click
            .reform(move |_| setlist_for_loading.clone());
        let on_delete_click_prop = self.props.on_delete_click.reform(move |_| setlist.clone());

        html! {
            <div class={class} key=key>
                <div class="button-group -compact">
                    <button class="setlist-list-item-load" data-v={key.to_string()} onclick=on_load_click_prop>{text}</button>
                    <button class="setlist-list-item-delete" data-v={key.to_string()} onclick=on_delete_click_prop title="Remove">
                        <i class="im im-trash-can"></i>
                        <span class="sr-only">{"Remove"}</span>
                    </button>
                </div>
            </div>
        }
    }
}
