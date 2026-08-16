use super::state::{intention_feedback, IntentionFeedback};
use crate::i18n::Translation;
use leptos::prelude::*;

/// Renders persistent help text for the intention editor.
#[component]
pub(super) fn IntentionMeta(copy: Memo<Translation>) -> impl IntoView {
    view! {
        <div class="intention-meta">
            <p id="prayer-intention-help" class="intention-help">
                {move || copy.get().intention_help}
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
        <Show when=move || feedback.get().is_some_and(IntentionFeedback::should_show)>
            <p
                id="prayer-intention-feedback"
                class:intention-error=move || feedback.get().is_some_and(IntentionFeedback::is_error)
                class="intention-feedback"
                aria-live="polite"
                aria-atomic="true"
            >
                {move || intention_feedback(copy.get(), feedback.get())}
            </p>
        </Show>
    }
}
