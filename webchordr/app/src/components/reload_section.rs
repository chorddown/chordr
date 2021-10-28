use std::rc::Rc;

use chrono::DateTime;
use yew::prelude::*;

use libchordr::prelude::{Catalog, CatalogTrait};

use crate::components::nbsp::Nbsp;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ReloadSectionProps {
    pub catalog: Option<Rc<Catalog>>,
}

pub struct ReloadSection {
    props: ReloadSectionProps,
}

impl Component for ReloadSection {
    type Message = ();
    type Properties = ReloadSectionProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let revision = match &self.props.catalog {
            None => "n/a".to_string(),
            Some(c) => match DateTime::parse_from_rfc2822(&c.revision()) {
                Ok(d) => d.format("%a %d.%m.%y %H:%I").to_string(),
                Err(_) => c.revision(),
            },
        };

        let app_version = format!(
            "{}-{}",
            env!("CARGO_PKG_VERSION"),
            env!("CUNDD_BUILD_REVISION")
        );

        html! {
            <div class="reload-section">
                <div class="reload-button-container">
                    <a class="reload-button" href="/" title="Reload the song catalog">
                        <i class="im im-sync"></i>
                        <Nbsp/>
                        <span>{"Reload the catalog"}</span>
                    </a>
                </div>
                <div class="reload-section-catalog-revision">
                    {"Catalog revision: "}{revision}
                </div>

                <div class="reload-section-version">
                    {"App version: "}{app_version}
                </div>
            </div>
        }
    }
}
