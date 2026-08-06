use crate::ipc::update_info::UpdateInfo;
use gloo_events::EventListener;
use log::warn;
use wasm_bindgen::JsCast;
use web_sys::ServiceWorkerContainer;
use yew::Callback;

pub mod update_info;

pub enum IpcMessage {
    UpdateInfo(UpdateInfo),
}

pub fn register_ipc_handler(message_callback: Callback<IpcMessage>) -> Option<EventListener> {
    let service_worker = crate::helpers::window().navigator().service_worker();
    if has_service_worker(&service_worker) {
        Some(EventListener::new(
            &service_worker,
            "message",
            move |event: &web_sys::Event| {
                if let Some(event) = event.dyn_ref::<web_sys::MessageEvent>() {
                    match serde_wasm_bindgen::from_value::<UpdateInfo>(event.data()) {
                        Ok(version_info) => {
                            message_callback.emit(IpcMessage::UpdateInfo(version_info))
                        }
                        Err(_) => {
                            warn!("Unsupported message");
                        }
                    };
                } else {
                    warn!("{:?}", event)
                }
            },
        ))
    } else {
        None
    }
}

fn has_service_worker(service_worker: &ServiceWorkerContainer) -> bool {
    if service_worker.is_undefined() {
        return false;
    }

    if let Some(c) = service_worker.controller() {
        !c.is_null() && !c.is_undefined()
    } else {
        false
    }
}
