use super::mystery_card::MysteryCard;
use crate::i18n::Translation;
use leptos::prelude::*;

#[component]
pub fn MysteriesSection(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <section class="mysteries-section" aria-labelledby="mysteries-heading">
            <h2 id="mysteries-heading" class="section-title">{move || copy.get().mysteries_heading}</h2>
            <div class="mysteries-grid">
                <For
                    each=move || copy.get().groups.iter().copied().enumerate()
                    key=|(_, group)| group.title
                    children=move |(_, group)| view! {
                        <section class="mystery-group">
                            <h3>{group.title}</h3>
                            <div class="mystery-list">
                                {group.mysteries.iter().copied().map(|mystery| view! {
                                    <MysteryCard mystery fruit_label=copy.get().fruit_label />
                                }).collect_view()}
                            </div>
                        </section>
                    }
                />
            </div>
        </section>
    }
}
