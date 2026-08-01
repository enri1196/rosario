# Agent instructions

## Use CodeGraph first

This repository has a CodeGraph MCP index. Use CodeGraph as the primary way
to read, search, and understand source code. Before changing code, consult the
index so that edits are based on the repository's symbols and relationships,
not on an incomplete file-by-file scan.

Do not begin source exploration with `Read`, `rg`, `grep`, `find`, `sed`,
`cat`, `head`, `tail`, `less`, ad-hoc scripts, or shell loops. Do not use those
commands as a second pass to re-check results that CodeGraph already returned.
They lose structural context and duplicate the indexed work.

## Choose the narrowest useful CodeGraph query

- Use `codegraph_context` first for a task, feature, bug, or architectural
  question. It combines search, symbol context, callers, and callees. Include
  code when implementation details are needed and cap `maxNodes` to the size
  of the question.
- Use `codegraph_search` when the exact symbol name is known or needs to be
  discovered. It is a locator, not a replacement for context.
- Use `codegraph_node` for one symbol's signature, location, callers/callees,
  or complete body.
- Use one `codegraph_explore` call for several related symbols or files. Search
  first when names are unknown, then request a capped set with `maxFiles`.
  Do not loop over many `codegraph_node` calls.
- Use `codegraph_trace` for a complete flow from one symbol to another. Do not
  reconstruct a path manually with repeated search, caller, and callee calls.
- Use `codegraph_callers` to learn who invokes a symbol, `codegraph_callees`
  to learn what it invokes, and `codegraph_impact` before a refactor or API
  change. These tools are preferred over manually walking the graph.
- Use `codegraph_files` when the question is about directory or file
  inventory, rather than shell listing commands.
- Use `codegraph_status` to check index readiness, size, and pending syncs.

Useful query chains:

1. Onboarding or an unfamiliar task: `codegraph_context`, then at most one
   focused `codegraph_explore` or `codegraph_node` call.
2. A flow question: `codegraph_trace` from the starting symbol to the target,
   then explore only the hop bodies that need explanation.
3. A refactor: `codegraph_search`, `codegraph_callers`, then
   `codegraph_impact` before editing.
4. A regression: inspect callers of the suspected symbol, then widen to
   impact if the dependency surface is unexpected.

Prefer two or three focused calls over broad repeated queries. CodeGraph is an
AST-backed knowledge graph, so trust its returned symbol relationships and
source unless it reports a known limitation.

## Editing and index freshness

Use `apply_patch` for source edits. Do not call CodeGraph while an edit is in
progress. Its index is updated by a file watcher and may lag writes by about a
second.

After editing:

1. Allow the watcher time to synchronize.
2. Check CodeGraph status or the response staleness banner.
3. Re-query affected symbols once the index is fresh.
4. If a response says that specific files were edited since the last sync,
   treat those files as pending. Continue trusting unaffected results.

If the project is not initialized or the index is unavailable, report that
condition and offer to initialize it with `codegraph init -i`. Do not silently
replace CodeGraph with a broad shell search. A narrowly scoped direct file read
is a last resort only when CodeGraph cannot expose a required detail or
explicitly identifies an edited file as stale.

## Verification

CodeGraph provides structural context, not correctness validation. After an
edit, run the repository's appropriate formatter, compiler, linter, and tests.
Use shell commands for those checks and for version-control metadata, but not
as a substitute for understanding source. Keep verification proportional to
the change and report what passed or what could not run.

Remember that cross-file resolution is best-effort name matching and dynamic
dispatch can be ambiguous. When a result is ambiguous, narrow the query by
symbol name, file, or flow and state the uncertainty rather than guessing.

## Quick project map

This is a small bilingual Rosary guide built with Leptos and Rust. Start with
the application composition and follow the data outward:

- `app/src/lib.rs` is the application entry point. `App` owns the reactive
  language signal and derives the shared `Memo<Translation>` passed to the
  page components.
- `app/src/i18n.rs` contains `Language`, `Translation`, prayer text, mystery
  data, labels, and the Italian/English content. Add user-visible bilingual
  copy here rather than hard-coding it in a component.
- `app/src/components/header.rs` renders the title, language selector, skip
  link, and document-language update.
- `app/src/components/prayer_sidebar.rs` renders the five reusable prayers.
- `app/src/components/rosary_guide.rs` renders the creed, rosary diagram,
  step legend, ending text, and decade note.
- `app/src/components/rosary_diagram.rs` owns the diagram SVG and its labels.
- `app/src/components/mysteries_section.rs` and
  `app/src/components/mystery_card.rs` render the mystery groups and cards.
- `app/src/components/footer.rs` renders the footer; `mod.rs` exposes the
  component module surface.
- `style/main.scss` is the shared responsive visual system. Component class
  names map directly to the selectors here.
- `frontend` is the WebAssembly entry package; `server` serves the generated
  Leptos site and provides the Axum fallback to `index.html`.
- `end2end` contains the Playwright browser checks for the initial rendering
  and language switching.

## Fast start for a new feature

For a new conversation, use this short sequence:

1. Read this file, then call `codegraph_status` and
   `codegraph_context` with the feature request.
2. Search for the nearest existing symbol, translation field, component, and
   CSS class. Use `codegraph_trace` for behavior that crosses components and
   `codegraph_impact` before changing shared data or public component inputs.
3. Decide the smallest owning layer: translations/data in `i18n.rs`, markup
   and behavior in a component, layout and visual treatment in `main.scss`,
   browser coverage in `end2end`, or server/runtime behavior in `server`.
4. Implement with `apply_patch`, preserving the existing reactive pattern:
   pass the shared `Memo<Translation>` down and read localized values through
   `copy.get()` in rendered closures.
5. Wait for CodeGraph's file watcher, confirm the affected files are synced,
   and re-query the changed symbols. This catches stale structural assumptions
   before verification.
6. Run the narrowest useful checks first, then the standard workspace checks:
   `cargo fmt --all -- --check`, `cargo check --workspace`, and, when the
   toolchain is available, `cargo leptos build`. Run the Playwright suite for
   user-visible flows such as language switching.
7. Summarize the changed files, behavior, and checks in the handoff so the
   next conversation can continue from evidence rather than rediscovery.

When adding a visible feature, update both language translations, preserve
semantic HTML and accessible labels, keep mobile layouts in mind, and add or
extend an end-to-end check when the behavior is user-facing.
