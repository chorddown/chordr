import Sortable from 'sortablejs';

const SETLIST_CHANGE_SORTING = "chordr:setlist-change-sorting";

export class SortableWrapper {
    constructor(element, callback, options) {
        console.log('[SortableWrapper] New', element, options)
        options = options || {};

        options.onEnd = function (e) {
            setTimeout(() => {
                callback(e.oldIndex, e.newIndex);
            }, 100);

            // create and dispatch the event
            const customEvent = new CustomEvent(SETLIST_CHANGE_SORTING, {
                detail: {
                    originalEvent: e,
                    oldIndex: e.oldIndex,
                    newIndex: e.newIndex,
                }
            });

            element.dispatchEvent(customEvent);
        };
        this.sortable = Sortable.create(element, options);
        console.debug("[SortableWrapper] Initialized sortable", this.sortable);
    }

    destroy() {
        if (this.sortable) {
            console.debug("[SortableWrapper] Destroy", this.sortable);

            this.sortable.destroy();
            this.sortable = null;
        } else {
            console.debug("[SortableWrapper] Already destroyed");
        }
    }
}
