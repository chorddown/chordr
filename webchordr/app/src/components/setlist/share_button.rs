use crate::components::modal::Modal;
use crate::data_exchange::SetlistSerializeService;
use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use log::debug;
use std::rc::Rc;
use webchordr_common::data_exchange::SETLIST_LOAD_URL_PREFIX;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SetlistProps {
    pub setlist: Rc<Setlist>,
}

pub struct SetlistShareButton {
    modal_visible: bool,
    props: SetlistProps,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Msg {
    Show,
    Hide,
    Toggle,
}

impl Component for SetlistShareButton {
    type Message = Msg;
    type Properties = SetlistProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            modal_visible: false,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Show => {
                debug!("{:?}", msg);
                self.modal_visible = true
            }
            Msg::Hide => {
                debug!("{:?}", msg);
                self.modal_visible = false
            }
            Msg::Toggle => {
                debug!("{:?}", msg);
                self.modal_visible = !self.modal_visible
            }
        }
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
        let handle_click = self.link.callback(|_| Msg::Toggle);
        let button = html! {
            <button onclick=handle_click title="Share">
                <i class="im im-share"></i>
                <span>{ "Share" }</span>
            </button>
        };

        (if !self.modal_visible {
            button
        } else {
            let modal = self.build_modal();

            html! {
                <>
                    {button}
                    {modal}
                </>
            }
        }) as Html
    }
}

impl SetlistShareButton {
    fn build_share_url(&self) -> Result<String, WebError> {
        let host = crate::helpers::window().location().host()?;
        let serialized = SetlistSerializeService::serialize(self.props.setlist.as_ref())?;

        Ok(format!(
            "{}/{}{}",
            host, SETLIST_LOAD_URL_PREFIX, serialized
        ))
    }

    fn build_modal(&self) -> Html {
        (match self.build_share_url() {
            Ok(share_url) => {
                let handle_modal_close = self.link.callback(|_| Msg::Hide);

                html! {
                    <Modal
                        visible=true
                        header_text="Share your Setlist"
                        class="setlist-share"
                        on_close=handle_modal_close
                        >
                        <div class="clipboard-widget setlist-share">
                            <input type="text" id="setlist-share" readonly=true value=share_url/>
                            <button class="btn" data-clipboard-target="#setlist-share">
                                <i class="im im-copy"></i>
                            </button>
                        </div>
                    </Modal>
                }
            }
            Err(e) => {
                html! {
                    <Modal
                        visible=true
                        header_text="Error building Share-URL"
                        class="setlist-share">
                        {e}
                    </Modal>
                }
            }
        }) as Html
    }
}
