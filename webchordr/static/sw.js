self.addEventListener('install', function (e) {
    e.waitUntil(
        caches.open('chordr').then(function (cache) {
            return cache.addAll([
                '/',
                '/index.html',
                '/fonts/libre-baskerville-v7-latin_latin-ext-regular.woff2',
                '/stylesheets/chordr-default-styles.css',
                '/stylesheets/chordr-app.css',
                '/webchordr.js',
                '/webchordr.wasm',
                '/catalog.json'
            ]);
        })
    );
});

self.addEventListener('fetch', function (event) {
    console.log(event.request.url);

    event.respondWith(
        caches.match(event.request).then(function (response) {
            return response || fetch(event.request);
        })
    );
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
