use crate::components::modal::Question;
use crate::data_exchange::SetlistDeserializeService;
use libchordr::prelude::{Catalog, Setlist, SetlistEntry, SongData};
use log::{error, info};
use std::rc::Rc;
use web_sys::window;
use webchordr_events::Event;
use webchordr_events::SetlistEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SetlistProps {
    pub catalog: Rc<Catalog>,
    pub serialized_setlist: String,
    pub current_setlist: Option<Rc<Setlist>>,
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
    fn build_setlist(&self) -> Setlist {
        let deserialize_result = SetlistDeserializeService::deserialize(
            &self.props.serialized_setlist,
            &*self.props.catalog,
        );

        if deserialize_result.errors.len() > 0 {
            let errors = deserialize_result
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            error!("{}", errors);
        }

        deserialize_result.setlist
    }

    fn render_setlist(&self, setlist: &Setlist) -> Html {
        let render = |song: SetlistEntry| {
            let key = song.title();

            html! { <li key=key.clone()>{key}</li> }
        };

        (html! {
            <div class="setlist-load-preview-viewport">
                <ul>
                    {for setlist.clone().into_iter().map(&render)}
                </ul>
            </div>
        }) as Html
    }

    fn render_empty_setlist(&self) -> Html {
        (html! {
            <div class="setlist-load-preview-viewport">
                <ul></ul>
            </div>
        }) as Html
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
                info!("User canceled Setlist import");
                self.visible = false;
                window()
                    .expect("Could not detect the JS window object")
                    .location()
                    .set_href("#/")
                    .expect("Could not change the location href");
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let on_answer_1 = self.link.callback(|_| Msg::ChooseNo);
        let on_answer_2 = self.link.callback(|_| Msg::ChooseYes);

        let current_setlist = match &self.props.current_setlist {
            Some(s) => self.render_setlist(s),
            None => self.render_empty_setlist(),
        };
        let new_setlist = self.render_setlist(&self.build_setlist());

        html! {
            <Question
                question_text="Do you want to load the Setlist and delete yours?"
                answer_1_text="No"
                answer_2_text="Yes"
                on_answer_1=on_answer_1
                on_answer_2=on_answer_2
                visible=self.visible
                class="setlist-load-preview-modal"
            >
                <div class="setlist-load-preview-container">
                    <div class="setlist-load-preview-col">
                        <h3>{"Your Setlist"}</h3>
                        {current_setlist}
                    </div>
                    <div class="setlist-load-preview-col">
                        <h3>{"New Setlist"}</h3>
                        {new_setlist}
                    </div>
                </div>
            </Question>
        }
    }
}
