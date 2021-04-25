use log::{debug, error};
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
    InputChange(ChangeData),
}

pub struct SongNotes {
    props: SongNotesProps,
    link: ComponentLink<Self>,
}

impl Component for SongNotes {
    type Message = Msg;
    type Properties = SongNotesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputChange(ChangeData::Value(v)) => {
                let settings = self.props.song_info.song_settings.with_note(v);

                self.props
                    .on_change
                    .emit((self.props.song_info.song.id(), settings));
            }
            Msg::InputChange(change_data) => error!("Invalid change data {:?}", change_data),
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
        let onchange = self.link.callback(|e: ChangeData| Msg::InputChange(e));
        let notes = self.props.song_info.song_settings.note();
        debug!(
            "Show notes: '{}' from Song Settings {:?}",
            notes, self.props.song_info.song_settings
        );

        (html! {
            <div class="song-notes">
                <textarea placeholder={"Notes"} onchange=onchange value=notes />
            </div>
        }) as Html
    }
}
