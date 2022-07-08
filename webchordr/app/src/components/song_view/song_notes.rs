use log::{debug, error};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use libchordr::prelude::{ListEntryTrait, SongId, SongSettings};

use crate::state::SongInfo;

#[derive(Properties, Clone)]
pub struct SongNotesProps {
    pub on_change: Callback<(SongId, SongSettings)>,
    pub song_info: SongInfo,
}

impl PartialEq for SongNotesProps {
    fn eq(&self, other: &Self) -> bool {
        self.song_info == other.song_info
    }
}

pub enum Msg {
    InputChange(String),
}

pub struct SongNotes {}

impl Component for SongNotes {
    type Message = Msg;
    type Properties = SongNotesProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChange(v) => {
                let settings = ctx.props().song_info.song_settings.with_note(v);

                ctx.props()
                    .on_change
                    .emit((ctx.props().song_info.song.id(), settings));
            } // Msg::InputChange(change_data) => error!("Invalid change data {:?}", change_data),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // Use batch_callback so if something unexpected happens we can return
        // None and do nothing
        let onchange = link.batch_callback(|e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            input.map(|input| Msg::InputChange(input.value()))
        });

        // let onchange = ctx.link().callback(Msg::InputChange);
        let notes = ctx.props().song_info.song_settings.note().to_owned();
        debug!(
            "Show notes: '{}' from Song Settings {:?}",
            notes,
            ctx.props().song_info.song_settings
        );

        (html! {
            <div class="song-notes">
                <textarea placeholder={"Notes"} onchange={onchange} value={notes} />
            </div>
        }) as Html
    }
}
