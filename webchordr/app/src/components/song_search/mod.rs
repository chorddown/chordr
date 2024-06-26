use std::rc::Rc;

use gloo_timers::callback::Timeout;
use libchordr::models::meta::tags::Tag;
use libchordr::models::meta::MetaTrait;
use log::{debug, info};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Component;

use libchordr::models::catalog::*;
use libchordr::models::list::ListEntryTrait;
use libchordr::prelude::{SearchIndex, Song, SongData, SongSorting};
use webchordr_common::components::link::Link;
use webchordr_common::route::AppRoute;
use webchordr_song_list::Item as SongItem;

use self::link::SongSearchLink;

mod link;

pub struct SongSearch {
    search: String,
    catalog_revision: String,
    search_index: Option<SearchIndex>,
    timeout: Option<Timeout>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SongSearchProps {
    pub catalog: Rc<Catalog>,
    pub show_back_button: bool,
}

impl SongSearch {
    /// Return the [Song]s from the [Catalog] filtered by [self.search]
    fn get_filtered_songs<'a>(&'a self, props: &'a SongSearchProps) -> Vec<&'a Song> {
        if self.search.is_empty() || self.search_index.is_none() {
            self.get_all_songs(props)
        } else {
            self.search_index
                .as_ref()
                .expect("Search index not built yet")
                .search_by_term(&self.search)
                .sort_by_title()
        }
    }

    fn get_all_songs<'a>(&self, props: &'a SongSearchProps) -> Vec<&'a Song> {
        props.catalog.iter().collect::<Vec<&Song>>().sort_by_title()
    }

    fn needs_to_build_search_index(&self) -> bool {
        !self.search.is_empty() && !self.search.trim().is_empty() && self.search_index.is_none()
    }

    fn get_back_link(&self, ctx: &Context<Self>) -> Html {
        (if ctx.props().show_back_button {
            html! { <Link class="song-search-back back-link -inline" to={AppRoute::Index}><i class="im im-angle-left"></i>{ "Back" }</Link> }
        } else {
            html! {}
        }) as Html
    }

    fn render_filter(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            Msg::SearchChange(e.target_unchecked_into::<HtmlInputElement>().value())
        });

        (html! {
            <>
                <h1><SongSearchLink />{"Search Songs"}</h1>
                <input type="search"
                       value={self.search.clone()}
                       {oninput}
                       placeholder="Search"/>
            </>
        }) as Html
    }

    fn render_tags(&self, ctx: &Context<Self>) -> Html {
        let render_tag = |tag: Tag| {
            let tag_string = tag.to_string();
            let on_click = ctx
                .link()
                .callback(move |_: MouseEvent| Msg::SearchChange(tag_string.clone()));

            html! {
                <button type="button" role="button" onclick={on_click} key={tag.to_string()}>
                    {tag.to_string_without_hashtag()}
                </button>
            }
        };

        let catalog = &ctx.props().catalog;
        if catalog.is_empty() {
            return html! {};
        };

        let mut unique_tags: Vec<Tag> = catalog
            .iter()
            .into_iter()
            .flat_map(|s| s.meta().tags())
            .collect::<std::collections::HashSet<Tag>>()
            .into_iter()
            .collect::<Vec<_>>();
        unique_tags.sort_unstable();

        html! {
            <div class="song-search-tags">
                <div class="song-search-tags-caption">{"Tags"}</div>
                { for unique_tags.into_iter().map(render_tag) }
            </div>
        }
    }
}

pub enum Msg {
    SearchChange(String),
    Debounce(String),
    BuildSearchIndex,
}

impl Component for SongSearch {
    type Message = Msg;
    type Properties = SongSearchProps;

    fn create(ctx: &Context<Self>) -> Self {
        SongSearch {
            search: String::new(),
            catalog_revision: ctx.props().catalog.revision(),
            search_index: None,
            timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SearchChange(new_search) => {
                // Clear the previous timer if one exists
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                }

                // Debounce the handling of user input
                let debounced = ctx.link().callback(Msg::Debounce);
                self.timeout = Some(Timeout::new(100, move || {
                    debounced.emit(new_search);
                }));

                false
            }

            Msg::Debounce(new_search) => {
                info!("New search {}", new_search);
                self.search = new_search;

                true
            }

            Msg::BuildSearchIndex => {
                self.search_index = Some(build_search_index_from_props(ctx.props()));

                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let catalog_changed = ctx.props().catalog.revision() != self.catalog_revision;
        if catalog_changed {
            self.search_index = None;
            self.catalog_revision = ctx.props().catalog.revision();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let render_song_item = |song: &Song| {
            let data_key = song.title();
            let song_id = song.id();
            let key = song_id.as_str();

            html! {
                <SongItem<Song> class="song-item button"
                    {key}
                    {data_key}
                    song={song.clone()}/>
            }
        };

        let props = ctx.props();
        let songs = if self.needs_to_build_search_index() {
            ctx.link().send_message(Msg::BuildSearchIndex);

            self.get_all_songs(props)
        } else {
            self.get_filtered_songs(props)
        };

        let inner = if !songs.is_empty() {
            html! { <div class="song-search-results">{ for songs.into_iter().map(render_song_item) }</div>}
        } else {
            html! { <div class="song-search-results -no-results">{"No matching songs found"}</div>}
        };

        let filter = self.render_filter(ctx);
        let back = self.get_back_link(ctx);
        let tags = self.render_tags(ctx);

        html! {
            <div class="song-search-song-list song-list">
                {filter}
                {tags}
                {inner}
                {back}
            </div>
        }
    }
}

fn build_search_index_from_props(props: &SongSearchProps) -> SearchIndex {
    debug!("Build search index");
    let search_index = SearchIndex::build_for_catalog(props.catalog.clone());
    debug!("Did build search index");

    search_index
}
