use crate::i18n::Translation;
use leptos::prelude::*;

#[component]
pub fn PrayerSidebar(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <aside class="prayers-col" aria-labelledby="prayers-heading">
            <h2 id="prayers-heading" class="visually-hidden">{move || copy.get().prayers_heading}</h2>
            <For
                each=move || copy.get().prayers.iter().copied().enumerate()
                key=|(_, prayer)| prayer.title
                children=move |(_, prayer)| view! {
                    <article class="prayer-block">
                        <h3>{prayer.title}</h3>
                        <p>{prayer.text}</p>
                    </article>
                }
            />
        </aside>
    }
}
