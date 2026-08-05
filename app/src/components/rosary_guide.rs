use super::rosary_diagram::RosaryDiagram;
use super::{GuideBox, GuidedPrayer, MysteryRecommendation, PrayerIntention};
use crate::i18n::{Language, Translation};
use crate::rosary_session::RosarySession;
use leptos::prelude::*;

#[component]
pub fn RosaryGuide(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    guided_session: RwSignal<Option<RosarySession>>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    view! {
        <section class="guide" aria-labelledby="guide-heading">
            <h2 id="guide-heading" class="section-kicker">{move || copy.get().guide_title}</h2>
            <PrayerIntention copy intentions />

            <GuidedPrayer copy language session=guided_session intentions />

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
        </section>
    }
}
