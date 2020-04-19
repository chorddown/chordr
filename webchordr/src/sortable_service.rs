use crate::errors::WebError;
use crate::events::sorting_change::Sorting;
use crate::events::{SortingChange, SETLIST_CHANGE_SORTING};
use stdweb::web::HtmlElement;
use stdweb::{js, Value};
use yew::Callback;

/// Service to make a HtmlElement sortable using [Shopify/draggable](https://github.com/Shopify/draggable)
pub struct SortableService {}

#[must_use]
pub struct SortableHandle(Option<Value>);

impl SortableHandle {
    pub fn destroy(&mut self) -> Result<(), WebError> {
        if let Some(ref sortable) = self.0.take() {
            js! { @(no_return)
                const sortable = @{sortable};
                console.debug("Destroy sortable", sortable);

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

    //noinspection RsLiveness
    pub fn make_sortable(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>,
        options: SortableOptions,
    ) -> Result<SortableHandle, ()> {
        let handler = move |old_index: i64, new_index: i64| {
            callback.emit(SortingChange::new(
                old_index as Sorting,
                new_index as Sorting,
            ));
        };

        let handle = js!(
            const element = @{element};
            const options = {};
            options.handle = @{options.handle};
            options.forceFallback = @{options.force_fallback};
            options.delay = @{options.delay};

            options.onEnd = function (e) {
                setTimeout(() => {
                    const handler = @{handler};
                    handler(e.oldIndex, e.newIndex);
                }, 100);

                // create and dispatch the event
                const customEvent = new CustomEvent(@{SETLIST_CHANGE_SORTING}, {
                    detail: {
                        originalEvent: e,
                        oldIndex: e.oldIndex,
                        newIndex: e.newIndex,
                    }
                });
                element.dispatchEvent(customEvent);
            };
            const sortable = Sortable.create(element, options);
            console.debug("Initialize sortable", sortable);

            return sortable;
        );

        Ok(SortableHandle(Some(handle)))
    }
}

pub struct SortableOptions {
    pub delay: i32,
    pub handle: Option<String>,
    pub force_fallback: bool,
}

impl Default for SortableOptions {
    fn default() -> Self {
        SortableOptions {
            delay: 0,
            handle: None,
            force_fallback: false,
        }
    }
}
