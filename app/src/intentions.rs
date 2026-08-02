#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) const INTENTIONS_STORAGE_KEY: &str = "rosary-intentions";
pub(crate) const INTENTION_MAX_CHARS: usize = 500;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IntentionValidationError {
    Empty,
    TooLong,
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

pub(crate) fn resolve_stored_intention(saved: Option<&str>) -> Option<String> {
    saved.and_then(|value| validate_intention(value).ok())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn load_intention() -> Option<String> {
    let saved = web_sys::window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(INTENTIONS_STORAGE_KEY)
        .ok()
        .flatten();
    resolve_stored_intention(saved.as_deref())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_intention() -> Option<String> {
    resolve_stored_intention(None)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn save_intention(value: &str) -> bool {
    let Ok(normalized) = validate_intention(value) else {
        return false;
    };
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .is_some_and(|storage| {
            storage
                .set_item(INTENTIONS_STORAGE_KEY, &normalized)
                .is_ok()
        })
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn save_intention(_value: &str) -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn clear_intention() -> bool {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .is_some_and(|storage| storage.remove_item(INTENTIONS_STORAGE_KEY).is_ok())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn clear_intention() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_intention, resolve_stored_intention, validate_intention,
        IntentionValidationError, INTENTION_MAX_CHARS,
    };

    #[test]
    fn normalizes_leading_and_trailing_whitespace() {
        assert_eq!(normalize_intention("  For my family.\n"), "For my family.");
    }

    #[test]
    fn accepts_five_hundred_unicode_scalar_values() {
        let intention = "🙏".repeat(INTENTION_MAX_CHARS);

        assert_eq!(validate_intention(&intention), Ok(intention));
    }

    #[test]
    fn rejects_more_than_five_hundred_unicode_scalar_values() {
        let intention = "é".repeat(INTENTION_MAX_CHARS + 1);

        assert_eq!(
            validate_intention(&intention),
            Err(IntentionValidationError::TooLong)
        );
    }

    #[test]
    fn rejects_an_empty_normalized_intention() {
        assert_eq!(
            validate_intention(" \n\t "),
            Err(IntentionValidationError::Empty)
        );
    }

    #[test]
    fn resolves_missing_empty_and_malformed_storage_as_no_intention() {
        let malformed = "x".repeat(INTENTION_MAX_CHARS + 1);

        assert_eq!(resolve_stored_intention(None), None);
        assert_eq!(resolve_stored_intention(Some("  ")), None);
        assert_eq!(resolve_stored_intention(Some(&malformed)), None);
    }

    #[test]
    fn resolves_and_normalizes_a_valid_saved_intention() {
        assert_eq!(
            resolve_stored_intention(Some("  Peace in our home  ")),
            Some("Peace in our home".to_owned())
        );
    }
}
