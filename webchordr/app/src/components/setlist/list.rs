use std::rc::Rc;
use std::sync::Arc;

use gloo_dialogs::confirm;
use log::{debug, error};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use cqrs::prelude::AsyncRepositoryTrait;
use libchordr::models::setlist::Setlist;
use webchordr_common::errors::WebError;
use webchordr_events::{Event, SetlistEvent};
use webchordr_persistence::persistence_manager::PMType;
use webchordr_persistence::prelude::SetlistWebRepository;

use crate::state::State;

use super::add_button::AddButton;

use self::item::Item;

mod item;

#[derive(Properties, Clone)]
pub struct ListProps {
    pub setlists: Vec<Setlist>,
    pub persistence_manager: Arc<PMType>,
    pub state: Rc<State>,
    pub on_event: Callback<Event>,
}

pub struct List {
    props: ListProps,
    link: ComponentLink<Self>,
    setlists: Option<Vec<Setlist>>,
    error: Option<WebError>,
}

pub enum Msg {
    FindAll,
    Load(Setlist),
    Add(Setlist),
    Delete(Setlist),

    SetlistsLoaded(Vec<Setlist>),
    LoadError(WebError),
}

impl Component for List {
    type Message = Msg;
    type Properties = ListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            setlists: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FindAll => self.find_all_setlists(),
            Msg::Add(setlist) => self.persist_new_setlist(setlist),
            Msg::Load(setlist) => self.load_setlist(setlist),
            Msg::Delete(setlist) => self.delete_setlist(setlist),
            Msg::SetlistsLoaded(v) => self.setlists = Some(v),
            Msg::LoadError(e) => self.error = Some(e),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.setlists != props.setlists
            || self.props.state != props.state
            || self.props.on_event != props.on_event
        {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.setlists.is_none() {
            self.find_all_setlists();

            return html! {};
        }
        let state = self.props.state.clone();
        let current_setlist = state.current_setlist();
        let render = |setlist: &Setlist| {
            let key = setlist.id();
            let on_load_click = self.link.callback(|s| Msg::Load(s));
            let on_delete_click = self.link.callback(|s| Msg::Delete(s));

            let highlight = match current_setlist {
                Some(ref c) => c.id() == setlist.id(),
                None => false,
            };

            html! {
                <li key=key>
                    <Item
                        on_load_click=on_load_click
                        on_delete_click=on_delete_click
                        setlist=setlist.clone()
                        highlight=highlight
                    />
                </li>
            }
        };

        let entries = self.setlists.as_ref().unwrap().iter();
        let on_add_button_click = self.link.callback(|s| Msg::Add(s));
        debug!("Redraw {} setlists", entries.len());

        (html! {
            <div class="setlist-list">
                <ul>
                    {for entries.map(render)}
                </ul>
                <div class="button-group">
                    <AddButton
                        text="Create new setlist"
                        state=state.clone()
                        on_click=on_add_button_click.clone()
                        clone_current=false
                    />
                    <AddButton
                        text="Copy current setlist"
                        state=state
                        on_click=on_add_button_click
                        clone_current=true
                    />
                </div>
            </div>
        }) as Html
    }
}

impl List {
    fn find_all_setlists(&self) {
        let pm = self.props.persistence_manager.clone();
        let finished = self.link.callback(|result: Result<_, _>| match result {
            Ok(l) => Msg::SetlistsLoaded(l),
            Err(e) => Msg::LoadError(e),
        });

        spawn_local(async move {
            let result = SetlistWebRepository::new(&*pm).find_all().await;

            finished.emit(result)
        });
    }

    fn persist_new_setlist(&mut self, setlist: Setlist) {
        let pm = self.props.persistence_manager.clone();
        let on_ok = self
            .link
            .batch_callback(|s| vec![Msg::FindAll, Msg::Load(s)]);

        spawn_local(async move {
            let result = SetlistWebRepository::new(&*pm).add(setlist.clone()).await;

            match result {
                Ok(_) => on_ok.emit(setlist),
                Err(e) => error!("Failed to add the new setlist {:?}: {}", setlist, e),
            }
        });
    }

    fn load_setlist(&self, setlist: Setlist) {
        self.props
            .on_event
            .emit(SetlistEvent::SetCurrentSetlist(setlist).into())
    }

    fn delete_setlist(&self, setlist: Setlist) {
        if !confirm(&format!("Delete setlist '{}'?", setlist.name())) {
            return;
        }

        let send_reload = self.link.callback(|_| Msg::FindAll);
        let pm = self.props.persistence_manager.clone();

        spawn_local(async move {
            let result = SetlistWebRepository::new(&*pm)
                .delete(setlist.clone())
                .await;

            match result {
                Ok(_) => send_reload.emit(()),
                Err(e) => error!("Failed to add the new setlist {:?}: {}", setlist, e),
            }
        });
    }
}
