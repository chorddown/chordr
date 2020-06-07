const CACHE_NAME = 'chordr';

/* See also webchordr/static/index.html:36 */
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
        debug: {
            text: "%cDEBUG%c webchordr%c ",
            style: "color: white; padding: 0 3px; background: #0066ff;"
        },
    }
}

self.addEventListener('install', function (e) {
    e.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll([
                '/',
                '/index.html',
                '/assets/fonts/libre-baskerville-v7-latin_latin-ext-regular.woff2',
                '/assets/fonts/libre-baskerville-v7-latin_latin-ext-italic.woff2',
                '/assets/fonts/merriweather-v21-latin-regular.woff2',
                '/assets/fonts/merriweather-v21-latin-700.woff2',
                '/assets/icons/fonts/iconmonstr-iconic-font.woff2',
                '/stylesheets/chordr-default-styles.css',
                '/stylesheets/chordr-app.css',
                '/webchordr.js',
                '/webchordr.wasm',
                '/setup.js',
                '/assets/libraries/clipboard.min.js',
                '/assets/libraries/Sortable.min.js',
                '/catalog.json'
            ]);
        })
    );
});

const fetchFromServer = function (event) {
    return fetch(event.request).then(
        function (response) {
            if (!response || response.status !== 200 || response.type !== 'basic') {
                return response;
            }

            /* Stash copy of response */
            const cachedResponse = response.clone();
            caches.open(CACHE_NAME)
                .then(function (cache) {
                    cache.put(event.request, cachedResponse).then();
                });

            return response;
        }
    );
};

self.addEventListener('fetch', function (event) {
    event.respondWith(
        caches.match(event.request)
            .then(function (response) {
                /* Check if there is a cached entry for the request */
                if (!response) {
                    const message = 'Fetch ' + event.request.url + ' from server';
                    console.info(
                        consoleStyles.label.info.text + message,
                        consoleStyles.label.info.style,
                        consoleStyles.pathStyle,
                        consoleStyles.normalStyle
                    )

                    return fetchFromServer(event);
                } else {
                    /* If online try to fetch the latest version in the background */
                    if (navigator.onLine) {
                        fetchFromServer(event).then(function () {
                            const message = 'Did load ' + event.request.url + ' in background';
                            console.info(
                                consoleStyles.label.info.text + message,
                                consoleStyles.label.info.style,
                                consoleStyles.pathStyle,
                                consoleStyles.normalStyle
                            )
                        }).catch(function (e) {
                            const message = 'Failed loading ' + event.request.url + ' in background';
                            console.error(
                                consoleStyles.label.error.text + message,
                                consoleStyles.label.error.style,
                                consoleStyles.pathStyle,
                                consoleStyles.normalStyle,
                                e
                            )
                        });
                    }
                    return response;
                }
            })
//         fetch(event.request).catch(function () {
//             return caches.match(event.request);
//         })
    );
    // let catalogRequest = /\/catalog\.json/.test(event.request.url);
    // if (catalogRequest) {
    //     console.log('[SW] Try to fetch new catalog and fall back to cache', event.request.url);
    //     event.respondWith(
    //         fetch(event.request).catch(function () {
    //             return caches.match(event.request);
    //         })
    //     );
    // } else {
    //     console.log('[SW] Query cache and fall back to fetch', event.request.url);
    //
    //     event.respondWith(
    //         caches.match(event.request).then(function (response) {
    //             return response || fetch(event.request);
    //         })
    //     );
    // }
});

// self.addEventListener('activate', function (event) {
//     event.waitUntil(
//         caches.keys().then(function (cacheNames) {
//             return Promise.all(
//                 cacheNames.filter(function (cacheName) {
//                     // Return true if you want to remove this cache,
//                     // but remember that caches are shared across
//                     // the whole origin
//                     // return true;
//                 }).map(function (cacheName) {
//                     return caches.delete(cacheName);
//                 })
//             );
//         })
//     );
// });
