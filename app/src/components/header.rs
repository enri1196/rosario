use crate::i18n::{Language, Translation};
use leptos::prelude::*;

#[component]
pub fn Header(language: RwSignal<Language>, copy: Memo<Translation>) -> impl IntoView {
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
