use crate::components::modal::Question;
use crate::events::Event;
use crate::events::SetlistEvent;
use libchordr::prelude::{Catalog, Setlist, SetlistEntry};
use std::rc::Rc;
use log::error;
use stdweb::console;
use yew::prelude::*;
use crate::data_exchange::SetlistDeserializeService;

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
        let deserialize_result = SetlistDeserializeService::deserialize(&self.props.serialized_setlist, &self.props.catalog);

        if deserialize_result.errors.len() > 0 {
            let errors = deserialize_result.errors.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ");
            error!("{}", errors);
        }

        deserialize_result.setlist
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
                self.props
                    .on_load
                    .emit(Event::SetlistEvent(SetlistEvent::Replace(new_setlist)));
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
