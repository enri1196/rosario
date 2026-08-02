use super::GuideBox;
use crate::i18n::Translation;
use crate::intentions::{
    clear_intention, save_intention, validate_intention, IntentionValidationError,
    INTENTION_MAX_CHARS,
};
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum IntentionFeedback {
    Empty,
    TooLong,
    Saved,
    Cleared,
    StorageUnavailable,
}

impl IntentionFeedback {
    const fn is_error(self) -> bool {
        matches!(self, Self::Empty | Self::TooLong)
    }
}

#[component]
pub fn PrayerIntention(
    copy: Memo<Translation>,
    intention: RwSignal<Option<String>>,
) -> impl IntoView {
    let draft = RwSignal::new(intention.get_untracked().unwrap_or_default());
    let feedback = RwSignal::<Option<IntentionFeedback>>::new(None);

    view! {
        <GuideBox>
            <section class="prayer-intention" aria-labelledby="prayer-intention-title">
                <h3 id="prayer-intention-title">{move || copy.get().intention_title}</h3>
                <label for="prayer-intention-text">{move || copy.get().intention_label}</label>
                <textarea
                    id="prayer-intention-text"
                    rows="5"
                    prop:value=move || draft.get()
                    aria-describedby="prayer-intention-help prayer-intention-count prayer-intention-feedback"
                    aria-invalid=move || feedback.get().is_some_and(IntentionFeedback::is_error).to_string()
                    on:input=move |event| {
                        let value = event_target_value(&event);
                        let is_too_long = value.chars().count() > INTENTION_MAX_CHARS;
                        draft.set(value);
                        feedback.set(is_too_long.then_some(IntentionFeedback::TooLong));
                    }
                ></textarea>
                <div class="intention-meta">
                    <p id="prayer-intention-help" class="intention-help">
                        {move || copy.get().intention_help}
                    </p>
                    <p id="prayer-intention-count" class="intention-count">
                        {move || format!(
                            "{}: {}/{}",
                            copy.get().intention_character_count_label,
                            draft.get().chars().count(),
                            INTENTION_MAX_CHARS,
                        )}
                    </p>
                </div>
                <div class="intention-actions">
                    <button
                        type="button"
                        class="intention-save-button"
                        on:click=move |_| {
                            match validate_intention(&draft.get_untracked()) {
                                Ok(normalized) => {
                                    draft.set(normalized.clone());
                                    intention.set(Some(normalized.clone()));
                                    feedback.set(Some(if save_intention(&normalized) {
                                        IntentionFeedback::Saved
                                    } else {
                                        IntentionFeedback::StorageUnavailable
                                    }));
                                }
                                Err(IntentionValidationError::Empty) => {
                                    feedback.set(Some(IntentionFeedback::Empty));
                                }
                                Err(IntentionValidationError::TooLong) => {
                                    feedback.set(Some(IntentionFeedback::TooLong));
                                }
                            }
                        }
                    >
                        {move || copy.get().intention_save_label}
                    </button>
                    <button
                        type="button"
                        class="intention-clear-button"
                        on:click=move |_| {
                            draft.set(String::new());
                            intention.set(None);
                            feedback.set(Some(if clear_intention() {
                                IntentionFeedback::Cleared
                            } else {
                                IntentionFeedback::StorageUnavailable
                            }));
                        }
                    >
                        {move || copy.get().intention_clear_label}
                    </button>
                </div>
                <p
                    id="prayer-intention-feedback"
                    class:intention-error=move || feedback.get().is_some_and(IntentionFeedback::is_error)
                    class="intention-feedback"
                    aria-live="polite"
                    aria-atomic="true"
                >
                    {move || intention_feedback(copy.get(), feedback.get())}
                </p>
            </section>
        </GuideBox>
    }
}

fn intention_feedback(copy: Translation, feedback: Option<IntentionFeedback>) -> &'static str {
    match feedback {
        None => "",
        Some(IntentionFeedback::Empty) => copy.intention_empty_error,
        Some(IntentionFeedback::TooLong) => copy.intention_too_long_error,
        Some(IntentionFeedback::Saved) => copy.intention_saved_status,
        Some(IntentionFeedback::Cleared) => copy.intention_cleared_status,
        Some(IntentionFeedback::StorageUnavailable) => copy.intention_storage_error,
    }
}
