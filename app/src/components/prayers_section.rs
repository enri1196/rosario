use crate::i18n::Translation;
use crate::navigation::AppSection;
use leptos::prelude::*;

/// Renders the complete Rosary prayer collection as an independent workspace view.
#[component]
pub fn PrayersSection(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <section
            id=AppSection::Prayers.panel_id()
            class="workspace-panel prayers-section"
            role="tabpanel"
            aria-labelledby=AppSection::Prayers.control_id()
        >
            <h2
                id=AppSection::Prayers.heading_id()
                class="section-title workspace-heading"
                tabindex="-1"
            >
                {move || copy.get().prayers_heading}
            </h2>
            <div class="prayers-list">
                <For
                    each=move || copy.get().sidebar_prayers.iter().copied().enumerate()
                    key=|(_, prayer)| prayer.title
                    children=move |(_, prayer)| view! {
                        <article class="prayer-block">
                            <h3>{prayer.title}</h3>
                            <p>{prayer.text}</p>
                        </article>
                    }
                />
            </div>
        </section>
    }
}
