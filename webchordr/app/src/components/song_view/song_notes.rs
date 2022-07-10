use log::debug;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement};
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChange(v) => {
                let song_info = &ctx.props().song_info;
                let settings = song_info.song_settings.with_note(v);

                ctx.props().on_change.emit((song_info.song.id(), settings));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // @see https://yew.rs/docs/concepts/html/events#using-jscast
        let onchange = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            input.map(|input| Msg::InputChange(input.value()))
        });

        let song_settings = &ctx.props().song_info.song_settings;
        let notes = song_settings.note().to_owned();
        debug!(
            "Show notes: '{}' from Song Settings {:?}",
            notes, song_settings
        );

        (html! {
            <div class="song-notes">
                <textarea placeholder={"Notes"} onchange={onchange} value={notes} />
            </div>
        }) as Html
    }
}
