use super::state::{
    confirm_draft, intention_delete_label, intention_position_label, persist_update,
    reorder_by_offset, reorder_to_value, IntentionFeedback,
};
use crate::components::{AppButton, ButtonVariant};
use crate::i18n::Translation;
use crate::intentions::{remove_intention, INTENTIONS_MAX_COUNT, INTENTION_MAX_CHARS};
use leptos::{html, prelude::*};

#[component]
pub(super) fn IntentionTagList(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
    draft: RwSignal<Option<String>>,
    feedback: RwSignal<Option<IntentionFeedback>>,
    dragged_intention: RwSignal<Option<String>>,
    focus_after_reorder: RwSignal<Option<String>>,
    draft_input: NodeRef<html::Input>,
) -> impl IntoView {
    view! {
        <ul
            class="intention-tags"
            aria-label=move || copy.get().intention_list_label
        >
            <For
                each=move || intentions.get()
                key=|intention| intention.clone()
                children=move |intention| view! {
                    <SavedIntentionTag
                        copy
                        intentions
                        intention
                        feedback
                        dragged_intention
                        focus_after_reorder
                    />
                }
            />
            <Show when=move || draft.get().is_some()>
                <DraftIntentionTag copy intentions draft feedback draft_input />
            </Show>
            <AddIntentionControl copy intentions draft feedback />
        </ul>
    }
}

#[component]
fn SavedIntentionTag(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
    intention: String,
    feedback: RwSignal<Option<IntentionFeedback>>,
    dragged_intention: RwSignal<Option<String>>,
    focus_after_reorder: RwSignal<Option<String>>,
) -> impl IntoView {
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
            on:dragstart=move |_| dragged_intention.set(Some(drag_value.clone()))
            on:dragover=move |event| event.prevent_default()
            on:drop=move |event| {
                event.prevent_default();
                if let Some(source) = dragged_intention.get_untracked() {
                    reorder_to_value(intentions, &source, &drop_value, feedback);
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
            <AppButton
                variant=ButtonVariant::IconSecondary
                class="intention-delete-button"
                aria_label=move || intention_delete_label(
                    copy.get(),
                    &delete_label_value,
                    &intentions.get(),
                )
                on:keydown=move |event| event.stop_propagation()
                on_click=move |_| {
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
            </AppButton>
        </li>
    }
}

/// Renders the inline draft field and its embedded live character count.
#[component]
fn DraftIntentionTag(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
    draft: RwSignal<Option<String>>,
    feedback: RwSignal<Option<IntentionFeedback>>,
    draft_input: NodeRef<html::Input>,
) -> impl IntoView {
    view! {
        <li class="intention-draft-tag">
            <div class="intention-draft-field">
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
                <span
                    id="prayer-intention-character-count"
                    class="intention-character-count"
                    aria-label=move || format!(
                        "{}: {}/{}",
                        copy.get().intention_character_count_label,
                        draft.get().map_or(0, |value| value.chars().count()),
                        INTENTION_MAX_CHARS,
                    )
                >
                    {move || format!(
                        "{}/{}",
                        draft.get().map_or(0, |value| value.chars().count()),
                        INTENTION_MAX_CHARS,
                    )}
                </span>
            </div>
        </li>
    }
}

#[component]
fn AddIntentionControl(
    copy: Memo<Translation>,
    intentions: RwSignal<Vec<String>>,
    draft: RwSignal<Option<String>>,
    feedback: RwSignal<Option<IntentionFeedback>>,
) -> impl IntoView {
    view! {
        <li class="intention-add-slot">
            <Show
                when=move || {
                    draft.get().is_none() && intentions.get().len() < INTENTIONS_MAX_COUNT
                }
                fallback=move || view! {
                    <AppButton
                        variant=ButtonVariant::IconPrimary
                        class="intention-add-button"
                        aria_label=move || copy.get().intention_add_label
                        disabled=Signal::derive(|| true)
                        on_click=move |_| {}
                    >
                        <span aria-hidden="true">"+"</span>
                    </AppButton>
                }
            >
            <AppButton
                variant=ButtonVariant::IconPrimary
                class="intention-add-button"
                aria_label=move || copy.get().intention_add_label
                on_click=move |_| {
                        feedback.set(None);
                        draft.set(Some(String::new()));
                    }
                >
                    <span aria-hidden="true">"+"</span>
            </AppButton>
            </Show>
        </li>
    }
}
