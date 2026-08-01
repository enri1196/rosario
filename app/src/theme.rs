//! Models the effective UI theme separately from browser persistence.
//!
//! The pure [`Theme`] behavior is available to host and browser builds, while
//! browser APIs are confined to the synchronization helpers in this module.

use web_sys::Document;

/// The storage key used for an explicitly selected theme.
pub(crate) const THEME_STORAGE_KEY: &str = "rosary-theme";

/// The effective color theme displayed by the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Theme {
    /// The dark, jewel-toned presentation.
    Dark,
    /// The light, warm-paper presentation.
    Light,
}

impl Theme {
    /// Resolves the initial effective theme from storage and then system media.
    ///
    /// Invalid preferences and unavailable browser APIs are deliberately
    /// non-fatal so theme initialization can never prevent the UI rendering.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_browser() -> Self {
        let Some(window) = web_sys::window() else {
            return Self::Dark;
        };

        let saved_theme = window
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok().flatten());
        let prefers_light = window
            .match_media("(prefers-color-scheme: light)")
            .ok()
            .flatten()
            .map(|query| query.matches());

        Self::resolve(saved_theme.as_deref(), prefers_light)
    }

    /// Returns the dark fallback when browser APIs are unavailable on a host.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn from_browser() -> Self {
        Self::resolve(None, None)
    }

    /// Resolves stored and media-query inputs into an effective theme.
    ///
    /// A valid explicit preference wins; otherwise a light media match selects
    /// light mode and every missing or false input safely falls back to dark.
    pub(crate) fn resolve(saved_theme: Option<&str>, prefers_light: Option<bool>) -> Self {
        saved_theme.and_then(Self::from_attribute).unwrap_or(
            if matches!(prefers_light, Some(true)) {
                Self::Light
            } else {
                Self::Dark
            },
        )
    }

    /// Returns the value stored in and applied through `data-theme`.
    ///
    /// An HTML attribute is used instead of a class so theme state is explicit
    /// and cannot be confused with presentational component classes.
    pub(crate) const fn as_attribute(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    /// Parses a supported `data-theme` or stored preference value.
    pub(crate) const fn from_attribute(value: &str) -> Option<Self> {
        match value.as_bytes() {
            b"dark" => Some(Self::Dark),
            b"light" => Some(Self::Light),
            _ => None,
        }
    }

    /// Returns the opposite effective theme.
    pub(crate) const fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

/// Applies the effective theme to the document root.
///
/// The result is ignored because a missing or locked document element must not
/// prevent the rest of the client application from rendering.
pub(crate) fn apply_theme(theme: Theme, document: &Document) {
    if let Some(root) = document.document_element() {
        let _ = root.set_attribute("data-theme", theme.as_attribute());
    }
}

/// Persists an explicit user choice when browser storage is available.
///
/// Storage can be blocked or unavailable in private browsing contexts, so all
/// failures are intentionally ignored.
#[cfg(target_arch = "wasm32")]
pub(crate) fn persist_theme(theme: Theme) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme.as_attribute());
    }
}

/// Does nothing when persistence is requested outside a browser build.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn persist_theme(_theme: Theme) {
    let _ = THEME_STORAGE_KEY;
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn parses_supported_attribute_values() {
        assert_eq!(Theme::from_attribute("dark"), Some(Theme::Dark));
        assert_eq!(Theme::from_attribute("light"), Some(Theme::Light));
    }

    #[test]
    fn rejects_invalid_attribute_values() {
        assert_eq!(Theme::from_attribute(""), None);
        assert_eq!(Theme::from_attribute("Dark"), None);
        assert_eq!(Theme::from_attribute("system"), None);
    }

    #[test]
    fn toggles_between_effective_themes() {
        assert_eq!(Theme::Dark.toggle(), Theme::Light);
        assert_eq!(Theme::Light.toggle(), Theme::Dark);
    }

    #[test]
    fn saved_theme_wins_over_media_preference() {
        assert_eq!(Theme::resolve(Some("dark"), Some(true)), Theme::Dark);
        assert_eq!(Theme::resolve(Some("light"), Some(false)), Theme::Light);
    }

    #[test]
    fn invalid_or_missing_storage_uses_media_preference() {
        assert_eq!(Theme::resolve(Some("system"), Some(true)), Theme::Light);
        assert_eq!(Theme::resolve(None, Some(true)), Theme::Light);
        assert_eq!(Theme::resolve(None, Some(false)), Theme::Dark);
    }

    #[test]
    fn unavailable_browser_inputs_fall_back_to_dark() {
        assert_eq!(Theme::resolve(None, None), Theme::Dark);
        assert_eq!(Theme::resolve(Some(""), None), Theme::Dark);
    }
}
