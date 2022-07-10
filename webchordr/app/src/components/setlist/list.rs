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

impl PartialEq for ListProps {
    fn eq(&self, other: &Self) -> bool {
        self.setlists == other.setlists
            && self.state == other.state
            && self.on_event == other.on_event
    }
}
pub struct List {
    setlists: Option<Vec<Setlist>>,
    error: Option<WebError>,
}

pub enum Msg {
    FindAll,
    Load(Rc<Setlist>),
    Add(Setlist),
    Delete(Rc<Setlist>),

    SetlistsLoaded(Vec<Setlist>),
    LoadError(WebError),
}

impl Component for List {
    type Message = Msg;
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            setlists: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FindAll => self.find_all_setlists(ctx),
            Msg::Add(setlist) => self.persist_new_setlist(ctx, setlist),
            Msg::Load(setlist) => self.load_setlist(ctx, (*setlist).clone()),
            Msg::Delete(setlist) => self.delete_setlist(ctx, (*setlist).clone()),
            Msg::SetlistsLoaded(v) => self.setlists = Some(v),
            Msg::LoadError(e) => self.error = Some(e),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.setlists.is_none() {
            self.find_all_setlists(ctx);

            return html! {};
        }
        let state = ctx.props().state.clone();
        let current_setlist = state.current_setlist();
        let render = |setlist: &Setlist| {
            let key = setlist.id();
            let on_load_click = ctx.link().callback(Msg::Load);
            let on_delete_click = ctx.link().callback(Msg::Delete);

            let highlight = match current_setlist {
                Some(ref c) => c.id() == setlist.id(),
                None => false,
            };

            let setlist_prop = Rc::new(setlist.clone());

            html! {
                <li key={key}>
                    <Item
                        {on_load_click}
                        {on_delete_click}
                        setlist={setlist_prop}
                        {highlight}
                    />
                </li>
            }
        };

        let entries = self.setlists.as_ref().unwrap().iter();
        let on_add_button_click = ctx.link().callback(Msg::Add);
        debug!("Redraw {} setlists", entries.len());

        (html! {
            <div class="setlist-list">
                <ul>
                    {for entries.map(render)}
                </ul>
                <div class="button-group">
                    <AddButton
                        text="Create empty setlist"
                        state={state.clone()}
                        on_click={on_add_button_click.clone()}
                        clone_current={false}
                    />
                    <AddButton
                        text="Copy current setlist"
                        {state}
                        on_click={on_add_button_click}
                        clone_current={true}
                    />
                </div>
            </div>
        }) as Html
    }
}

impl List {
    fn find_all_setlists(&self, ctx: &Context<Self>) {
        let pm = ctx.props().persistence_manager.clone();
        let finished = ctx
            .link()
            .callback(|result: Result<Vec<Setlist>, _>| match result {
                Ok(mut l) => {
                    l.sort_by(|a, b| a.name().cmp(b.name()));
                    Msg::SetlistsLoaded(l)
                }
                Err(e) => Msg::LoadError(e),
            });

        spawn_local(async move {
            let result = SetlistWebRepository::new(&*pm).find_all().await;

            finished.emit(result)
        });
    }

    fn persist_new_setlist(&mut self, ctx: &Context<Self>, setlist: Setlist) {
        let pm = ctx.props().persistence_manager.clone();
        let on_ok = ctx
            .link()
            .batch_callback(|s| vec![Msg::FindAll, Msg::Load(s)]);

        spawn_local(async move {
            let result = SetlistWebRepository::new(&*pm).add(setlist.clone()).await;

            match result {
                Ok(_) => on_ok.emit(Rc::new(setlist)),
                Err(e) => error!("Failed to add the new setlist {:?}: {}", setlist, e),
            }
        });
    }

    fn load_setlist(&self, ctx: &Context<Self>, setlist: Setlist) {
        ctx.props()
            .on_event
            .emit(SetlistEvent::SetCurrentSetlist(setlist).into())
    }

    fn delete_setlist(&self, ctx: &Context<Self>, setlist: Setlist) {
        if !confirm(&format!("Delete setlist '{}'?", setlist.name())) {
            return;
        }

        let send_reload = ctx.link().callback(|_| Msg::FindAll);
        let pm = ctx.props().persistence_manager.clone();

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
