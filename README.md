# Rosary Guide

A responsive, bilingual Rosary guide built with [Leptos](https://leptos.dev/). The original standalone `index.html` is retained as the design reference; the running application lives in the Rust workspace. The installable app keeps its core prayer guide available offline after the first successful visit.

## Structure

- `app/src/components/` — reusable page and content components
- `app/src/i18n.rs` — typed Italian and English translations
- `app/src/lib.rs` — application composition, language state, and theme state
- `app/src/theme.rs` — theme resolution, root synchronization, and persistence
- `style/main.scss` — responsive visual design and semantic theme tokens
- `frontend` — WebAssembly entry point
- `server` — static Axum server used by `cargo-leptos`
- `public` — source-owned favicon, install manifest, PWA icons, and versioned service worker copied into the generated site root by `cargo-leptos`
- `.github/workflows/pages.yml` — production build and GitHub Pages deployment

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

## Release on GitHub Pages

In the repository settings on GitHub, open **Pages** and select **GitHub Actions**
as the build source. Pushes to `master` then build and deploy the static site;
the workflow can also be started manually from the Actions tab.

This repository may remain private when the owner has GitHub Pro. For a
personal-account repository, the deployed Pages site is still public; private
Pages access requires an organization on GitHub Enterprise Cloud.

The release workflow uses GitHub's reported Pages base path, so the same build
supports the project URL at <https://enri1196.github.io/rosario/> and a future
custom domain. `cargo leptos build --release` remains the only asset build, and
the server's `--generate-site` mode writes the production `index.html` without
starting Axum.

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
