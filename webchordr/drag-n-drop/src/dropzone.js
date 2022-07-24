const throttle = (callback, limit = 300) => {
    let waiting = false;

    return function () {
        if (!waiting) {
            callback.apply(this, arguments);
            waiting = true;
            setTimeout(function () {
                waiting = false;
            }, limit);
        }
    }
}

/**
 * @interface
 */
const Position = {
    x: 0.0,
    y: 0.0,
}

/**
 * @param {Position} position
 * @param {DOMRect} targetBox
 * @return {boolean}
 */
const isOverTarget = (position, targetBox) => {
    if (position.x < targetBox.x) {
        return false;
    }
    if (position.y < targetBox.y) {
        return false;
    }
    if (position.x > targetBox.x + targetBox.width) {
        return false;
    }
    if (position.y > targetBox.y + targetBox.height) {
        return false;
    }
    return true
}

export class DraggableItem {
    /**
     * @param {HTMLElement} element
     * @param {(e: TouchEvent, i: HTMLElement, p: Position)=>void} onTouchMove
     * @param {(e: TouchEvent, i: HTMLElement, p: Position)=>void} onTouchEnd
     * @param {(e: TouchEvent, i: HTMLElement, p: Position)=>void} onTouchCancel
     */
    constructor(element, onTouchMove, onTouchEnd, onTouchCancel) {
        this.handleTouchStart = this.handleTouchStart.bind(this)
        this.handleTouchMove = this.handleTouchMove.bind(this)
        this.handleTouchEnd = this.handleTouchEnd.bind(this)
        this.handleTouchCancel = this.handleTouchCancel.bind(this)
        this.element = element;
        this.onTouchMove = onTouchMove;
        this.onTouchEnd = onTouchEnd;
        this.onTouchCancel = onTouchCancel;
        this.position = {x: 0.0, y: 0.0}
        /**
         * Offset to attach the element at the right position under the finger
         * @type {{top: number, left: number}}
         */
        this.offset = {left: 0.0, top: 0.0}
        this.clone = null;
        this.originalDraggableAttribute = element.getAttribute('draggable')

        element.setAttribute('draggable', 'true')
        element.addEventListener('touchstart', this.handleTouchStart)
        element.addEventListener('touchmove', this.handleTouchMove)
        element.addEventListener('touchend', this.handleTouchEnd)
        element.addEventListener('touchcancel', this.handleTouchCancel)
    }

    destroy() {
        this.element.setAttribute('draggable', this.originalDraggableAttribute)
        this.element.removeEventListener('touchstart', this.handleTouchStart)
        this.element.removeEventListener('touchmove', this.handleTouchMove)
        this.element.removeEventListener('touchend', this.handleTouchEnd)
        this.element.removeEventListener('touchcancel', this.handleTouchCancel)
        this.removeClone()
    }

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleTouchStart(event) {
        const element = this.element;
        const initialLeft = element.getBoundingClientRect().left;
        const initialTop = element.getBoundingClientRect().top;
        const currentTouch = event.targetTouches[0]
        this.offset = {
            left: currentTouch.clientX - initialLeft,
            top: currentTouch.clientY - initialTop,
        }

        const clone = element.cloneNode(true)
        clone.style.position = 'fixed'
        clone.style.zIndex = '1000'
        clone.style.width = element.getBoundingClientRect().width + 'px'
        clone.style.height = element.getBoundingClientRect().height + 'px'
        clone.style.left = initialLeft + 'px'
        clone.style.top = initialTop + 'px'
        element.parentNode.insertBefore(clone, element)

        this.clone = clone;
    };

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleTouchMove(event) {
        event.preventDefault();
        const touchLocation = event.targetTouches[0];

        const offset = this.offset;
        const clone = this.clone;

        clone.style.left = (touchLocation.clientX - offset.left) + 'px'
        clone.style.top = (touchLocation.clientY - offset.top) + 'px'
        this.position = {
            x: touchLocation.clientX,
            y: touchLocation.clientY
        };

        (this.onTouchMove)(event, this.element, this.position)
    }

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleTouchEnd(event) {
        this.removeClone();
        (this.onTouchEnd)(event, this.element, this.position)
    }

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleTouchCancel(event) {
        this.removeClone();
        (this.onTouchCancel)(event, this.element, this.position)
    }

    /**
     * @private
     */
    removeClone() {
        if (this.clone.parentElement) {
            try {
                this.clone.parentElement.removeChild(this.clone)
            } catch (e) {
                console.error(e)
            }
        }
    }
}

/**
 * @param {string} itemSelector
 * @param {HTMLElement} item
 * @return {HTMLElement|undefined}
 * @private
 */
function getElementMatchingItemSelector(itemSelector, item) {
    if (item.matches(itemSelector)) {
        return item;
    }

    if (item.parentElement && item.parentElement.matches(itemSelector)) {
        return item.parentElement;
    }

    return undefined;
}

/**
 * @param {string[]} itemSelectors
 * @param {HTMLElement} item
 * @return {HTMLElement|undefined}
 * @private
 */
function getElementMatchingItemSelectors(itemSelectors, item) {
    for (const itemSelector of itemSelectors) {
        const element = getElementMatchingItemSelector(itemSelector, item);
        if (element) {
            return element
        }
    }

    return undefined
}

class DropzoneTouch {
    /**
     * @param {HTMLElement} target
     * @param {String[]} itemSelectors
     * @param {(songId:string)=>void} onDropOverTarget
     * @param {HTMLElement} [target]
     */
    constructor(target, itemSelectors, onDropOverTarget) {
        this.handleDocumentTouchStart = this.handleDocumentTouchStart.bind(this)
        this.handleDocumentTouchMove = this.handleDocumentTouchMove.bind(this)
        this.handleItemTouchMove = this.handleItemTouchMove.bind(this)
        this.handleTouchEnd = this.handleTouchEnd.bind(this)
        this.handleTouchCancel = this.handleTouchCancel.bind(this)
        this.itemSelectors = itemSelectors;
        this.draggableItem = undefined;
        /** @type {HTMLElement} */
        this.target = target;
        this.onDropOverTarget = onDropOverTarget;
        this.touchStartPosition = {x: 0, y: 0}

        document.addEventListener('touchstart', this.handleDocumentTouchStart)
        document.addEventListener('touchmove', this.handleDocumentTouchMove)
    }

    destroy() {
        if (this.draggableItem) {
            this.draggableItem.destroy()
        }
        document.removeEventListener('touchstart', this.handleDocumentTouchStart)
        document.removeEventListener('touchmove', this.handleDocumentTouchMove)
    }

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleDocumentTouchStart(event) {
        const item = this.getElementMatchingItemSelectors(event.target);
        if (!item) {
            return;
        }

        // event.preventDefault();
        const touchLocation = event.targetTouches[0];

        /* Store the current touch location and start time for later comparison */
        this.touchStartPosition = {
            x: touchLocation.clientX,
            y: touchLocation.clientY
        };
        this.touchStartTime = (new Date()).valueOf()
    }

    /**
     * @param {TouchEvent} event
     * @private
     */
    handleDocumentTouchMove(event) {
        if (this.isDragging) {
            return
        }

        const touchLocation = event.targetTouches[0];
        if (!this.isTouchDrag(this.touchStartTime, this.touchStartPosition, touchLocation)) {
            return
        }

        const item = this.getElementMatchingItemSelectors(event.target);
        if (item) {
            /* Start handling the current interaction as a drag */
            this.draggableItem = new DraggableItem(
                item,
                throttle(this.handleItemTouchMove),
                this.handleTouchEnd,
                this.handleTouchCancel
            )

            this.draggableItem.handleTouchStart(event);
            this.isDragging = true
        }
    }

    /**
     * Measure the touch location distance after a threshold of 100ms
     *
     * @param {number} touchStartTime
     * @param {Position} startTouchPosition
     * @param {Touch} currentTouchLocation
     * @return {boolean}
     */
    isTouchDrag(touchStartTime, startTouchPosition, currentTouchLocation) {
        const millisecondThreshold = 100;
        const positionThreshold = 10;
        const sampleIntervalDidPass = (new Date()).valueOf() - millisecondThreshold > touchStartTime;
        if (!sampleIntervalDidPass) {
            return false;
        }

        return Math.abs(currentTouchLocation.clientX - startTouchPosition.x) > positionThreshold
            || Math.abs(currentTouchLocation.clientY - startTouchPosition.y) > positionThreshold;
    }

    /**
     * @param {HTMLElement} item
     * @return {HTMLElement|undefined}
     * @private
     */
    getElementMatchingItemSelectors(item) {
        return getElementMatchingItemSelectors(this.itemSelectors, item);
    }

    /**
     * Handle the touch move of the item
     *
     * @param {TouchEvent} event
     * @param {HTMLElement} item
     * @param {Position} position
     * @private
     */
    handleItemTouchMove(event, item, position) {
        const targetBox = this.target.getBoundingClientRect();

        if (isOverTarget(position, targetBox)) {
            this.target.classList.add('-touch-hover');
        } else {
            this.target.classList.remove('-touch-hover');
        }
    }

    /**
     * @param {TouchEvent} event
     * @param {HTMLElement} item
     * @param {Position} position
     * @private
     */
    handleTouchEnd(event, item, position) {
        this.cleanup();
        const targetBox = this.target.getBoundingClientRect();
        if (isOverTarget(position, targetBox)) {
            const onDropArgument = {dataset: item.dataset};
            (this.onDropOverTarget)(onDropArgument);
        }
    }

    /**
     * @param {TouchEvent} event
     * @param {HTMLElement} item
     * @param {Position} position
     * @private
     */
    handleTouchCancel(event, item, position) {
        this.cleanup();
    }

    /**
     * @private
     */
    cleanup() {
        this.isDragging = false;
        this.target.classList.remove('-touch-hover');
        this.draggableItem.destroy();
        this.draggableItem = undefined;
    }
}

export class Dropzone {
    /**
     * @param {HTMLElement} target
     * @param {String|String[]} itemSelectors An array of CSS-selector strings or a comma separated list of CSS-selectors
     * @param {(songId:string)=>void} onDropOverTarget
     * @param {HTMLElement} [target]
     */
    constructor(target, itemSelectors, onDropOverTarget) {
        const preparedItemSelectors = Array.isArray(itemSelectors) ? itemSelectors : ('' + itemSelectors).split(',');
        if ('ontouchstart' in window) {
            this.handler = new DropzoneTouch(target, preparedItemSelectors, onDropOverTarget);
        } else if ('ondragstart' in window) {
            buildOutput(true, 'dropzone').warn('Native drag\'n drop not implemented');
            // this.handler = new DropzoneNative(target, preparedItemSelectors, onDropOverTarget);
        } else {
            buildOutput(true, 'dropzone').warn('No matching drag\'n drop implementation available');
        }
    }

    destroy() {
        if (this.handler) {
            this.handler.destroy()
        }
    }
}

export {Dropzone as DropzoneWrapper};
