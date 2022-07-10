use self::index::*;
use self::index_item::IndexItem;
use self::link::SongBrowserLink;
use libchordr::models::catalog::*;
use libchordr::models::song_data::SongData;
use libchordr::prelude::SongSorting;
use libchordr::prelude::{ListEntryTrait, Song};
use std::rc::Rc;
use webchordr_common::components::link::Link;
use webchordr_common::route::AppRoute;
use webchordr_song_list::Item as SongItem;
use yew::prelude::*;
use yew::Component;

mod index;
mod index_item;
mod link;

pub struct SongBrowser {}

const SONG_BROWSER_PLACEHOLDER: &str = "_";

#[derive(Properties, PartialEq, Clone)]
pub struct SongBrowserProps {
    pub chars: String,
    pub catalog: Rc<Catalog>,
}

impl SongBrowser {
    /// Return the [Song]s from the [Catalog] filtered by [props.chars]
    fn get_filtered_songs<'a, 'b>(&'a self, ctx: &'b Context<Self>) -> Vec<&'b Song> {
        let songs: Vec<&Song> = if self.has_chars(ctx) {
            let chars = &ctx.props().chars;
            ctx.props()
                .catalog
                .iter()
                .filter(|s| str::starts_with(&s.title().to_lowercase(), chars))
                .collect()
        } else {
            ctx.props().catalog.iter().collect()
        };

        songs.sort_by_title()
    }

    /// Return the indexes for the filtered [Song]s
    fn get_indexes_for_filtered_songs(&self, ctx: &Context<Self>) -> Vec<Index> {
        let root_chars = if self.has_chars(ctx) {
            &ctx.props().chars
        } else {
            ""
        };
        build_indexes(self.get_filtered_songs(ctx), root_chars)
    }

    fn has_chars(&self, ctx: &Context<Self>) -> bool {
        let chars = &ctx.props().chars;

        !chars.is_empty() && chars != SONG_BROWSER_PLACEHOLDER
    }

    fn get_back_link(&self, ctx: &Context<Self>) -> Html {
        (if self.has_chars(ctx) {
            let chars = &ctx.props().chars;
            let parameter = self.get_back_link_parameter(ctx, chars);

            let to = AppRoute::SongBrowser { chars: parameter };

            html! { <Link class="song-browser-back back-link -inline" {to}><i class="im im-angle-left"></i>{ "Back" }</Link> }
        } else {
            html! {}
        }) as Html
    }

    fn get_back_link_parameter(&self, ctx: &Context<Self>, chars: &str) -> String {
        if !self.has_chars(ctx) {
            return SONG_BROWSER_PLACEHOLDER.to_owned();
        }

        let count = char_count(chars);
        if count < 1 {
            return SONG_BROWSER_PLACEHOLDER.to_owned();
        }
        let sub_string = sub_string(chars, count - 1);
        if sub_string.is_empty() {
            SONG_BROWSER_PLACEHOLDER.to_owned()
        } else {
            sub_string
        }
    }

    fn render_header(&self, ctx: &Context<Self>) -> Html {
        let header_suffix = if self.has_chars(ctx) {
            html! { <>{":"} <span class="song-browser-header-suffix">{&ctx.props().chars}</span></> }
        } else {
            html! {}
        };

        html! { <h1><SongBrowserLink />{"Browse Songs"}{header_suffix}</h1> }
    }
}

impl Component for SongBrowser {
    type Message = ();
    type Properties = SongBrowserProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SongBrowser {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let songs = self.get_filtered_songs(ctx);

        if songs.len() > 24 || !self.has_chars(ctx) {
            self.render_index(ctx)
        } else {
            self.render_songs(ctx, songs)
        }
    }
}

impl SongBrowser {
    fn render_index(&self, ctx: &Context<Self>) -> Html {
        let render_index_item = |index: Index| {
            let key = index.chars.clone();

            html! {
                <div class="col-xs-12 col-sm-6 col-3">
                    <IndexItem class="song-browser-index-item grid-button"
                        key={key}
                        index={index}/>
                </div>
            }
        };

        let indexes_for_filtered_songs = self.get_indexes_for_filtered_songs(ctx);

        html! {
            <div class="song-browser-index-list">
                {self.render_header(ctx)}
                <div class="row grid">
                    { for indexes_for_filtered_songs.into_iter().map(render_index_item) }
                </div>
                {self.get_back_link(ctx)}
            </div>
        }
    }

    fn render_songs(&self, ctx: &Context<Self>, songs: Vec<&Song>) -> Html {
        let render_song_item = |song: &Song| {
            let data_key = song.title();
            let song_id = song.id();
            let key = song_id.as_str();

            html! {
                <SongItem<Song> class="song-item grid-button"
                    key={key}
                    data_key={data_key}
                    song={song.clone()}/>
            }
        };

        html! {
            <div class="song-browser-song-list song-list">
                {self.render_header(ctx)}
                <div class="columns-md-2">
                    { for songs.into_iter().map(render_song_item) }
                </div>
                {self.get_back_link(ctx)}
            </div>
        }
    }
}
