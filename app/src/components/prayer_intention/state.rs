use crate::i18n::Translation;
use crate::intentions::{add_intention, move_intention, save_intentions, IntentionValidationError};
use leptos::{html, prelude::*};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum IntentionFeedback {
    TooLong,
    TooMany,
    Duplicate,
    Added,
    Removed,
    Reordered,
    StorageUnavailable,
}

impl IntentionFeedback {
    pub(super) const fn is_error(self) -> bool {
        matches!(self, Self::TooLong | Self::TooMany | Self::Duplicate)
    }
}

pub(super) fn confirm_draft(
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

pub(super) fn focus_draft_input(draft_input: NodeRef<html::Input>) {
    if let Some(input) = draft_input.get() {
        let _ = input.focus();
    }
}

pub(super) fn reorder_to_value(
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

pub(super) fn reorder_by_offset(
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
pub(super) fn focus_intention_at(position: usize) {
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
pub(super) fn focus_intention_at(_position: usize) {}

pub(super) fn persist_update(
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

pub(super) fn intention_position_label(
    copy: Translation,
    value: &str,
    intentions: &[String],
) -> String {
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

pub(super) fn intention_delete_label(
    copy: Translation,
    value: &str,
    intentions: &[String],
) -> String {
    let position = intentions
        .iter()
        .position(|intention| intention == value)
        .map_or(0, |index| index + 1);
    format!("{} {}", copy.intention_delete_label, position)
}

pub(super) fn intention_feedback(
    copy: Translation,
    feedback: Option<IntentionFeedback>,
) -> &'static str {
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
