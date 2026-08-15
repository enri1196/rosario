use crate::i18n::MysterySet;

pub(crate) const GUIDED_STEP_COUNT: usize = 31;
const OPENING_STEP_COUNT: usize = 5;
const STEPS_PER_DECADE: usize = 5;

/// Number of guided prayer phases represented inside each decade.
pub(crate) const DECADE_PRAYER_COUNT: usize = STEPS_PER_DECADE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OpeningStep {
    SignOfCross,
    Creed,
    OurFather,
    ThreeHailMarys,
    GloryBe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DecadePrayer {
    Mystery,
    OurFather,
    TenHailMarys,
    GloryBe,
    FatimaPrayer,
}

impl DecadePrayer {
    /// Returns the one-based position of this prayer phase within a decade.
    pub(crate) const fn number(self) -> usize {
        match self {
            Self::Mystery => 1,
            Self::OurFather => 2,
            Self::TenHailMarys => 3,
            Self::GloryBe => 4,
            Self::FatimaPrayer => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Decade {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Decade {
    pub(crate) const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::First),
            1 => Some(Self::Second),
            2 => Some(Self::Third),
            3 => Some(Self::Fourth),
            4 => Some(Self::Fifth),
            _ => None,
        }
    }

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::First => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Fifth => 4,
        }
    }

    pub(crate) const fn number(self) -> usize {
        self.index() + 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GuidedStep {
    Opening(OpeningStep),
    Decade {
        decade: Decade,
        prayer: DecadePrayer,
    },
    Closing,
}

impl GuidedStep {
    pub(crate) const fn decade(self) -> Option<Decade> {
        match self {
            Self::Decade { decade, .. } => Some(decade),
            Self::Opening(_) | Self::Closing => None,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Opening(step) => match step {
                OpeningStep::SignOfCross => 0,
                OpeningStep::Creed => 1,
                OpeningStep::OurFather => 2,
                OpeningStep::ThreeHailMarys => 3,
                OpeningStep::GloryBe => 4,
            },
            Self::Decade { decade, prayer } => {
                let prayer_index = match prayer {
                    DecadePrayer::Mystery => 0,
                    DecadePrayer::OurFather => 1,
                    DecadePrayer::TenHailMarys => 2,
                    DecadePrayer::GloryBe => 3,
                    DecadePrayer::FatimaPrayer => 4,
                };
                OPENING_STEP_COUNT + decade.index() * STEPS_PER_DECADE + prayer_index
            }
            Self::Closing => GUIDED_STEP_COUNT - 1,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Opening(OpeningStep::SignOfCross),
            1 => Self::Opening(OpeningStep::Creed),
            2 => Self::Opening(OpeningStep::OurFather),
            3 => Self::Opening(OpeningStep::ThreeHailMarys),
            4 => Self::Opening(OpeningStep::GloryBe),
            index if index < GUIDED_STEP_COUNT - 1 => {
                let decade_step = index - OPENING_STEP_COUNT;
                let decade = Decade::from_index(decade_step / STEPS_PER_DECADE)
                    .expect("guided sequence has exactly five decades");
                let prayer = match decade_step % STEPS_PER_DECADE {
                    0 => DecadePrayer::Mystery,
                    1 => DecadePrayer::OurFather,
                    2 => DecadePrayer::TenHailMarys,
                    3 => DecadePrayer::GloryBe,
                    _ => DecadePrayer::FatimaPrayer,
                };
                Self::Decade { decade, prayer }
            }
            _ => Self::Closing,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SessionCompletion {
    InProgress,
    Complete,
}

/// Typed decade and prayer-phase progress for the active guided step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DecadeProgress {
    decade: Decade,
    prayer: DecadePrayer,
}

impl DecadeProgress {
    /// Returns the one-based decade number.
    pub(crate) const fn decade_number(self) -> usize {
        self.decade.number()
    }

    /// Returns the one-based prayer-phase number within the active decade.
    pub(crate) const fn prayer_number(self) -> usize {
        self.prayer.number()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RosarySession {
    active_step: GuidedStep,
    selected_mystery_set: MysterySet,
    focused_mystery: Option<Decade>,
    completion: SessionCompletion,
}

impl RosarySession {
    pub(crate) const fn start(selected_mystery_set: MysterySet) -> Self {
        Self {
            active_step: GuidedStep::Opening(OpeningStep::SignOfCross),
            selected_mystery_set,
            focused_mystery: None,
            completion: SessionCompletion::InProgress,
        }
    }

    pub(crate) const fn start_for_mystery(
        selected_mystery_set: MysterySet,
        focused_mystery: Decade,
    ) -> Self {
        Self {
            focused_mystery: Some(focused_mystery),
            ..Self::start(selected_mystery_set)
        }
    }

    pub(crate) fn next(&mut self) {
        if self.is_complete() {
            return;
        }

        if self.is_last_step() {
            self.complete();
        } else {
            self.active_step = GuidedStep::from_index(self.active_step.index() + 1);
        }
    }

    pub(crate) fn previous(&mut self) {
        if self.is_complete() {
            self.completion = SessionCompletion::InProgress;
            return;
        }

        self.active_step = GuidedStep::from_index(self.active_step.index().saturating_sub(1));
    }

    pub(crate) fn reset(&mut self) {
        self.active_step = GuidedStep::Opening(OpeningStep::SignOfCross);
        self.completion = SessionCompletion::InProgress;
    }

    pub(crate) fn complete(&mut self) {
        if self.is_last_step() {
            self.completion = SessionCompletion::Complete;
        }
    }

    pub(crate) const fn active_step(self) -> GuidedStep {
        self.active_step
    }

    pub(crate) const fn active_decade(self) -> Option<Decade> {
        self.active_step.decade()
    }

    /// Returns typed decade-level progress when the active step belongs to a decade.
    pub(crate) const fn active_decade_progress(self) -> Option<DecadeProgress> {
        let Some(decade) = self.active_decade() else {
            return None;
        };
        let GuidedStep::Decade { prayer, .. } = self.active_step else {
            return None;
        };

        Some(DecadeProgress { decade, prayer })
    }

    pub(crate) const fn selected_mystery_set(self) -> MysterySet {
        self.selected_mystery_set
    }

    pub(crate) const fn focused_mystery(self) -> Option<Decade> {
        self.focused_mystery
    }

    pub(crate) const fn step_number(self) -> usize {
        self.active_step.index() + 1
    }

    pub(crate) const fn is_first_step(self) -> bool {
        self.active_step.index() == 0
    }

    pub(crate) const fn is_last_step(self) -> bool {
        self.active_step.index() == GUIDED_STEP_COUNT - 1
    }

    pub(crate) const fn is_complete(self) -> bool {
        matches!(self.completion, SessionCompletion::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Decade, DecadePrayer, GuidedStep, OpeningStep, RosarySession, DECADE_PRAYER_COUNT,
        GUIDED_STEP_COUNT, OPENING_STEP_COUNT, STEPS_PER_DECADE,
    };
    use crate::i18n::MysterySet;

    #[test]
    fn starts_at_the_first_opening_step() {
        let session = RosarySession::start(MysterySet::Joyful);

        assert_eq!(
            session.active_step(),
            GuidedStep::Opening(OpeningStep::SignOfCross)
        );
        assert_eq!(session.step_number(), 1);
        assert_eq!(session.active_decade(), None);
        assert!(!session.is_complete());
    }

    #[test]
    fn enters_each_decade_in_order() {
        let mut session = RosarySession::start(MysterySet::Luminous);

        for expected_decade in [
            Decade::First,
            Decade::Second,
            Decade::Third,
            Decade::Fourth,
            Decade::Fifth,
        ] {
            while session.active_decade() != Some(expected_decade) {
                session.next();
            }

            assert_eq!(session.active_decade(), Some(expected_decade));
            for _ in 1..STEPS_PER_DECADE {
                session.next();
                assert_eq!(session.active_decade(), Some(expected_decade));
            }
        }
    }

    #[test]
    fn reports_each_decade_prayer_position() {
        let mut session = RosarySession::start(MysterySet::Luminous);
        let prayers = [
            DecadePrayer::Mystery,
            DecadePrayer::OurFather,
            DecadePrayer::TenHailMarys,
            DecadePrayer::GloryBe,
            DecadePrayer::FatimaPrayer,
        ];

        assert_eq!(session.active_decade_progress(), None);
        for _ in 0..OPENING_STEP_COUNT {
            session.next();
        }

        for decade_number in 1..=5 {
            for (prayer_index, expected_prayer) in prayers.into_iter().enumerate() {
                assert_eq!(
                    session.active_step(),
                    GuidedStep::Decade {
                        decade: Decade::from_index(decade_number - 1).unwrap(),
                        prayer: expected_prayer,
                    }
                );
                let progress = session.active_decade_progress().unwrap();
                assert_eq!(progress.decade_number(), decade_number);
                assert_eq!(progress.prayer_number(), prayer_index + 1);
                assert!(progress.prayer_number() <= DECADE_PRAYER_COUNT);
                session.next();
            }
        }

        assert_eq!(session.active_step(), GuidedStep::Closing);
        assert_eq!(session.active_decade_progress(), None);
    }

    #[test]
    fn previous_and_next_are_clamped_at_boundaries() {
        let mut session = RosarySession::start(MysterySet::Sorrowful);
        session.previous();
        assert!(session.is_first_step());

        for _ in 1..GUIDED_STEP_COUNT {
            session.next();
        }
        assert!(session.is_last_step());
        assert!(!session.is_complete());

        session.next();
        assert!(session.is_complete());
        session.next();
        assert!(session.is_complete());
        assert!(session.is_last_step());
    }

    #[test]
    fn reset_preserves_the_selected_mystery() {
        let mut session = RosarySession::start_for_mystery(MysterySet::Glorious, Decade::Third);
        session.next();
        session.next();
        session.reset();

        assert!(session.is_first_step());
        assert_eq!(session.selected_mystery_set(), MysterySet::Glorious);
        assert_eq!(session.focused_mystery(), Some(Decade::Third));
        assert!(!session.is_complete());
    }

    #[test]
    fn completion_requires_the_closing_step() {
        let mut session = RosarySession::start(MysterySet::Joyful);
        session.complete();
        assert!(!session.is_complete());

        for _ in 1..GUIDED_STEP_COUNT {
            session.next();
        }
        session.complete();
        assert!(session.is_complete());

        session.previous();
        assert!(!session.is_complete());
        assert!(session.is_last_step());
    }
}
