use crate::components::modal::Question;
use crate::data_exchange::SetlistDeserializeService;
use crate::helpers::window;
use cqrs::prelude::AsyncRepositoryTrait;
use libchordr::prelude::{Catalog, Setlist, SetlistEntry, SongData};
use log::{error, info};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;
use webchordr_common::data_exchange::SETLIST_LOAD_URL_PREFIX;
use webchordr_common::errors::{SharingError, WebError};
use webchordr_events::Event;
use webchordr_events::SetlistEvent;
use webchordr_persistence::persistence_manager::PMType;
use webchordr_persistence::prelude::SetlistWebRepository;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct SetlistProps {
    pub catalog: Rc<Catalog>,
    pub serialized_setlist: String,
    pub current_setlist: Option<Rc<Setlist>>,
    pub persistence_manager: Arc<PMType>,
    pub on_load: Callback<Event>,
}

pub struct SetlistLoad {
    visible: bool,
    props: SetlistProps,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ChooseNo,
    ChooseYes,
    PrepareSetlist(Setlist),
    LoadSetlist(Setlist),
}

impl SetlistLoad {
    fn build_setlist(&self) -> Result<Setlist, WebError> {
        let serialized_setlist = self.get_shared_data()?;
        let deserialize_result =
            SetlistDeserializeService::deserialize(&serialized_setlist, &*self.props.catalog)?;

        if !deserialize_result.errors.is_empty() {
            let errors = deserialize_result
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            error!("{}", errors);
        }

        let setlist = deserialize_result.setlist;
        let original_name = setlist.name();
        let new_name = format!("{} (imported & unsaved)", original_name);

        Ok(setlist.with_name(new_name))
    }

    fn get_shared_data(&self) -> Result<String, WebError> {
        let hash: String = window().location().hash()?;
        let share_url_prefix = SETLIST_LOAD_URL_PREFIX;
        if hash.starts_with(share_url_prefix) {
            Ok(hash.trim_start_matches(share_url_prefix).to_string())
        } else {
            Err(WebError::sharing_error(SharingError::Deserialization(
                format!("Could not fetch shared data from URL {}", hash),
            )))
        }
    }

    fn render_setlist(&self, setlist: &Setlist) -> Html {
        let render = |song: &SetlistEntry| {
            let title = song.title();
            let transpose_semitone = song.settings().map_or(1, |s| s.transpose_semitone());
            let text = if transpose_semitone == 0 || title.is_empty() {
                title.to_string() // only the (possibly empty) title is used
            } else {
                let prefix = if transpose_semitone > 0 { "+" } else { "" };
                format!("{} ({}{} ♬)", title, prefix, transpose_semitone)
            };

            html! { <li key=title>{text}</li> }
        };

        (html! {
            <div class="setlist-load-preview-viewport">
                <ul>
                    {for setlist.iter().map(&render)}
                </ul>
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
            Msg::ChooseNo => {
                info!("User canceled Setlist import");
                self.visible = false;
                window()
                    .location()
                    .set_href("#/")
                    .expect("Could not change the location href");
            }
            Msg::ChooseYes => {
                let new_setlist = self.build_setlist();
                match new_setlist {
                    Ok(s) => self.link.send_message(Msg::PrepareSetlist(s)),
                    Err(e) => {
                        let _ = window().alert_with_message(&e.to_string());
                    }
                }
            }
            Msg::PrepareSetlist(new_setlist) => {
                let on_load_callback = self.link.callback(Msg::LoadSetlist);
                let pm = self.props.persistence_manager.clone();
                spawn_local(async move {
                    let result = SetlistWebRepository::new(&*pm).find_all().await;
                    let setlist_to_load = match result {
                        Ok(lists) => get_setlist_with_unique_id(new_setlist, &lists),
                        Err(_) => new_setlist,
                    };

                    on_load_callback.emit(setlist_to_load)
                })
            }
            Msg::LoadSetlist(new_setlist) => {
                self.visible = false;
                self.props
                    .on_load
                    .emit(Event::SetlistEvent(SetlistEvent::Replace(new_setlist)))
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.catalog != props.catalog
            && self.props.serialized_setlist != props.serialized_setlist
            && self.props.current_setlist != props.current_setlist
            && self.props.on_load != props.on_load
        {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let new_setlist_result = self.build_setlist();
        (match new_setlist_result {
            Ok(new_setlist) => {
                let on_answer_1 = self.link.callback(|_| Msg::ChooseNo);
                let on_answer_2 = self.link.callback(|_| Msg::ChooseYes);

                let rendered_setlist = self.render_setlist(&new_setlist);
                html! {
                    <Question
                        question_text="Do you want to load the Setlist?"
                        answer_1_text="No"
                        answer_2_text="Yes"
                        on_answer_1=on_answer_1
                        on_answer_2=on_answer_2
                        visible=self.visible
                        class="setlist-load-preview-modal"
                    >
                        <div class="setlist-load-preview-container">
                            <div class="setlist-load-preview">
                                <h3>{"New Setlist"}</h3>
                                {rendered_setlist}
                            </div>
                        </div>
                    </Question>
                }
            }
            Err(e) => {
                error!("{}", e);
                html! {}
            }
        }) as Html
    }
}

fn get_setlist_with_unique_id(new_setlist: Setlist, all_setlists: &[Setlist]) -> Setlist {
    let mut new_id = new_setlist.id();
    while all_setlists.iter().map(|s| s.id()).any(|x| x == new_id) {
        new_id += 1;
    }

    new_setlist.with_id(new_id)
}
