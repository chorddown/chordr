pub use crate::control::navigate::Navigate;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};
use yew::Callback;

pub mod navigate;

#[derive(Debug)]
pub enum Control {
    Navigate(Navigate),
}

pub struct KeyboardControl {
    _listener: EventListener,
}

impl KeyboardControl {
    pub fn new(callback: Callback<Control>) -> Self {
        let document = crate::helpers::window()
            .document()
            .expect("Could not detect the JS document");
        let inner_callback = move |event: &Event| {
            if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
                if event.code() == "ArrowRight" {
                    callback.emit(Control::Navigate(Navigate::NextSong))
                } else if event.code() == "ArrowLeft" {
                    callback.emit(Control::Navigate(Navigate::PreviousSong))
                }
            }
        };

        let listener = EventListener::new(&document, "keydown", inner_callback);
        Self {
            _listener: listener,
        }
    }
}
