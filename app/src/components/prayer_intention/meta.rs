use super::state::{intention_feedback, IntentionFeedback};
use crate::i18n::Translation;
use crate::intentions::{INTENTIONS_MAX_COUNT, INTENTION_MAX_CHARS};
use leptos::prelude::*;

#[component]
pub(super) fn IntentionMeta(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
    draft: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="intention-meta">
            <p id="prayer-intention-help" class="intention-help">
                {move || copy.get().intention_help}
            </p>
            <p id="prayer-intention-character-count" class="intention-count">
                {move || format!(
                    "{}: {}/{}",
                    copy.get().intention_character_count_label,
                    draft.get().map_or(0, |value| value.chars().count()),
                    INTENTION_MAX_CHARS,
                )}
            </p>
            <p class="intention-count intention-total-count">
                {move || format!(
                    "{}: {}/{}",
                    copy.get().intention_count_label,
                    intentions.get().len(),
                    INTENTIONS_MAX_COUNT,
                )}
            </p>
        </div>
    }
}

#[component]
pub(super) fn IntentionFeedbackMessage(
    copy: Memo<Translation>,
    feedback: RwSignal<Option<IntentionFeedback>>,
) -> impl IntoView {
    view! {
        <p
            id="prayer-intention-feedback"
            class:intention-error=move || feedback.get().is_some_and(IntentionFeedback::is_error)
            class="intention-feedback"
            aria-live="polite"
            aria-atomic="true"
        >
            {move || intention_feedback(copy.get(), feedback.get())}
        </p>
    }
}
