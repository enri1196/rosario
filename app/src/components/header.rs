use crate::i18n::{persist_language, Language, Translation};
use crate::theme::Theme;
use leptos::prelude::*;

/// Renders the bilingual header controls and synchronizes document language.
///
/// `theme` is the application's effective display mode; browser persistence
/// and root-document synchronization remain owned by `theme.rs` and `App`.
#[component]
pub fn Header(
    language: RwSignal<Language>,
    copy: Memo<Translation>,
    theme: RwSignal<Theme>,
) -> impl IntoView {
    Effect::new(move |_| {
        let code = language.get().code();
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            if let Some(root) = document.document_element() {
                let _ = root.set_attribute("lang", code);
            }
        }
    });

    view! {
        <a class="skip-link" href="#main-content">{move || copy.get().skip_link}</a>
        <header class="site-header">
            <div class="ornament" aria-hidden="true">"✦ ✦ ✦ ❧ ✦ ✦ ✦"</div>
            <div class="header-row">
                <span class="header-spacer" aria-hidden="true"></span>
                <h1>{move || copy.get().heading}</h1>
                <div class="header-controls">
                    <label class="language-picker">
                        <select
                            aria-label=move || copy.get().language_label
                            on:change=move |event| {
                                let selected_language = Language::from_code(&event_target_value(&event));
                                language.set(selected_language);
                                persist_language(selected_language);
                            }
                        >
                            {Language::ALL.into_iter().map(|item| view! {
                                <option value=item.code() selected=move || language.get() == item>{item.label()}</option>
                            }).collect_view()}
                        </select>
                    </label>
                    // This changes only effective state; `theme.rs` owns browser
                    // persistence and `App` owns document synchronization.
                    <button
                        class="theme-toggle"
                        type="button"
                        aria-label=move || format!("{}: {}", copy.get().theme_control_label, match theme.get() {
                            Theme::Dark => copy.get().light_theme_label,
                            Theme::Light => copy.get().dark_theme_label,
                        })
                        aria-pressed=move || (theme.get() == Theme::Light).to_string()
                        title=move || match theme.get() {
                            Theme::Dark => copy.get().light_theme_label,
                            Theme::Light => copy.get().dark_theme_label,
                        }
                        on:click=move |_| theme.update(|current| *current = current.toggle())
                    >
                        <span aria-hidden="true">
                            {move || if theme.get() == Theme::Dark { "☀" } else { "☾" }}
                        </span>
                    </button>
                </div>
            </div>
            <div class="ornament" aria-hidden="true">"✦ ✦ ✦ ❧ ✦ ✦ ✦"</div>
        </header>
    }
}
