use yew::Callback;
use stdweb::{js, Value};
use crate::events::{SETLIST_CHANGE_SORTING, SortingChange};
use stdweb::web::HtmlElement;
use crate::events::sorting_change::Sorting;
use crate::errors::WebError;


/// Service to make a HtmlElement sortable using [Shopify/draggable](https://github.com/Shopify/draggable)
pub struct SortableService {}

#[must_use]
pub struct SortableHandle(Option<Value>);

impl SortableHandle {
    pub fn destroy(&mut self) -> Result<(), WebError> {
        if let Some(ref sortable) = self.0.take() {
            js! { @(no_return)
                const sortable = @{sortable};

                sortable.destroy();
            }

            Ok(())
        } else {
            Err(WebError::sortable_error("Sortable handle is empty"))
        }
    }
}

impl SortableService {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn make_sortable<E: 'static + ::stdweb::private::JsSerializeOwned + ::stdweb::web::IHtmlElement>(
    pub fn make_sortable(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>,
    ) -> Result<SortableHandle, ()> {
        self.register(element, callback)
    }

    //noinspection RsLiveness
    // fn register<E: 'static + ::stdweb::private::JsSerializeOwned + ::stdweb::web::IHtmlElement>(
    fn register(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>) -> Result<SortableHandle, ()>
    {
        let handler = move |old_index: i64, new_index: i64| {
            callback.emit(SortingChange::new(old_index as Sorting, new_index as Sorting));
        };
        let handle = js!(
            const element = @{element};
            const sortable = new Sortable.default(element, {
                draggable: "a",
                delay: 300
            });

            // sortable.on("sortable:start", (e) => console.log(e, "sortable:start"));
            // sortable.on("sortable:sort", (e) => console.log(e, "sortable:sort"));
            // sortable.on("sortable:sorted", (e) => console.log(e, "sortable:sorted"));
            sortable.on("sortable:stop", (e) => {
                setTimeout(() => {
                    const handler = @{handler};
                    handler(e.oldIndex, e.newIndex);
                }, 100);
                console.debug(e.oldIndex, e.newIndex);

                // create and dispatch the event
                const customEvent = new CustomEvent(@{SETLIST_CHANGE_SORTING}, {
                    detail: {
                        originalEvent: e,
                        oldIndex: e.oldIndex,
                        newIndex: e.newIndex,
                    }
                });
                element.dispatchEvent(customEvent);
            });

            return sortable;
        );

        Ok(SortableHandle(Some(handle)))
    }
}
