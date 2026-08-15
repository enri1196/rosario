# Guided Rosary UX Redesign Plan

Status: implemented and verified (2026-08-15)

## Objective

Make the guided Rosary feel calm, predictable, and easy to operate while praying:

- preserve the existing shared `RosarySession` state machine and 31-step order;
- keep prayer text centered without layout jumps between steps;
- use side navigation on larger screens and reachable labeled controls on phones;
- make progress meaningful at both the full-Rosary and decade levels;
- keep restart, close, completion, keyboard, focus, bilingual, and reduced-motion behavior accessible.

## Current baseline

The working tree already contains the first layout pass:

- `GuidedPrayer` renders centered step content in `.guided-step-panel-content`;
- `.guided-step-layout` reserves a stable responsive region;
- previous/next controls sit at the sides on larger layouts;
- the header contains a restart icon beside close;
- long content can scroll inside the step panel;
- the guided Playwright spec checks centered text and vertically aligned controls.

This plan refines that pass and adds the remaining UX behavior. It does not replace the session model or introduce a second navigation state.

## Scope

### In scope

- responsive navigation controls;
- stable prayer-content sizing and scroll-position behavior;
- translated decade-level progress;
- consistent inline SVG controls;
- completion-state call to action;
- restrained step transitions with reduced-motion support;
- keyboard, focus, screen-reader, and browser coverage.

### Explicitly excluded

- audio prayers;
- shareable prayer links;
- custom mystery schedules;
- changing the 31-step Rosary sequence;
- persistence or synchronization of guided-session progress;
- changes to the rosary diagram geometry or the general application theme.

## Design contract

The implementation must preserve these invariants:

1. `App` remains the owner of the single `RwSignal<Option<RosarySession>>`.
2. `RosarySession::previous`, `next`, and `reset` remain the only navigation mutations.
3. Previous is disabled on the first step; next completes the session from the final step.
4. Restart always returns to the opening step while preserving the selected mystery set and focused mystery.
5. Every visible string is supplied by `Translation` in both Italian and English.
6. Icon-only controls have translated `aria-label` and `title` values; visible mobile labels are supplementary.
7. Focus moves to the new step heading after navigation and restart.
8. Long prayers remain readable and are never clipped; any internal scrolling is keyboard- and touch-accessible.
9. Motion is disabled or minimized under `prefers-reduced-motion: reduce`.
10. The session remains usable at narrow mobile widths and with keyboard-only navigation.

## Implementation sequence

### Checkpoint 1: Typed progress context and translations

Files:

- `app/src/rosary_session.rs`
- `app/src/components/guided_prayer.rs`
- `app/src/i18n.rs`

Work:

1. Extend the pure session model only if required to expose the active decade number and active prayer position. Build on `GuidedStep`, `DecadePrayer`, `active_decade`, and `step_number`; do not calculate decade state independently in the view.
2. Add focused unit tests for opening, each decade prayer, closing, first-step boundaries, last-step boundaries, and completion.
3. Replace the single generic progress presentation with a localized context such as `Passo 14 di 31 · Decina 2 di 5 · Ave Maria 4 di 10` and its English equivalent.
4. Add only the translation fields needed for the new labels. Reuse `guided_restart_label`, `guided_previous_label`, `guided_next_label`, and `guided_finish_label` where their meaning already matches.
5. Keep progress text short enough to wrap cleanly on mobile and expose it through the existing polite live region.

Acceptance:

- progress is correct for opening, each decade prayer, and closing;
- Italian and English render equivalent information;
- no hard-coded visible labels remain in the component.

Suggested commit: `feat(guided): add decade-aware progress context`

### Checkpoint 2: Responsive navigation and stable content frame

Files:

- `app/src/components/guided_prayer.rs`
- `style/main.scss`

Work:

1. Keep the three-column desktop layout: previous control, centered content, next control.
2. At the mobile breakpoint, change the layout to two rows: the step panel spans the full width and the previous/next controls form a bottom control bar.
3. Show concise translated labels beside the icons on mobile; retain icon-only controls on larger screens when the accessible name and title remain available.
4. Preserve a responsive `min-height` for the step frame, but avoid an always-forced fixed height. Permit the frame to grow for long prayers and cap/scroll only when the viewport cannot accommodate the content.
5. Give the scrollable panel stable scrollbar space and reset its scroll position to the top whenever the active step changes.
6. Keep the side/bottom controls vertically or horizontally centered independently of prayer text height.
7. Preserve the existing 48px minimum target size and focus outline.

Acceptance:

- desktop controls remain aligned when moving between short and long prayers;
- mobile controls are visible, labeled, and do not squeeze the prayer text;
- the Creed and other long prayers are fully readable;
- moving forward, backward, or restarting begins the new prayer at a predictable scroll position;
- no page-level horizontal overflow appears at 390px wide.

Suggested commit: `feat(guided): make navigation responsive`

### Checkpoint 3: Consistent SVG icons and completion action

Files:

- `app/src/components/guided_prayer.rs`
- `style/main.scss`

Work:

1. Replace Unicode `←`, `→`, and `↶` glyphs with small inline SVGs using a consistent `viewBox`, stroke width, and `aria-hidden="true"`.
2. Keep the translated accessible names: previous, next/complete, restart, and close.
3. Keep restart beside close in the header as a secondary shortcut available throughout the session.
4. Add a visible completion-state `Prega di nuovo` / `Pray again` primary button using `guided_restart_label`. It must call the same `RosarySession::reset` path as the header control.
5. Keep optional post-Rosary prayers in the completion state and preserve their current order and bilingual content.

Acceptance:

- icons render consistently across supported browsers;
- screen readers announce meaningful translated control names;
- the completion screen provides an obvious text action to begin again;
- restart behavior is identical from the header and completion CTA.

Suggested commit: `feat(guided): refine controls and completion state`

### Checkpoint 4: Motion, focus, and accessibility hardening

Files:

- `app/src/components/guided_prayer.rs`
- `style/main.scss`
- `end2end/tests/guided-prayer.spec.ts`

Work:

1. Add a short opacity/translate transition to the step-content wrapper without animating height.
2. Use keyed or equivalent step-content rendering so the transition runs when the step changes, while preserving the heading focus target.
3. Disable the transition under `prefers-reduced-motion: reduce`.
4. Preserve `aria-live="polite"` for progress updates and `aria-current="step"` for the active heading.
5. Verify that keyboard users can reach restart, close, previous, next, and the completion CTA in a logical order.
6. Ensure the focused heading remains visible without unexpectedly scrolling the whole page.

Acceptance:

- keyboard navigation still moves focus to the new heading;
- reduced-motion mode has no transform/transition requirement;
- the live region announces progress without duplicating the entire prayer text;
- focus-visible styling remains visible in both themes.

Suggested commit: `test(guided): harden responsive and accessible navigation`

## Browser coverage

Extend `end2end/tests/guided-prayer.spec.ts` to cover:

1. Desktop layout: side controls, centered text, stable panel geometry, disabled previous at step one.
2. Mobile layout at 390px: full-width step panel, bottom labeled controls, 48px minimum targets, and no horizontal overflow.
3. Forward/backward navigation across a short step and a long prayer, including panel scroll reset.
4. Italian and English decade-aware progress text.
5. Header restart from an intermediate step.
6. Completion state: optional prayers remain present and `Prega di nuovo` / `Pray again` returns to step one.
7. Keyboard focus after next, previous, restart, and completion restart.
8. Dark and light theme rendering with semantic colors intact.

Use stable roles, accessible names, and focused component classes rather than depending on raw glyph text. Run the focused file with one worker and the line reporter in constrained environments.

## Verification workflow

After each implementation checkpoint:

```text
cargo fmt --all -- --check
cargo check --workspace
```

Before marking the redesign complete:

```text
cargo test --workspace
cargo leptos build
cd end2end && npx playwright test tests/guided-prayer.spec.ts --workers=1 --reporter=line
cd end2end && npx playwright test --workers=1 --reporter=line
```

If browser processes cannot launch in the sandbox, retry the focused suite with the approved elevated browser environment and record the limitation if it persists. Do not treat a server bind failure or browser abort as an application assertion failure.

## Final review and closure

Before handoff:

- review `AGENTS.md` for any durable architecture or verification changes;
- confirm both translations and semantic HTML;
- inspect the diff for unintended changes to the Rosary session model or diagram;
- run `git diff --check`;
- confirm the focused and full browser suites have either passed or have a clearly documented environment blocker.

Suggested final closure commit: `chore(guided): close Rosary UX redesign`
