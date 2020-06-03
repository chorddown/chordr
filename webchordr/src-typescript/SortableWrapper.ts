import Sortable, {SortableEvent} from 'sortablejs' ;

const SETLIST_CHANGE_SORTING = "chordr:setlist-change-sorting";

type Options = Sortable.Options;

export class SortableWrapper {
    private sortable: Sortable | undefined;

    constructor(
        element: HTMLElement,
        callback: (oldIndex: number | undefined, newIndex: number | undefined) => void,
        options?: Options
    ) {
        console.log('[SortableWrapper] New', element, options)
        options = options || {};

        options.onEnd = function (e: SortableEvent) {
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
            this.sortable = undefined;
        } else {
            console.debug("[SortableWrapper] Already destroyed");
        }
    }
}
