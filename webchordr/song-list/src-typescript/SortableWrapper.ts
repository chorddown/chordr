import Sortable, {SortableEvent} from 'sortablejs' ;

const SETLIST_CHANGE_SORTING = "chordr:setlist-change-sorting";

type Options = Sortable.Options;

const consoleStyles = {
    normalStyle: "background: inherit; color: inherit",
    pathStyle: "font-weight: bold; color: inherit",
    label: {
        info: {
            text: "%cINFO%c webchordr%c ",
            style: "color: white; padding: 0 3px; background: #029202;"
        },
        error: {
            text: "%cERROR%c webchordr%c ",
            style: "color: white; padding: 0 3px; background: #ff2863;"
        },
        warn: {
            text: "%cWARN%c webchordr%c ",
            style: "color: white; padding: 0 3px; background: #c18d12;"
        },
        debug: {
            text: "%cDEBUG%c webchordr%c ",
            style: "color: white; padding: 0 3px; background: #0066ff;"
        },
    }
}

export class SortableWrapper {
    private sortable: Sortable | undefined;

    constructor(
        element: HTMLElement,
        callback: (oldIndex: number | undefined, newIndex: number | undefined) => void,
        options?: Options
    ) {
        console.info(
            consoleStyles.label.info.text + '[SortableWrapper] New',
            consoleStyles.label.info.style,
            consoleStyles.pathStyle,
            consoleStyles.normalStyle
        )
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
        console.debug(
            consoleStyles.label.debug.text + '[SortableWrapper] Initialized sortable',
            consoleStyles.label.debug.style,
            consoleStyles.pathStyle,
            consoleStyles.normalStyle
        )
    }

    destroy() {
        if (this.sortable) {
            console.debug(
                consoleStyles.label.debug.text + '[SortableWrapper] Destroy',
                consoleStyles.label.debug.style,
                consoleStyles.pathStyle,
                consoleStyles.normalStyle
            );

            this.sortable.destroy();
            this.sortable = undefined;
        } else {
            console.debug(
                consoleStyles.label.debug.text + '[SortableWrapper] Already destroyed',
                consoleStyles.label.debug.style,
                consoleStyles.pathStyle,
                consoleStyles.normalStyle
            );
        }
    }
}
