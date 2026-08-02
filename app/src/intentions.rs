#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) const INTENTIONS_STORAGE_KEY: &str = "rosary-intentions";
pub(crate) const INTENTION_MAX_CHARS: usize = 50;
pub(crate) const INTENTIONS_MAX_COUNT: usize = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IntentionValidationError {
    Empty,
    TooLong,
    TooMany,
    Duplicate,
}

pub(crate) fn normalize_intention(value: &str) -> String {
    value.trim().to_owned()
}

pub(crate) fn validate_intention(value: &str) -> Result<String, IntentionValidationError> {
    let normalized = normalize_intention(value);
    if normalized.is_empty() {
        return Err(IntentionValidationError::Empty);
    }
    if normalized.chars().count() > INTENTION_MAX_CHARS {
        return Err(IntentionValidationError::TooLong);
    }

    Ok(normalized)
}

pub(crate) fn add_intention(
    intentions: &[String],
    value: &str,
) -> Result<Vec<String>, IntentionValidationError> {
    let normalized = validate_intention(value)?;
    if intentions.iter().any(|existing| existing == &normalized) {
        return Err(IntentionValidationError::Duplicate);
    }
    if intentions.len() >= INTENTIONS_MAX_COUNT {
        return Err(IntentionValidationError::TooMany);
    }

    let mut updated = intentions.to_vec();
    updated.push(normalized);
    Ok(updated)
}

pub(crate) fn remove_intention(intentions: &[String], value: &str) -> Vec<String> {
    intentions
        .iter()
        .filter(|intention| intention.as_str() != value)
        .cloned()
        .collect()
}

pub(crate) fn move_intention(intentions: &[String], from: usize, to: usize) -> Vec<String> {
    if from >= intentions.len() || to >= intentions.len() || from == to {
        return intentions.to_vec();
    }

    let mut reordered = intentions.to_vec();
    let intention = reordered.remove(from);
    reordered.insert(to, intention);
    reordered
}

fn validate_intentions(values: Vec<String>) -> Option<Vec<String>> {
    if values.len() > INTENTIONS_MAX_COUNT {
        return None;
    }

    let mut normalized = Vec::with_capacity(values.len());
    for value in values {
        let value = validate_intention(&value).ok()?;
        if normalized.contains(&value) {
            return None;
        }
        normalized.push(value);
    }
    Some(normalized)
}

pub(crate) fn resolve_stored_intentions(saved: Option<&str>) -> Vec<String> {
    let Some(saved) = saved else {
        return Vec::new();
    };
    let trimmed = saved.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    if trimmed.starts_with('[') {
        return serde_json::from_str::<Vec<String>>(trimmed)
            .ok()
            .and_then(validate_intentions)
            .unwrap_or_default();
    }

    // Migrate the previous single-string format under the same storage key.
    validate_intention(trimmed).ok().into_iter().collect()
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) fn serialize_intentions(intentions: &[String]) -> Option<String> {
    let normalized = validate_intentions(intentions.to_vec())?;
    serde_json::to_string(&normalized).ok()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn load_intentions() -> Vec<String> {
    let saved = web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(INTENTIONS_STORAGE_KEY).ok().flatten());
    resolve_stored_intentions(saved.as_deref())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_intentions() -> Vec<String> {
    resolve_stored_intentions(None)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn save_intentions(intentions: &[String]) -> bool {
    let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    else {
        return false;
    };

    if intentions.is_empty() {
        return storage.remove_item(INTENTIONS_STORAGE_KEY).is_ok();
    }

    let Some(serialized) = serialize_intentions(intentions) else {
        return false;
    };
    storage
        .set_item(INTENTIONS_STORAGE_KEY, &serialized)
        .is_ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_intentions(_intentions: &[String]) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{
        add_intention, move_intention, normalize_intention, remove_intention,
        resolve_stored_intentions, serialize_intentions, validate_intention,
        IntentionValidationError, INTENTIONS_MAX_COUNT, INTENTION_MAX_CHARS,
    };

    #[test]
    fn normalizes_leading_and_trailing_whitespace() {
        assert_eq!(normalize_intention("  For my family.\n"), "For my family.");
    }

    #[test]
    fn accepts_fifty_unicode_scalar_values() {
        let intention = "🙏".repeat(INTENTION_MAX_CHARS);

        assert_eq!(validate_intention(&intention), Ok(intention));
    }

    #[test]
    fn rejects_more_than_fifty_unicode_scalar_values() {
        let intention = "é".repeat(INTENTION_MAX_CHARS + 1);

        assert_eq!(
            validate_intention(&intention),
            Err(IntentionValidationError::TooLong)
        );
    }

    #[test]
    fn adds_normalized_unique_intentions_up_to_the_limit() {
        assert_eq!(
            add_intention(&["Peace".to_owned()], "  Family  "),
            Ok(vec!["Peace".to_owned(), "Family".to_owned()])
        );
        assert_eq!(
            add_intention(&["Peace".to_owned()], "Peace"),
            Err(IntentionValidationError::Duplicate)
        );

        let full = (0..INTENTIONS_MAX_COUNT)
            .map(|index| format!("Intention {index}"))
            .collect::<Vec<_>>();
        assert_eq!(
            add_intention(&full, "One more"),
            Err(IntentionValidationError::TooMany)
        );
    }

    #[test]
    fn removes_and_reorders_intentions_without_losing_values() {
        let intentions = vec!["First".to_owned(), "Second".to_owned(), "Third".to_owned()];

        assert_eq!(
            move_intention(&intentions, 0, 2),
            vec!["Second".to_owned(), "Third".to_owned(), "First".to_owned()]
        );
        assert_eq!(
            remove_intention(&intentions, "Second"),
            vec!["First".to_owned(), "Third".to_owned()]
        );
        assert_eq!(move_intention(&intentions, 8, 0), intentions);
    }

    #[test]
    fn serializes_and_resolves_an_ordered_collection() {
        let intentions = vec!["First".to_owned(), "Second".to_owned()];
        let serialized = serialize_intentions(&intentions).expect("valid intentions serialize");

        assert_eq!(resolve_stored_intentions(Some(&serialized)), intentions);
    }

    #[test]
    fn migrates_the_previous_single_intention_format() {
        assert_eq!(
            resolve_stored_intentions(Some("  Peace in our home  ")),
            vec!["Peace in our home".to_owned()]
        );
    }

    #[test]
    fn malformed_or_invalid_storage_resolves_to_an_empty_collection() {
        let overlong = "x".repeat(INTENTION_MAX_CHARS + 1);

        assert!(resolve_stored_intentions(None).is_empty());
        assert!(resolve_stored_intentions(Some("[not-json]")).is_empty());
        assert!(resolve_stored_intentions(Some(&overlong)).is_empty());
        assert!(resolve_stored_intentions(Some("[\"Same\",\"Same\"]")).is_empty());
    }
}
