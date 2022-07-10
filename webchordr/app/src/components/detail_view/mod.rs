use log::error;
use yew::prelude::*;

use crate::helpers::window;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailViewProps {
    pub children: Children,
    #[prop_or_default]
    pub close_uri: Option<&'static str>,
}

pub struct DetailView {}

impl Component for DetailView {
    type Message = ();
    type Properties = DetailViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="detail-view">
                <header class="detail-view-header">
                    { self.render_close_button(ctx) }
                </header>
                <div class="detail-view-content">
                    { ctx.props().children.clone() }
                </div>
            </div>
        }
    }
}

impl DetailView {
    fn render_close_button(&self, ctx: &Context<Self>) -> Html {
        if let Some(href) = ctx.props().close_uri {
            return html! {
                <a role="button" class="close-button discreet" {href}>
                    <i class="im im-x-mark"></i>
                </a>
            };
        }

        let close = ctx.link().callback(|_| {
            if window()
                .history()
                .expect("Could not retrieve History")
                .back()
                .is_err()
            {
                error!("Could not go back to previous view/route")
            }
        });

        html! {
            <button class="close-button discreet" onclick={close}>
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
