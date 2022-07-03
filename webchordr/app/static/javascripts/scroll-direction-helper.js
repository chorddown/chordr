const threshold = 5;
const detachThreshold = 50;
const classList = document.documentElement.classList;

let lastScrollPosition = 0.0;
let lastScrollDirection = undefined;
let isAttached = false;

/* Initialize */
export function initialize() {
    document.addEventListener('DOMContentLoaded', function () {
        addScrollClasses();
        window.addEventListener("scroll", throttle(addScrollClasses), {passive: true});
    });
}

function addScrollClasses() {
    const currentScrollPosition = window.scrollY;
    checkIfAttached(currentScrollPosition);
    checkDirection(currentScrollPosition);
}

function checkIfAttached(currentScrollPosition) {
    if (currentScrollPosition < detachThreshold) {
        if (!isAttached) {
            classList.add('attached')
            classList.remove('detached')
            isAttached = true
        }
    } else {
        if (isAttached) {
            classList.remove('attached')
            classList.add('detached')
            isAttached = false
        }
    }
}

function checkDirection(currentScrollPosition) {
    let scrollDirection = '';
    const diff = lastScrollPosition - currentScrollPosition;
    if (Math.abs(diff) > threshold) {
        if (diff > 0) {
            scrollDirection = 'direction-up';
        } else {
            scrollDirection = 'direction-down';
        }

        if (lastScrollDirection !== scrollDirection) {
            classList.remove(lastScrollDirection)
            classList.add(scrollDirection)
            lastScrollDirection = scrollDirection;
        }
    }
    lastScrollPosition = currentScrollPosition
}

function throttle(callback, limit) {
    let waiting = false;
    return function () {
        if (!waiting) {
            callback.apply(this, arguments);
            waiting = true;
            setTimeout(function () {
                waiting = false;
            }, limit || 20);
        }
    }
}
