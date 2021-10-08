use crate::helpers::window;
use log::error;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailViewProps {
    pub children: Children,
    #[prop_or_default]
    pub close_uri: Option<&'static str>,
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
        html! {
            <div class="detail-view">
                <header class="detail-view-header">
                    { self.render_close_button() }
                </header>
                <div class="detail-view-content">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }
}

impl DetailView {
    fn render_close_button(&self) -> Html {
        if let Some(href) = self.props.close_uri {
            return html! {
                <a role="button" class="close-button discreet" href=href>
                    <i class="im im-x-mark"></i>
                </a>
            };
        }

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
            <button class="close-button discreet" onclick=close>
                <i class="im im-x-mark"></i>
            </button>
        }
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn dv_test() {}
}
