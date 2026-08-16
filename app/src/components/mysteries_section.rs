use super::mystery_card::MysteryCard;
use crate::i18n::{Language, MysterySet, Translation};
use crate::navigation::AppSection;
use crate::rosary_session::Decade;
use leptos::prelude::*;

/// Renders every mystery group as the independent Mysteries workspace view.
#[component]
pub fn MysteriesSection(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    open_guided: Callback<(MysterySet, Decade)>,
) -> impl IntoView {
    view! {
        <section
            id=AppSection::Mysteries.panel_id()
            class="workspace-panel mysteries-section"
            role="tabpanel"
            aria-labelledby=AppSection::Mysteries.control_id()
        >
            <h2
                id=AppSection::Mysteries.heading_id()
                class="section-title workspace-heading"
                tabindex="-1"
            >
                {move || copy.get().mysteries_heading}
            </h2>
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
                                        open_guided
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
