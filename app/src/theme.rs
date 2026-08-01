//! Models the effective UI theme separately from browser persistence.
//!
//! The pure [`Theme`] behavior is available to host and browser builds, while
//! browser APIs are confined to the synchronization helpers in this module.

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
}
