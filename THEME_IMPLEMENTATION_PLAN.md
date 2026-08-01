# Dark and Light Theme Implementation Plan

## Goal

Add a user-selectable dark/light theme to the Rosary guide while preserving
the existing visual identity, responsive layout, bilingual UI, accessibility,
and SVG diagram legibility.

The theme must:

- Start from an explicitly saved user choice when one exists.
- Otherwise follow the browser's `prefers-color-scheme` setting.
- Allow switching from the header without a page reload.
- Persist the user's choice in `localStorage`.
- Apply the theme on the root `<html>` element through
  `data-theme="dark"` or `data-theme="light"`.
- Avoid hard-coded theme-specific colors in component or SVG styles.
- Keep all new user-visible labels available in Italian and English.

## Proposed design

Use a small Rust theme domain module and CSS custom-property tokens:

```text
Theme enum
   ↓
App owns RwSignal<Theme>
   ↓
Header renders the toggle and persists changes
   ↓
<html data-theme="dark|light">
   ↓
style/main.scss resolves semantic color tokens
```

`Theme` should represent the effective display mode only (`Dark` or `Light`).
The absence of a saved preference is an initialization concern: resolve it
from `localStorage`, then `matchMedia("(prefers-color-scheme: light)")`, and
finally fall back to dark.

## Commit-oriented task plan

Each task below is independently reviewable and should be committed only after
its completion criteria pass. Do not combine unrelated tasks into one commit.

### Task 1 — Define the theme domain and browser boundary

**Commit:** `feat(theme): add documented theme model`

**Files:**

- Add `app/src/theme.rs`.
- Update `app/src/lib.rs` to register `mod theme;`.

**Symbols and behavior:**

- Add `pub(crate) enum Theme { Dark, Light }` with `Clone`, `Copy`,
  `Debug`, `PartialEq`, and `Eq` derives.
- Add `Theme::as_attribute(self) -> &'static str` returning `"dark"` or
  `"light"`.
- Add `Theme::toggle(self) -> Self`.
- Add a documented constant for the storage key, for example
  `rosary-theme`.
- Add pure parsing helpers such as
  `Theme::from_attribute(value: &str) -> Option<Self>` so behavior can be
  tested without a browser.
- Keep all `web_sys` access out of the pure enum methods.

**Rust documentation requirements:**

- Start the module with `//!` explaining that it models the effective UI
  theme and separates pure theme behavior from browser persistence.
- Add `///` documentation to `Theme`, its public(crate) methods, and the
  storage-key constant.
- Explain in a short comment why the theme is stored as an HTML attribute
  rather than a class.

**Completion criteria:**

- The module compiles on host and `wasm32-unknown-unknown`.
- Unit tests cover attribute parsing and toggling.
- No UI behavior changes yet.

### Task 2 — Add theme resolution, persistence, and root application

**Commit:** `feat(theme): persist and apply browser theme`

**Files:**

- Extend `app/src/theme.rs`.
- Update `app/src/lib.rs` in `App`.

**Symbols and behavior:**

- Add a browser-only `Theme::from_browser() -> Self` boundary that:
  1. Reads `window.localStorage` using the storage key.
  2. Parses only `dark` and `light`; ignores invalid values.
  3. Checks `window.matchMedia("(prefers-color-scheme: light)")` when no
     valid saved value exists.
  4. Falls back to `Theme::Dark` if browser APIs are unavailable.
- Add a documented helper such as
  `apply_theme(theme: Theme, document: &Document)` that sets
  `document.document_element().set_attribute("data-theme", ...)`.
- Add a documented helper to persist the effective theme. Storage failures
  must be ignored gracefully; private browsing and blocked storage must not
  break rendering.
- In `App`, create `let theme = RwSignal::new(Theme::from_browser())`.
- Add an `Effect` that applies the signal value to the root document element
  and persists user changes.
- Pass `theme` to `Header`.

**Hydration/flash handling:**

- Confirm whether this application is fully client-mounted or SSR-hydrated.
- If the browser shows a visible first-paint flash, add a tiny inline theme
  bootstrap in `server/src/main.rs::shell` before the stylesheet is applied.
  The bootstrap may only read the storage key, inspect the media query, and
  set `data-theme`; it must not duplicate calendar or application logic.
- Document this decision in a Rust `///` comment beside the shell bootstrap.

**Completion criteria:**

- Reloading after a manual choice preserves the choice.
- Clearing storage follows the OS/browser preference.
- Invalid storage values fall back safely.
- Theme changes update `<html data-theme>` without reload.
- Host and WASM checks pass.

### Task 3 — Add bilingual theme controls to the header

**Commit:** `feat(theme): add accessible bilingual theme toggle`

**Files:**

- Update `app/src/i18n.rs`.
- Update `app/src/components/header.rs`.
- Update `app/src/lib.rs` only if the `Header` call needs the new signal.

**Translation fields:**

Add fields to `Translation` and both `IT`/`EN` constants for:

- Theme control accessible label.
- Dark-mode state label.
- Light-mode state label.

Use complete user-facing strings rather than relying on icon meaning. For
example, the control can expose `Passa al tema chiaro` / `Switch to light theme`
when the current theme is dark, and the inverse when it is light.

**Header symbols and markup:**

- Change `Header` to accept `theme: RwSignal<Theme>`.
- Keep the existing language `<select>` behavior unchanged.
- Add a semantic `<button type="button">` beside the language picker.
- Set `aria-label` reactively from the current theme and translation.
- Set `aria-pressed` to reflect whether light mode is active, or use an
  equivalent explicit state model consistently.
- Toggle only through `Theme::toggle`; do not duplicate theme strings or
  browser persistence in the component.
- Preserve keyboard operation and the existing skip-link behavior.

**Rust documentation requirements:**

- Add `///` documentation to any new component and any changed public(crate)
  prop contract.
- Add a concise comment explaining that the button changes the effective
  theme while `theme.rs` owns persistence and document synchronization.

**Completion criteria:**

- The control is reachable and operable with keyboard only.
- Its label and state update when the language or theme changes.
- Italian and English render the correct action text.
- The header remains usable at the existing mobile breakpoints.

### Task 4 — Convert dark colors into semantic design tokens

**Commit:** `refactor(theme): centralize semantic color tokens`

**Files:**

- Update `style/main.scss`.

**Token work:**

Keep the current dark appearance as the dark-theme baseline, but replace
theme-specific literals with semantic variables. At minimum, define tokens
for:

- Page background and secondary background.
- Panel/surface background.
- Primary and dim text.
- Gold accent, bright gold, and dim gold.
- Header gradient endpoints.
- Guide radial-gradient glow.
- Surface border and translucent surface fill.
- Form-control background and focus outline.
- Diagram chain, diagram copy, art-gradient endpoints, and footer text.
- Shadow/glow colors where the alpha value is part of the theme.

Update every affected selector, including `.site-header`, `.language-picker
select`, `.guide`, `.creed-box`, `.guide-box`, `.chain`, `.diagram-title`,
`.diagram-copy`, `.mystery-art`, `.footer`, focus states, and any remaining
hard-coded dark colors. Preserve non-theme geometry, typography, and spacing.

**Completion criteria:**

- Searching the stylesheet for the old theme-specific literals finds no
  unintended UI color usage.
- Dark mode is visually unchanged apart from the new control.
- All existing responsive selectors still apply.

### Task 5 — Add and tune the light theme token set

**Commit:** `feat(theme): add accessible light color palette`

**Files:**

- Update `style/main.scss`.

**Implementation:**

- Add `[data-theme="light"]` overrides for every semantic token from Task 4.
- Use warm paper/cream surfaces consistent with the Rosary design rather
  than pure white everywhere.
- Use dark text with verified contrast on every light surface.
- Re-tune gold accent values where the dark-mode gold does not contrast
  sufficiently against light backgrounds.
- Ensure gradients, SVG chain lines, bead fills, diagram labels, borders,
  and footer text remain visible in light mode.
- Set `color-scheme` consistently on the root/theme selectors so native
  controls render appropriately.
- Add `:focus-visible` styles for the theme button, language select, and any
  interactive control. The focus indicator must be visible in both themes.
- Do not use a blanket `filter`, `invert`, or opacity wash over the entire
  application.

**Completion criteria:**

- Both themes are legible at desktop and mobile widths.
- Text, controls, borders, diagram labels, and decorative SVG elements remain
  distinguishable in both themes.
- Contrast is checked for body text, dim text, button text, selected controls,
  and focus indicators.

### Task 6 — Add automated theme and accessibility coverage

**Commit:** `test(theme): cover persistence, switching, and contrast surfaces`

**Files:**

- Extend `app/src/theme.rs` unit tests.
- Update `end2end/tests/example.spec.ts` or add
  `end2end/tests/theme.spec.ts`.
- Update `end2end/playwright.config.ts` only if a browser/project setting is
  needed.

**Unit tests:**

- `dark` and `light` attribute parsing.
- Invalid and empty values.
- Toggle behavior.
- Storage/media-query resolution through testable pure boundaries where
  possible; keep browser API mocking isolated to the browser tests.

**Playwright tests:**

- Default rendering has a valid `data-theme` attribute.
- Clicking the theme button changes `data-theme` and the visible control
  state without navigation.
- Reloading preserves the selected theme.
- Clearing the preference and emulating light/dark system preference selects
  the expected initial theme.
- Theme switching does not break the Italian-to-English language switch.
- The theme button has an accessible name and is keyboard operable.
- Check the key surfaces in both modes: header, prayer panel, guide boxes,
  rosary SVG, mystery cards, and footer.

**Completion criteria:**

- Rust unit tests and Playwright tests pass.
- No console errors occur during theme initialization or switching.
- Tests do not depend on the current calendar date.

### Task 7 — Document the feature and finish the handoff

**Commit:** `docs(theme): document theme architecture and QA`

**Files:**

- Update `AGENTS.md` with the theme module, signal flow, token policy, and
  test locations.
- Update `README.md` with the user-facing behavior and the storage key only
  if that implementation detail is useful to contributors.
- Keep this plan as the design/implementation record, adding links to the
  final implementation files after completion.

**Rust documentation requirements:**

- Verify every new public(crate) theme type, function, component, and module
  has explanatory `//!`/`///` documentation.
- Document browser-only fallbacks and why storage/media-query failures are
  non-fatal.

**Completion criteria:**

- A new contributor can locate theme state, browser synchronization, CSS
  tokens, translations, and tests from `AGENTS.md`.
- `cargo fmt --all -- --check`, `cargo check --workspace`, the WASM check,
  and the full relevant Playwright suite pass.
- Each task above is represented by a focused commit with a clear message.

## Recommended execution order

Complete tasks strictly in order. Tasks 1–2 establish behavior, Task 3 adds
the user control, Tasks 4–5 change visual output, Task 6 protects the feature,
and Task 7 records the final architecture. If a task exposes a design change,
update this plan before starting the next commit rather than silently folding
scope into an existing task.

## Explicit non-goals

- Do not add a third user-facing “system” mode unless product requirements
  change; system preference is only the initial fallback.
- Do not redesign the existing typography, layout, rosary geometry, or mystery
  content as part of the theme work.
- Do not persist language and theme under one combined preference object;
  keep the existing language signal independent from the theme key.

## Final implementation

- Theme model and browser boundary: [`app/src/theme.rs`](app/src/theme.rs)
- Application state and document synchronization: [`app/src/lib.rs`](app/src/lib.rs)
- Bilingual accessible control: [`app/src/components/header.rs`](app/src/components/header.rs)
- Italian and English labels: [`app/src/i18n.rs`](app/src/i18n.rs)
- Pre-stylesheet bootstrap: [`server/src/main.rs`](server/src/main.rs)
- Dark/light semantic tokens: [`style/main.scss`](style/main.scss)
- Browser coverage: [`end2end/tests/theme.spec.ts`](end2end/tests/theme.spec.ts)
