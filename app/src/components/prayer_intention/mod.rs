mod meta;
mod state;
mod tag_list;

use self::meta::{IntentionFeedbackMessage, IntentionMeta};
use self::state::{focus_draft_input, focus_intention_at, IntentionFeedback};
use self::tag_list::IntentionTagList;
use super::GuideBox;
use crate::i18n::Translation;
use crate::intentions::INTENTIONS_MAX_COUNT;
use leptos::{html, prelude::*};
use std::cell::Cell;

#[component]
pub fn PrayerIntention(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    let draft = RwSignal::<Option<String>>::new(None);
    let feedback = RwSignal::<Option<IntentionFeedback>>::new(None);
    let dragged_intention = RwSignal::<Option<String>>::new(None);
    let focus_after_reorder = RwSignal::<Option<String>>::new(None);
    let draft_input = NodeRef::<html::Input>::new();
    let was_drafting = Cell::new(false);

    Effect::new(move |_| {
        let is_drafting = draft.get().is_some();
        let was_drafting = was_drafting.replace(is_drafting);
        if is_drafting && !was_drafting {
            focus_draft_input(draft_input);
        }
    });

    Effect::new(move |_| {
        let Some(value) = focus_after_reorder.get() else {
            return;
        };
        if let Some(position) = intentions
            .get()
            .iter()
            .position(|intention| intention == &value)
        {
            focus_intention_at(position);
        }
        focus_after_reorder.set(None);
    });

    view! {
        <GuideBox>
            <section class="prayer-intention" aria-labelledby="prayer-intention-title">
                <header class="intention-heading-row">
                    <h3 id="prayer-intention-title">{move || copy.get().intention_title}</h3>
                    <span
                        class="intention-count intention-total-count"
                        aria-label=move || format!(
                            "{}: {}/{}",
                            copy.get().intention_count_label,
                            intentions.get().len(),
                            INTENTIONS_MAX_COUNT,
                        )
                    >
                        {move || format!("{}/{}", intentions.get().len(), INTENTIONS_MAX_COUNT)}
                    </span>
                </header>
                <IntentionMeta copy />
                <IntentionTagList
                    copy
                    intentions
                    draft
                    feedback
                    dragged_intention
                    focus_after_reorder
                    draft_input
                />

                <IntentionFeedbackMessage copy feedback />
            </section>
        </GuideBox>
    }
}
