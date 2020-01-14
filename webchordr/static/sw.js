const CACHE_NAME = 'chordr';
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

            // Stash copy of response
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
                if (!response) {
                    // If not, fetch request, and then cache response
                    return fetchFromServer(event);
                } else {
                    if (navigator.onLine) {
                        fetchFromServer(event).then(function () {
                            console.info('INFO:webchordr -- Did load ' + event.request.url + ' in background');
                        }).catch(function (e) {
                            console.error('ERROR:webchordr -- Failed loading ' + event.request.url + ' in background', e);
                        });
                    }
                    // If already cached
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

self.addEventListener('activate', function (event) {
    event.waitUntil(
        caches.keys().then(function (cacheNames) {
            return Promise.all(
                cacheNames.filter(function (cacheName) {
                    // Return true if you want to remove this cache,
                    // but remember that caches are shared across
                    // the whole origin
                    // return true;
                }).map(function (cacheName) {
                    return caches.delete(cacheName);
                })
            );
        })
    );
});
