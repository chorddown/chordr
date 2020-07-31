use crate::helpers::window;
use log::error;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailViewProps {
    pub children: Children,
}

pub struct DetailView {
    props: DetailViewProps,
    link: ComponentLink<Self>,
}

impl Component for DetailView {
    type Message = ();
    type Properties = DetailViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        let close = self.link.callback(|_| {
            if let Err(_) = window()
                .history()
                .expect("Could not retrieve History")
                .back()
            {
                error!("Could not go back to previous view/route")
            }
        });

        html! {
            <div class="detail-view">
                <header class="detail-view-header">
                    <button class="close-button discreet" onclick=close>
                        <i class="im im-x-mark"></i>
                    </button>
                </header>
                <div class="detail-view-content">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }
}
