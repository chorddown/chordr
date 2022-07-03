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
                    match event.data().into_serde::<UpdateInfo>() {
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

/// This check relies on the debug-representation to check if the Service Worker Container contains
/// a valid ServiceWorker instance
fn has_service_worker(service_worker: &ServiceWorkerContainer) -> bool {
    "ServiceWorkerContainer { obj: EventTarget { obj: Object { obj: JsValue(undefined) } } }"
        != &format!("{:?}", service_worker)
}
