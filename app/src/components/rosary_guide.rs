use super::rosary_diagram::RosaryDiagram;
use crate::i18n::Translation;
use leptos::prelude::*;

#[component]
pub fn RosaryGuide(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <section class="guide" aria-labelledby="guide-heading">
            <h2 id="guide-heading" class="section-kicker">{move || copy.get().guide_title}</h2>
            <article class="creed-box">
                <h3>{move || copy.get().creed_title}</h3>
                <p>{move || copy.get().creed}</p>
            </article>

            <RosaryDiagram copy />

            <ol class="steps-legend">
                <For
                    each=move || copy.get().steps.iter().copied().enumerate()
                    key=|(_, step)| *step
                    children=move |(index, step)| view! {
                        <li><span class="step-num">{index + 1}</span><span>{step}</span></li>
                    }
                />
            </ol>

            <div class="end-box">
                <h3>{move || copy.get().ending_title}</h3>
                <p>{move || copy.get().ending}</p>
            </div>
            <p class="note-box">{move || copy.get().decade_note}</p>
        </section>
    }
}
