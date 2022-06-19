use crate::ipc::update_info::UpdateInfo;
use gloo_events::EventListener;
use log::warn;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::Callback;

pub mod update_info;

pub enum IpcMessage {
    UpdateInfo(UpdateInfo),
}

pub fn register_ipc_handler(message_callback: Callback<IpcMessage>) -> EventListener {
    EventListener::new(
        &crate::helpers::window().navigator().service_worker(),
        "message",
        move |event: &web_sys::Event| {
            let event: &web_sys::MessageEvent = event.dyn_ref().unwrap_throw();

            match event.data().into_serde::<UpdateInfo>() {
                Ok(version_info) => message_callback.emit(IpcMessage::UpdateInfo(version_info)),
                Err(_) => {
                    warn!("Unsupported message");
                }
            };
        },
    )
}
