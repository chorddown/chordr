use yew::prelude::*;
use stdweb::console;
use crate::components::modal::Question;
use crate::events::SetlistEvent;
use std::rc::Rc;
use libchordr::prelude::{Catalog, Setlist, SetlistEntry};
use crate::events::Event;

#[derive(Properties, PartialEq, Clone)]
pub struct SetlistProps {
    pub catalog: Rc<Catalog>,
    pub serialized_setlist: String,

    pub on_load: Callback<Event>,
}

pub struct SetlistLoad {
    visible: bool,
    props: SetlistProps,
    link: ComponentLink<Self>,
}

#[allow(dead_code)]
pub enum Msg {
    Ignore,
    ChooseNo,
    ChooseYes,
}

impl SetlistLoad {
    fn build_setlist(&self) -> Setlist<SetlistEntry> {
        Setlist::with_entries(self.collect_setlist_entries())
    }

    fn collect_setlist_entries(&self) -> Vec<SetlistEntry> {
        self.props.serialized_setlist
            .split(',')
            .filter_map(|song_id| {
                self.props.catalog.get(song_id)
            })
            .map(|song| SetlistEntry::from_song(song))
            .collect()
    }
}

impl Component for SetlistLoad {
    type Message = Msg;
    type Properties = SetlistProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            visible: true,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {}
            Msg::ChooseNo => {
                self.visible = false;
            }
            Msg::ChooseYes => {
                let new_setlist = self.build_setlist();
                self.props.on_load.emit(Event::SetlistEvent(SetlistEvent::Replace(new_setlist)));
                self.visible = false;
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <Question
                question_text="Do you want to load the Setlist and delete yours?"
                answer_1_text="No"
                answer_2_text="Yes"
                on_answer_1=self.link.callback(|_|{console!(log, "choose 1");Msg::ChooseNo})
                on_answer_2=self.link.callback(|_|{console!(log, "choose 2");Msg::ChooseYes})
                visible=self.visible
            />
        }
    }
}

