# Rosary Guide

A responsive, bilingual Rosary guide built with [Leptos](https://leptos.dev/). The original standalone `index.html` is retained as the design reference; the running application lives in the Rust workspace.

## Structure

- `app/src/components.rs` — reusable page and content components
- `app/src/i18n.rs` — typed Italian and English translations
- `app/src/lib.rs` — application composition and language state
- `style/main.scss` — responsive visual design
- `frontend` — WebAssembly entry point
- `server` — static Axum server used by `cargo-leptos`

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

## Checks

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo leptos build
```

The optional Playwright test in `end2end/tests/example.spec.ts` checks the initial Italian rendering and the English language switch.
