use super::mystery_card::MysteryCard;
use crate::i18n::{Language, Translation};
use crate::rosary_session::{Decade, RosarySession};
use leptos::prelude::*;

#[component]
pub fn MysteriesSection(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    guided_session: RwSignal<Option<RosarySession>>,
) -> impl IntoView {
    view! {
        <section class="mysteries-section" aria-labelledby="mysteries-heading">
            <h2 id="mysteries-heading" class="section-title">{move || copy.get().mysteries_heading}</h2>
            <div class="mysteries-grid">
                <For
                each=move || copy.get().groups.iter().copied().enumerate()
                key=|(_, group)| group.set
                children=move |(_, group)| {
                    let set = group.set;
                    view! {
                        <section class="mystery-group">
                            <h3>{move || set.label(language.get())}</h3>
                            <div class="mystery-list">
                                {group.mysteries.iter().copied().enumerate().map(|(index, mystery)| view! {
                                    <MysteryCard
                                        mystery
                                        mystery_set=set
                                        decade=Decade::from_index(index).expect("mystery groups have five decades")
                                        fruit_label=copy.get().fruit_label
                                        pray_label=copy.get().guided_pray_mystery_label
                                        guided_session
                                    />
                                }).collect_view()}
                            </div>
                        </section>
                    }
                }
                />
            </div>
        </section>
    }
}
