use crate::i18n::Translation;
use leptos::prelude::*;

#[component]
pub fn Footer(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <footer class="footer">
            <span>"✦ "{move || copy.get().heading}" ✦"</span>
            <span>{move || copy.get().version}</span>
            <span>"✦ AD MAIOREM DEI GLORIAM ✦"</span>
        </footer>
    }
}
