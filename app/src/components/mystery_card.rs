use crate::i18n::Mystery;
use leptos::prelude::*;

#[component]
pub(super) fn MysteryCard(mystery: Mystery, fruit_label: &'static str) -> impl IntoView {
    view! {
        <article class="mystery-card">
            <div class="mystery-art" aria-hidden="true">{mystery.icon}</div>
            <h4>{mystery.title}</h4>
            <p>{mystery.meditation}</p>
            <p class="mystery-fruit"><strong>{fruit_label}</strong><span>{mystery.fruit}</span></p>
        </article>
    }
}
