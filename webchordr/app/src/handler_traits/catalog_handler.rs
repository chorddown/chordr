use yew::{Component, Context};

pub trait CatalogHandler: Component {
    fn fetch_catalog(&mut self, ctx: &Context<Self>);

    fn commit_changes(&mut self);
}
