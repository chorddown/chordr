use yew::prelude::*;
use log::debug;
use crate::components::modal::Modal;
use stdweb::web::window;
use crate::errors::WebError;
use libchordr::models::setlist::Setlist;
use std::rc::Rc;
use libchordr::prelude::SetlistEntry;
use crate::setlist_serializer_service::SetlistSerializerService;

#[derive(Properties, PartialEq, Clone)]
pub struct SetlistProps {
    pub setlist: Rc<Setlist<SetlistEntry>>,
}

pub struct SetlistShareButton {
    modal_visible: bool,
    props: SetlistProps,
    link: ComponentLink<Self>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Msg {
    ShowModal,
    HideModal,
    ToggleModal,
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
            Msg::ShowModal => {
                debug!("{:?}", msg);
                self.modal_visible = true
            }
            Msg::HideModal => {
                debug!("{:?}", msg);
                self.modal_visible = false
            }
            Msg::ToggleModal => {
                debug!("{:?}", msg);
                self.modal_visible = !self.modal_visible
            }
        }
        true
    }

    fn view(&self) -> Html {
        let handle_click = self.link.callback(|_| Msg::ToggleModal);
        let button = html! { <button onclick=handle_click><i class="im im-share"></i></button> };

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
        let host = match window().location() {
            Some(location) => location.host()?,
            None => return Err(WebError::js_error("Could not fetch current location"))
        };
        let serialized = SetlistSerializerService::build_share_url(self.props.setlist.as_ref())?;

        Ok(format!("{}/#/setlist/load/{}", host, serialized))
    }

    fn build_modal(&self) -> Html {
        (match self.build_share_url() {
            Ok(share_url) => {
                let handle_modal_close = self.link.callback(|_| Msg::HideModal);

                html! {
                    <Modal
                        visible=true
                        header_text="Share your Setlist"
                        class="setlist-share"
                        on_close=handle_modal_close
                        >
                        <div class="clipboard-widget setlist-share">
                            <input id="setlist-share" value=share_url/>
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
