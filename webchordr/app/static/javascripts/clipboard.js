function copy() {
    try {
        return document.execCommand('copy');
    } catch (err) {
        return false;
    }
}

function select(target) {
    if (typeof target.select === 'function') {
        target.select();
        return true;
    } else {
        return false;
    }
}

/**
 * @param {HTMLElement} element
 * @param {number} triesLeft
 * @returns {null|Element}
 */
function getClipboardButtonTarget(element, triesLeft) {
    if (typeof element.dataset.clipboardTarget === 'string') {
        try {
            return document.querySelector(element.dataset.clipboardTarget)
        } catch (e) {
            return null;
        }
    }

    if (triesLeft > 0 && element.parentElement) {
        return getClipboardButtonTarget(element.parentElement, triesLeft - 1)
    } else {
        return null;
    }
}

/* Initialize */
export function initialize() {
    document.addEventListener('click', function (e) {
        const target = getClipboardButtonTarget(e.target, 4);
        if (target) {
            try {
                if (select(target)) {
                    copy();
                }
            } catch (e) {
            }
        }
    });
}
