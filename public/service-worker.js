const CACHE_PREFIX = "rosary-shell-";
const CACHE_NAME = `${CACHE_PREFIX}v2`;
const APP_BASE = new URL("./", self.location.href);
const appUrl = (path) => new URL(path, APP_BASE).href;
const PKG_PATH = new URL("pkg/", APP_BASE).pathname;
const ICONS_PATH = new URL("icons/", APP_BASE).pathname;
const APP_SHELL = [
  "./",
  "index.html",
  "manifest.webmanifest",
  "favicon.ico",
  "favicon.png",
  "icons/rosary-192.png",
  "icons/rosary-512.png",
  "icons/rosary-maskable-192.png",
  "icons/rosary-maskable-512.png",
  "pkg/rosary.css",
  "pkg/rosary.js",
  "pkg/rosary.wasm",
].map(appUrl);

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches
      .open(CACHE_NAME)
      .then((cache) => cache.addAll(APP_SHELL))
      .then(() => self.skipWaiting()),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key.startsWith(CACHE_PREFIX) && key !== CACHE_NAME)
            .map((key) => caches.delete(key)),
        ),
      )
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const { request } = event;
  if (request.method !== "GET") {
    return;
  }

  const url = new URL(request.url);
  if (url.origin !== self.location.origin) {
    return;
  }

  if (request.mode === "navigate") {
    event.respondWith(
      fetch(request)
        .then((response) => {
          if (response.ok) {
            const copy = response.clone();
            return caches
              .open(CACHE_NAME)
              .then((cache) => cache.put(appUrl("index.html"), copy))
              .then(() => response);
          }
          return response;
        })
        .catch(() => caches.match(appUrl("index.html"))),
    );
    return;
  }

  const isStaticAsset =
    url.pathname.startsWith(PKG_PATH) ||
    url.pathname.startsWith(ICONS_PATH) ||
    url.href === appUrl("manifest.webmanifest") ||
    url.href === appUrl("favicon.ico") ||
    url.href === appUrl("favicon.png");

  if (!isStaticAsset) {
    return;
  }

  event.respondWith(
    caches.match(request).then((cached) => {
      if (cached) {
        return cached;
      }

      return fetch(request).then((response) => {
        if (response.ok && response.type === "basic") {
          const copy = response.clone();
          return caches
            .open(CACHE_NAME)
            .then((cache) => cache.put(request, copy))
            .then(() => response);
        }
        return response;
      });
    }),
  );
});
