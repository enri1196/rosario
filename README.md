# Rosary Guide

A responsive, bilingual Rosary guide built with [Leptos](https://leptos.dev/). The original standalone `index.html` is retained as the design reference; the running application lives in the Rust workspace. The installable app keeps its core prayer guide available offline after the first successful visit.

## Structure

- `app/src/components.rs` — reusable page and content components
- `app/src/i18n.rs` — typed Italian and English translations
- `app/src/lib.rs` — application composition, language state, and theme state
- `app/src/theme.rs` — theme resolution, root synchronization, and persistence
- `style/main.scss` — responsive visual design and semantic theme tokens
- `frontend` — WebAssembly entry point
- `server` — static Axum server used by `cargo-leptos`
- `public` — source-owned favicon, install manifest, PWA icons, and versioned service worker copied into the generated site root by `cargo-leptos`

## Run locally

Install `cargo-leptos` and the WebAssembly target once:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
```

Then start the development server:

```bash
cargo leptos watch
```

Open <http://127.0.0.1:3000>.

## Install and offline use

On a supported browser, install the Rosary Guide from the browser's app menu after opening it online once. The service worker caches only the application shell and same-origin static assets; private prayer intentions remain in browser-local storage and are never added to URLs or network requests.

Whenever the application shell changes, increment the cache version in `public/service-worker.js`. Activation removes older Rosary shell caches so deployments cannot retain them indefinitely.

## Themes

The header’s sun/moon button switches between the dark and light themes
without reloading. An explicit choice is stored under `rosary-theme`; when no
choice is saved, the initial theme follows `prefers-color-scheme` and safely
falls back to dark if browser APIs are unavailable. The effective mode is
always exposed as `data-theme="dark"` or `data-theme="light"` on `<html>`.

## Checks

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo check --workspace
cargo leptos build
```

The Playwright tests in `end2end/tests` cover the initial bilingual rendering,
language switching, theme initialization, persistence, keyboard operation,
responsive control alignment, install metadata, production service-worker
registration, offline reloads, and the key visual surfaces in both themes.
