use crate::i18n::MysterySet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CalendarDate {
    pub(crate) year: i32,
    pub(crate) month: u32,
    pub(crate) day: u32,
    pub(crate) weekday: u32,
}

impl CalendarDate {
    const fn new(year: i32, month: u32, day: u32, weekday: u32) -> Self {
        Self {
            year,
            month,
            day,
            weekday,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn today() -> Self {
        let date = js_sys::Date::new_0();
        Self::new(
            date.get_full_year() as i32,
            date.get_month() as u32 + 1,
            date.get_date() as u32,
            date.get_day() as u32,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn today() -> Self {
        // The app is mounted in the browser; this value only keeps non-WASM
        // checks and the server-side crate build deterministic.
        Self::new(2026, 1, 1, 4)
    }

    fn serial(self) -> i64 {
        let year = i64::from(self.year - 1);
        let leap_days = year / 4 - year / 100 + year / 400;
        let days_before_year = year * 365 + leap_days;
        days_before_year + i64::from(ordinal_day(self.year, self.month, self.day)) - 1
    }
}

pub(crate) struct Recommendation {
    pub(crate) date: CalendarDate,
    pub(crate) mystery: MysterySet,
}

pub(crate) fn recommendation_for(date: CalendarDate) -> Recommendation {
    let easter = easter_sunday(date.year);
    let mystery = if is_christmas_period(date) || is_advent(date) {
        MysterySet::Joyful
    } else if let Some(mystery) = special_feast(date, easter) {
        mystery
    } else if is_between(date, easter, add_days(easter, 49)) {
        MysterySet::Glorious
    } else if is_between(date, add_days(easter, -46), add_days(easter, -1)) {
        MysterySet::Sorrowful
    } else {
        weekday_mystery(date.weekday)
    };

    Recommendation { date, mystery }
}

fn weekday_mystery(weekday: u32) -> MysterySet {
    match weekday {
        1 | 6 => MysterySet::Joyful,
        2 | 5 => MysterySet::Sorrowful,
        3 | 0 => MysterySet::Glorious,
        _ => MysterySet::Luminous,
    }
}

fn special_feast(date: CalendarDate, easter: CalendarDate) -> Option<MysterySet> {
    if same_day(date, add_days(easter, 39)) || same_day(date, add_days(easter, 49)) {
        return Some(MysterySet::Glorious);
    }
    if same_day(date, add_days(easter, 60)) {
        return Some(MysterySet::Luminous);
    }

    match (date.month, date.day) {
        (3, 25) | (6, 24) | (10, 7) => Some(MysterySet::Joyful),
        (8, 6) => Some(MysterySet::Luminous),
        (9, 14) => Some(MysterySet::Sorrowful),
        (11, 1) => Some(MysterySet::Glorious),
        _ => None,
    }
}

fn is_christmas_period(date: CalendarDate) -> bool {
    (date.month == 12 && date.day >= 25) || (date.month == 1 && date.day <= 6)
}

fn is_advent(date: CalendarDate) -> bool {
    let christmas = CalendarDate::new(date.year, 12, 25, 0);
    is_between(date, advent_start(christmas), add_days(christmas, -1))
}

fn advent_start(christmas: CalendarDate) -> CalendarDate {
    let christmas_weekday = weekday_for(christmas);
    let days_before_christmas = if christmas_weekday == 0 {
        28
    } else {
        christmas_weekday + 21
    };
    add_days(christmas, -(days_before_christmas as i32))
}

fn easter_sunday(year: i32) -> CalendarDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;

    let date = CalendarDate::new(year, month as u32, day as u32, 0);
    CalendarDate::new(date.year, date.month, date.day, weekday_for(date))
}

fn add_days(mut date: CalendarDate, mut days: i32) -> CalendarDate {
    while days > 0 {
        date.day += 1;
        if date.day > days_in_month(date.year, date.month) {
            date.day = 1;
            date.month += 1;
            if date.month > 12 {
                date.month = 1;
                date.year += 1;
            }
        }
        days -= 1;
    }
    while days < 0 {
        if date.day == 1 {
            if date.month == 1 {
                date.month = 12;
                date.year -= 1;
            } else {
                date.month -= 1;
            }
            date.day = days_in_month(date.year, date.month);
        } else {
            date.day -= 1;
        }
        days += 1;
    }
    date.weekday = weekday_for(date);
    date
}

fn is_between(date: CalendarDate, start: CalendarDate, end: CalendarDate) -> bool {
    date.serial() >= start.serial() && date.serial() <= end.serial()
}

fn same_day(left: CalendarDate, right: CalendarDate) -> bool {
    left.year == right.year && left.month == right.month && left.day == right.day
}

fn weekday_for(date: CalendarDate) -> u32 {
    let epoch = CalendarDate::new(1970, 1, 1, 0).serial();
    (date.serial() - epoch + 4).rem_euclid(7) as u32
}

fn ordinal_day(year: i32, month: u32, day: u32) -> u32 {
    (1..month)
        .map(|previous_month| days_in_month(year, previous_month))
        .sum::<u32>()
        + day
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_gregorian_easter() {
        assert!(same_day(
            easter_sunday(2026),
            CalendarDate::new(2026, 4, 5, 0)
        ));
    }

    #[test]
    fn recommends_sorrowful_mysteries_during_lent() {
        let recommendation = recommendation_for(CalendarDate::new(2026, 2, 18, 3));
        assert!(matches!(recommendation.mystery, MysterySet::Sorrowful));
    }

    #[test]
    fn recommends_joyful_mysteries_during_advent() {
        let recommendation = recommendation_for(CalendarDate::new(2026, 12, 1, 2));
        assert!(matches!(recommendation.mystery, MysterySet::Joyful));
    }

    #[test]
    fn recognizes_the_easter_season() {
        let date = CalendarDate::new(2026, 4, 5, 0);
        let recommendation = recommendation_for(date);
        assert!(matches!(recommendation.mystery, MysterySet::Glorious));
    }
}
