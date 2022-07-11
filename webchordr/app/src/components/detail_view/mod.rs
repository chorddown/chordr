use log::error;
use webchordr_common::prelude::*;
use yew::prelude::*;

use crate::helpers::window;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailViewProps {
    pub children: Children,
    #[prop_or_default]
    pub close_route: Option<AppRoute>,
}

pub struct DetailView {}

impl Component for DetailView {
    type Message = ();
    type Properties = DetailViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        (html! {
            <div class="detail-view">
                <header class="detail-view-header">
                    { self.render_close_button(ctx) }
                </header>
                <div class="detail-view-content">
                    { ctx.props().children.clone() }
                </div>
            </div>
        }) as Html
    }
}

impl DetailView {
    fn render_close_button(&self, ctx: &Context<Self>) -> Html {
        if let Some(to) = &ctx.props().close_route {
            return html! {
                <Link role="button" class="close-button discreet" to={to.clone()}>
                    <i class="im im-x-mark"></i>
                </Link>
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
