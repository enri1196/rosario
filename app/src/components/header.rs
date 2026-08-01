use crate::i18n::{Language, Translation};
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
                // This changes only effective state; `theme.rs` owns browser
                // persistence and `App` owns document synchronization.
                <button
                    class="theme-toggle"
                    type="button"
                    aria-label=move || match theme.get() {
                        Theme::Dark => copy.get().light_theme_label,
                        Theme::Light => copy.get().dark_theme_label,
                    }
                    aria-pressed=move || (theme.get() == Theme::Light).to_string()
                    title=move || match theme.get() {
                        Theme::Dark => copy.get().light_theme_label,
                        Theme::Light => copy.get().dark_theme_label,
                    }
                    on:click=move |_| theme.update(|current| *current = current.toggle())
                >
                    <span class="theme-toggle-icon" aria-hidden="true">
                        {move || if theme.get() == Theme::Dark { "☀" } else { "☾" }}
                    </span>
                    <span class="theme-toggle-label">
                        <span class="visually-hidden">{move || format!("{}: ", copy.get().theme_control_label)}</span>
                        {move || match theme.get() {
                            Theme::Dark => copy.get().light_theme_label,
                            Theme::Light => copy.get().dark_theme_label,
                        }}
                    </span>
                </button>
                <h1>{move || copy.get().heading}</h1>
                <label class="language-picker">
                    <span>{move || copy.get().language_label}</span>
                    <select
                        aria-label=move || copy.get().language_label
                        on:change=move |event| language.set(Language::from_code(&event_target_value(&event)))
                    >
                        {Language::ALL.into_iter().map(|item| view! {
                            <option value=item.code() selected=move || language.get() == item>{item.label()}</option>
                        }).collect_view()}
                    </select>
                </label>
            </div>
            <div class="ornament" aria-hidden="true">"✦ ✦ ✦ ❧ ✦ ✦ ✦"</div>
        </header>
    }
}
