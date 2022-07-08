function patchClick(event) {
    if (!event.target) {
        return;
    }

    const target = event.target;
    if (target instanceof HTMLAnchorElement) {
        console.log('is link', target)
        if (target.getAttribute('role') === 'button') {
            console.log('is role button', target)
            event.preventDefault();
            window.history.pushState({}, '', target.href);
        }
    }
    event.preventDefault();
}

export function initialize() {
    document.addEventListener('click', patchClick)
}
