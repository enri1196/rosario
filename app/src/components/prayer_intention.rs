use super::GuideBox;
use crate::i18n::Translation;
use crate::intentions::{
    add_intention, move_intention, remove_intention, save_intentions, IntentionValidationError,
    INTENTIONS_MAX_COUNT, INTENTION_MAX_CHARS,
};
use leptos::{html, prelude::*};
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq, Eq)]
enum IntentionFeedback {
    TooLong,
    TooMany,
    Duplicate,
    Added,
    Removed,
    Reordered,
    StorageUnavailable,
}

impl IntentionFeedback {
    const fn is_error(self) -> bool {
        matches!(self, Self::TooLong | Self::TooMany | Self::Duplicate)
    }
}

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
                <h3 id="prayer-intention-title">{move || copy.get().intention_title}</h3>
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

                <ul
                    class="intention-tags"
                    aria-label=move || copy.get().intention_list_label
                >
                    <For
                        each=move || intentions.get()
                        key=|intention| intention.clone()
                        children=move |intention| {
                            let drag_value = intention.clone();
                            let drop_value = intention.clone();
                            let keyboard_value = intention.clone();
                            let label_value = intention.clone();
                            let delete_value = intention.clone();
                            let delete_label_value = intention.clone();

                            view! {
                                <li
                                    class="intention-tag"
                                    draggable="true"
                                    tabindex="0"
                                    aria-label=move || intention_position_label(
                                        copy.get(),
                                        &label_value,
                                        &intentions.get(),
                                    )
                                    on:dragstart=move |_| {
                                        dragged_intention.set(Some(drag_value.clone()));
                                    }
                                    on:dragover=move |event| event.prevent_default()
                                    on:drop=move |event| {
                                        event.prevent_default();
                                        if let Some(source) = dragged_intention.get_untracked() {
                                            reorder_to_value(
                                                intentions,
                                                &source,
                                                &drop_value,
                                                feedback,
                                            );
                                        }
                                        dragged_intention.set(None);
                                    }
                                    on:dragend=move |_| dragged_intention.set(None)
                                    on:keydown=move |event| {
                                        let offset = match event.key().as_str() {
                                            "ArrowLeft" | "ArrowUp" => -1,
                                            "ArrowRight" | "ArrowDown" => 1,
                                            _ => 0,
                                        };
                                        if offset != 0 {
                                            event.prevent_default();
                                            reorder_by_offset(
                                                intentions,
                                                &keyboard_value,
                                                offset,
                                                feedback,
                                                focus_after_reorder,
                                            );
                                        }
                                    }
                                >
                                    <span class="intention-drag-handle" aria-hidden="true">"⠿"</span>
                                    <span class="intention-tag-text">{intention}</span>
                                    <button
                                        type="button"
                                        class="intention-delete-button"
                                        aria-label=move || intention_delete_label(
                                            copy.get(),
                                            &delete_label_value,
                                            &intentions.get(),
                                        )
                                        on:keydown=move |event| event.stop_propagation()
                                        on:click=move |_| {
                                            let updated = remove_intention(
                                                &intentions.get_untracked(),
                                                &delete_value,
                                            );
                                            persist_update(
                                                intentions,
                                                updated,
                                                feedback,
                                                IntentionFeedback::Removed,
                                            );
                                        }
                                    >
                                        <span aria-hidden="true">"×"</span>
                                    </button>
                                </li>
                            }
                        }
                    />
                    <Show when=move || draft.get().is_some()>
                        <li class="intention-draft-tag">
                            <input
                                id="prayer-intention-text"
                                class="intention-tag-input"
                                node_ref=draft_input
                                type="text"
                                autocomplete="off"
                                aria-label=move || copy.get().intention_label
                                prop:value=move || draft.get().unwrap_or_default()
                                aria-describedby="prayer-intention-help prayer-intention-character-count prayer-intention-feedback"
                                aria-invalid=move || feedback.get().is_some_and(IntentionFeedback::is_error).to_string()
                                on:input=move |event| {
                                    let value = event_target_value(&event);
                                    let is_too_long = value.chars().count() > INTENTION_MAX_CHARS;
                                    draft.set(Some(value));
                                    feedback.set(is_too_long.then_some(IntentionFeedback::TooLong));
                                }
                                on:blur=move |_| confirm_draft(intentions, draft, feedback)
                                on:keydown=move |event| match event.key().as_str() {
                                    "Enter" => {
                                        event.prevent_default();
                                        confirm_draft(intentions, draft, feedback);
                                    }
                                    "Escape" => {
                                        event.prevent_default();
                                        draft.set(None);
                                        feedback.set(None);
                                    }
                                    _ => {}
                                }
                            />
                        </li>
                    </Show>
                    <li class="intention-add-slot">
                        <Show
                            when=move || {
                                draft.get().is_none()
                                    && intentions.get().len() < INTENTIONS_MAX_COUNT
                            }
                            fallback=move || view! {
                                <button
                                    type="button"
                                    class="intention-add-button"
                                    aria-label=move || copy.get().intention_add_label
                                    disabled=true
                                >
                                    <span aria-hidden="true">"+"</span>
                                </button>
                            }
                        >
                            <button
                                type="button"
                                class="intention-add-button"
                                aria-label=move || copy.get().intention_add_label
                                on:click=move |_| {
                                    feedback.set(None);
                                    draft.set(Some(String::new()));
                                }
                            >
                                <span aria-hidden="true">"+"</span>
                            </button>
                        </Show>
                    </li>
                </ul>

                <Show when=move || intentions.get().is_empty() && draft.get().is_none()>
                    <p class="intention-empty-state">{move || copy.get().intention_empty_state}</p>
                </Show>

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

fn confirm_draft(
    intentions: RwSignal<Vec<String>>,
    draft: RwSignal<Option<String>>,
    feedback: RwSignal<Option<IntentionFeedback>>,
) {
    let Some(value) = draft.get_untracked() else {
        return;
    };

    match add_intention(&intentions.get_untracked(), &value) {
        Ok(updated) => {
            draft.set(None);
            persist_update(intentions, updated, feedback, IntentionFeedback::Added);
        }
        Err(IntentionValidationError::Empty) => {
            draft.set(None);
            feedback.set(None);
        }
        Err(IntentionValidationError::TooLong) => {
            feedback.set(Some(IntentionFeedback::TooLong));
        }
        Err(IntentionValidationError::TooMany) => {
            feedback.set(Some(IntentionFeedback::TooMany));
        }
        Err(IntentionValidationError::Duplicate) => {
            feedback.set(Some(IntentionFeedback::Duplicate));
        }
    }
}

fn focus_draft_input(draft_input: NodeRef<html::Input>) {
    if let Some(input) = draft_input.get() {
        let _ = input.focus();
    }
}

fn reorder_to_value(
    intentions: RwSignal<Vec<String>>,
    source: &str,
    target: &str,
    feedback: RwSignal<Option<IntentionFeedback>>,
) {
    let current = intentions.get_untracked();
    let Some(from) = current.iter().position(|value| value == source) else {
        return;
    };
    let Some(to) = current.iter().position(|value| value == target) else {
        return;
    };
    if from == to {
        return;
    }

    persist_update(
        intentions,
        move_intention(&current, from, to),
        feedback,
        IntentionFeedback::Reordered,
    );
}

fn reorder_by_offset(
    intentions: RwSignal<Vec<String>>,
    value: &str,
    offset: isize,
    feedback: RwSignal<Option<IntentionFeedback>>,
    focus_after_reorder: RwSignal<Option<String>>,
) {
    let current = intentions.get_untracked();
    let Some(from) = current.iter().position(|intention| intention == value) else {
        return;
    };
    let to = from.saturating_add_signed(offset).min(current.len() - 1);
    if from == to {
        return;
    }

    persist_update(
        intentions,
        move_intention(&current, from, to),
        feedback,
        IntentionFeedback::Reordered,
    );
    focus_after_reorder.set(Some(value.to_owned()));
}

#[cfg(target_arch = "wasm32")]
fn focus_intention_at(position: usize) {
    let selector = format!(
        ".intention-tags > .intention-tag:nth-child({})",
        position + 1
    );
    let element = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.query_selector(&selector).ok().flatten())
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok());
    if let Some(element) = element {
        let _ = element.focus();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_intention_at(_position: usize) {}

fn persist_update(
    intentions: RwSignal<Vec<String>>,
    updated: Vec<String>,
    feedback: RwSignal<Option<IntentionFeedback>>,
    success: IntentionFeedback,
) {
    intentions.set(updated.clone());
    feedback.set(Some(if save_intentions(&updated) {
        success
    } else {
        IntentionFeedback::StorageUnavailable
    }));
}

fn intention_position_label(copy: Translation, value: &str, intentions: &[String]) -> String {
    let position = intentions
        .iter()
        .position(|intention| intention == value)
        .map_or(0, |index| index + 1);
    format!(
        "{} {} {} {}. {}",
        copy.intention_item_label,
        position,
        copy.guided_of_label,
        intentions.len(),
        copy.intention_reorder_label,
    )
}

fn intention_delete_label(copy: Translation, value: &str, intentions: &[String]) -> String {
    let position = intentions
        .iter()
        .position(|intention| intention == value)
        .map_or(0, |index| index + 1);
    format!("{} {}", copy.intention_delete_label, position)
}

fn intention_feedback(copy: Translation, feedback: Option<IntentionFeedback>) -> &'static str {
    match feedback {
        None => "",
        Some(IntentionFeedback::TooLong) => copy.intention_too_long_error,
        Some(IntentionFeedback::TooMany) => copy.intention_too_many_error,
        Some(IntentionFeedback::Duplicate) => copy.intention_duplicate_error,
        Some(IntentionFeedback::Added) => copy.intention_added_status,
        Some(IntentionFeedback::Removed) => copy.intention_removed_status,
        Some(IntentionFeedback::Reordered) => copy.intention_reordered_status,
        Some(IntentionFeedback::StorageUnavailable) => copy.intention_storage_error,
    }
}
