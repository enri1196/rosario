/// A top-level section in the single-page Rosario workspace.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AppSection {
    /// The intention editor and Rosary overview or active guided session.
    #[default]
    Guide,
    /// The four groups of Rosary mysteries.
    Mysteries,
    /// The reusable Rosary prayer collection.
    Prayers,
}

impl AppSection {
    /// All workspace sections in their visual and keyboard-navigation order.
    pub(crate) const ALL: [Self; 3] = [Self::Guide, Self::Mysteries, Self::Prayers];

    /// Returns the stable ID of this section's tab control.
    pub(crate) const fn control_id(self) -> &'static str {
        match self {
            Self::Guide => "guide-tab",
            Self::Mysteries => "mysteries-tab",
            Self::Prayers => "prayers-tab",
        }
    }

    /// Returns the stable ID of this section's tab panel.
    pub(crate) const fn panel_id(self) -> &'static str {
        match self {
            Self::Guide => "guide-panel",
            Self::Mysteries => "mysteries-panel",
            Self::Prayers => "prayers-panel",
        }
    }

    /// Returns the stable ID of this section's focusable visible heading.
    pub(crate) const fn heading_id(self) -> &'static str {
        match self {
            Self::Guide => "guide-heading",
            Self::Mysteries => "mysteries-heading",
            Self::Prayers => "prayers-heading",
        }
    }

    /// Returns the preceding section, wrapping at the start of the tab list.
    pub(crate) const fn previous(self) -> Self {
        match self {
            Self::Guide => Self::Prayers,
            Self::Mysteries => Self::Guide,
            Self::Prayers => Self::Mysteries,
        }
    }

    /// Returns the following section, wrapping at the end of the tab list.
    pub(crate) const fn next(self) -> Self {
        match self {
            Self::Guide => Self::Mysteries,
            Self::Mysteries => Self::Prayers,
            Self::Prayers => Self::Guide,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppSection;

    #[test]
    fn guide_is_the_default_section() {
        assert_eq!(AppSection::default(), AppSection::Guide);
    }

    #[test]
    fn section_ids_are_stable_and_unique() {
        assert_eq!(AppSection::Guide.control_id(), "guide-tab");
        assert_eq!(AppSection::Mysteries.panel_id(), "mysteries-panel");
        assert_eq!(AppSection::Prayers.heading_id(), "prayers-heading");

        for (index, section) in AppSection::ALL.iter().enumerate() {
            for other in AppSection::ALL.iter().skip(index + 1) {
                assert_ne!(section.control_id(), other.control_id());
                assert_ne!(section.panel_id(), other.panel_id());
                assert_ne!(section.heading_id(), other.heading_id());
            }
        }
    }

    #[test]
    fn section_navigation_wraps_in_both_directions() {
        assert_eq!(AppSection::Guide.previous(), AppSection::Prayers);
        assert_eq!(AppSection::Guide.next(), AppSection::Mysteries);
        assert_eq!(AppSection::Prayers.next(), AppSection::Guide);
    }
}
