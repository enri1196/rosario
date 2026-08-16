use super::rosary_diagram::RosaryDiagram;
use super::{GuideBox, GuidedPrayer, MysteryRecommendation, PrayerIntention};
use crate::i18n::{Language, Translation};
use crate::navigation::AppSection;
use crate::rosary_session::RosarySession;
use leptos::prelude::*;

/// Renders either the complete Guide overview card or the guided-session panel.
#[component]
pub fn RosaryGuide(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    guided_session: RwSignal<Option<RosarySession>>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || guided_session.get().is_some()
            fallback=move || view! {
                <section
                    id=AppSection::Guide.panel_id()
                    class="workspace-panel guide"
                    role="tabpanel"
                    aria-labelledby=AppSection::Guide.control_id()
                >
                    <h2
                        id=AppSection::Guide.heading_id()
                        class="section-kicker workspace-heading"
                        tabindex="-1"
                    >
                        {move || copy.get().guide_title}
                    </h2>
                    <RosaryOverview copy language guided_session intentions />
                </section>
            }
        >
            <section
                id=AppSection::Guide.panel_id()
                class="workspace-panel guided-workspace-panel"
                role="tabpanel"
                aria-labelledby=AppSection::Guide.control_id()
            >
                <GuidedPrayer copy language session=guided_session intentions />
            </section>
        </Show>
    }
}

/// Renders the non-session Rosary diagram, recommendation, and guide notes.
#[component]
fn RosaryOverview(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    guided_session: RwSignal<Option<RosarySession>>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <div class="rosary-overview">
            <PrayerIntention copy intentions />

            <RosaryDiagram copy guided_session />

            <MysteryRecommendation copy language />

            <GuideBox>
                <ol class="steps-legend">
                    <For
                        each=move || copy.get().steps.iter().copied().enumerate()
                        key=|(_, step)| *step
                        children=move |(index, step)| view! {
                            <li><span class="step-num">{index + 1}</span><span>{step}</span></li>
                        }
                    />
                </ol>
            </GuideBox>

            <GuideBox>
                <div class="end-box">
                    <h3>{move || copy.get().ending_title}</h3>
                    <p>{move || copy.get().ending}</p>
                </div>
            </GuideBox>
            <GuideBox>
                <p class="note-box">{move || copy.get().decade_note}</p>
            </GuideBox>
        </div>
    }
}
