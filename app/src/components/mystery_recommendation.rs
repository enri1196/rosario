use crate::i18n::{Language, Translation};
use leptos::prelude::*;

#[component]
pub fn MysteryRecommendation(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
) -> impl IntoView {
    let recommendation = move || recommendation_for(CalendarDate::today(), language.get());

    view! {
        <aside class="mystery-recommendation" aria-labelledby="mystery-recommendation-title">
            <h3 id="mystery-recommendation-title">
                {move || copy.get().mystery_recommendation_title}
            </h3>
            <p class="recommendation-date">
                {move || recommendation().date}
            </p>
            <p class="recommendation-mystery">
                <span class="recommendation-label">
                    {move || copy.get().mystery_recommendation_pray_label}
                </span>
                {move || recommendation().mystery}
            </p>
        </aside>
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CalendarDate {
    year: i32,
    month: u32,
    day: u32,
    weekday: u32,
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
    fn today() -> Self {
        let date = js_sys::Date::new_0();
        Self::new(
            date.get_full_year() as i32,
            date.get_month() as u32 + 1,
            date.get_date() as u32,
            date.get_day() as u32,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn today() -> Self {
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

#[derive(Clone, Copy)]
enum MysterySet {
    Joyful,
    Luminous,
    Sorrowful,
    Glorious,
}

#[derive(Clone, Copy)]
enum Context {
    Weekday(u32),
    Advent,
    Christmas,
    Lent,
    Easter,
    Annunciation,
    SaintJohn,
    Ascension,
    Pentecost,
    CorpusChristi,
    Transfiguration,
    HolyCross,
    Rosary,
    AllSaints,
}

struct Recommendation {
    date: String,
    _context: String,
    mystery: String,
}

fn recommendation_for(date: CalendarDate, language: Language) -> Recommendation {
    let easter = easter_sunday(date.year);
    let (mystery, context) = if is_christmas_period(date) {
        (MysterySet::Joyful, Context::Christmas)
    } else if is_advent(date) {
        (MysterySet::Joyful, Context::Advent)
    } else if let Some((mystery, context)) = special_feast(date, easter) {
        (mystery, context)
    } else if is_between(date, easter, add_days(easter, 49)) {
        (MysterySet::Glorious, Context::Easter)
    } else if is_between(date, add_days(easter, -46), add_days(easter, -1)) {
        (MysterySet::Sorrowful, Context::Lent)
    } else {
        (
            weekday_mystery(date.weekday),
            Context::Weekday(date.weekday),
        )
    };

    Recommendation {
        date: format_date(date, language),
        _context: context_text(context, language).to_owned(),
        mystery: mystery_text(mystery, language).to_owned(),
    }
}

fn weekday_mystery(weekday: u32) -> MysterySet {
    match weekday {
        1 | 6 => MysterySet::Joyful,
        2 | 5 => MysterySet::Sorrowful,
        3 | 0 => MysterySet::Glorious,
        _ => MysterySet::Luminous,
    }
}

fn special_feast(date: CalendarDate, easter: CalendarDate) -> Option<(MysterySet, Context)> {
    if same_day(date, add_days(easter, 39)) {
        return Some((MysterySet::Glorious, Context::Ascension));
    }
    if same_day(date, add_days(easter, 49)) {
        return Some((MysterySet::Glorious, Context::Pentecost));
    }
    if same_day(date, add_days(easter, 60)) {
        return Some((MysterySet::Luminous, Context::CorpusChristi));
    }

    match (date.month, date.day) {
        (3, 25) => Some((MysterySet::Joyful, Context::Annunciation)),
        (6, 24) => Some((MysterySet::Joyful, Context::SaintJohn)),
        (8, 6) => Some((MysterySet::Luminous, Context::Transfiguration)),
        (9, 14) => Some((MysterySet::Sorrowful, Context::HolyCross)),
        (10, 7) => Some((MysterySet::Joyful, Context::Rosary)),
        (11, 1) => Some((MysterySet::Glorious, Context::AllSaints)),
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

fn format_date(date: CalendarDate, language: Language) -> String {
    match language {
        Language::Italian => format!(
            "{} {} {} {}",
            weekday_name(date.weekday, language),
            date.day,
            month_name(date.month, language),
            date.year
        ),
        Language::English => format!(
            "{}, {} {} {}",
            weekday_name(date.weekday, language),
            month_name(date.month, language),
            date.day,
            date.year
        ),
    }
}

fn context_text(context: Context, language: Language) -> &'static str {
    match language {
        Language::Italian => match context {
            Context::Weekday(day) => weekday_name(day, language),
            Context::Advent => "Tempo di Avvento",
            Context::Christmas => "Tempo di Natale",
            Context::Lent => "Tempo di Quaresima",
            Context::Easter => "Tempo di Pasqua",
            Context::Annunciation => "Annunciazione del Signore",
            Context::SaintJohn => "Natività di San Giovanni Battista",
            Context::Ascension => "Ascensione del Signore",
            Context::Pentecost => "Pentecoste",
            Context::CorpusChristi => "Corpus Domini",
            Context::Transfiguration => "Trasfigurazione del Signore",
            Context::HolyCross => "Esaltazione della Santa Croce",
            Context::Rosary => "Beata Vergine del Rosario",
            Context::AllSaints => "Tutti i Santi",
        },
        Language::English => match context {
            Context::Weekday(day) => weekday_name(day, language),
            Context::Advent => "Advent",
            Context::Christmas => "Christmas season",
            Context::Lent => "Lent",
            Context::Easter => "Easter season",
            Context::Annunciation => "Annunciation of the Lord",
            Context::SaintJohn => "Nativity of Saint John the Baptist",
            Context::Ascension => "Ascension of the Lord",
            Context::Pentecost => "Pentecost",
            Context::CorpusChristi => "Corpus Christi",
            Context::Transfiguration => "Transfiguration of the Lord",
            Context::HolyCross => "Exaltation of the Holy Cross",
            Context::Rosary => "Our Lady of the Rosary",
            Context::AllSaints => "All Saints",
        },
    }
}

fn mystery_text(mystery: MysterySet, language: Language) -> &'static str {
    match (language, mystery) {
        (Language::Italian, MysterySet::Joyful) => "Misteri Gaudiosi",
        (Language::Italian, MysterySet::Luminous) => "Misteri Luminosi",
        (Language::Italian, MysterySet::Sorrowful) => "Misteri Dolorosi",
        (Language::Italian, MysterySet::Glorious) => "Misteri Gloriosi",
        (Language::English, MysterySet::Joyful) => "Joyful Mysteries",
        (Language::English, MysterySet::Luminous) => "Luminous Mysteries",
        (Language::English, MysterySet::Sorrowful) => "Sorrowful Mysteries",
        (Language::English, MysterySet::Glorious) => "Glorious Mysteries",
    }
}

fn weekday_name(weekday: u32, language: Language) -> &'static str {
    match (language, weekday) {
        (Language::Italian, 0) => "domenica",
        (Language::Italian, 1) => "lunedì",
        (Language::Italian, 2) => "martedì",
        (Language::Italian, 3) => "mercoledì",
        (Language::Italian, 4) => "giovedì",
        (Language::Italian, 5) => "venerdì",
        (Language::Italian, 6) => "sabato",
        (Language::English, 0) => "Sunday",
        (Language::English, 1) => "Monday",
        (Language::English, 2) => "Tuesday",
        (Language::English, 3) => "Wednesday",
        (Language::English, 4) => "Thursday",
        (Language::English, 5) => "Friday",
        (Language::English, 6) => "Saturday",
        _ => "",
    }
}

fn month_name(month: u32, language: Language) -> &'static str {
    match (language, month) {
        (Language::Italian, 1) => "gennaio",
        (Language::Italian, 2) => "febbraio",
        (Language::Italian, 3) => "marzo",
        (Language::Italian, 4) => "aprile",
        (Language::Italian, 5) => "maggio",
        (Language::Italian, 6) => "giugno",
        (Language::Italian, 7) => "luglio",
        (Language::Italian, 8) => "agosto",
        (Language::Italian, 9) => "settembre",
        (Language::Italian, 10) => "ottobre",
        (Language::Italian, 11) => "novembre",
        (Language::Italian, 12) => "dicembre",
        (Language::English, 1) => "January",
        (Language::English, 2) => "February",
        (Language::English, 3) => "March",
        (Language::English, 4) => "April",
        (Language::English, 5) => "May",
        (Language::English, 6) => "June",
        (Language::English, 7) => "July",
        (Language::English, 8) => "August",
        (Language::English, 9) => "September",
        (Language::English, 10) => "October",
        (Language::English, 11) => "November",
        (Language::English, 12) => "December",
        _ => "",
    }
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
        let recommendation =
            recommendation_for(CalendarDate::new(2026, 2, 18, 3), Language::English);
        assert_eq!(recommendation.mystery, "Sorrowful Mysteries");
    }

    #[test]
    fn recommends_joyful_mysteries_during_advent() {
        let recommendation =
            recommendation_for(CalendarDate::new(2026, 12, 1, 2), Language::English);
        assert_eq!(recommendation.mystery, "Joyful Mysteries");
    }

    #[test]
    fn recommends_ten_small_bead_boundaries_around_easter() {
        assert!(is_between(
            CalendarDate::new(2026, 4, 5, 0),
            easter_sunday(2026),
            add_days(easter_sunday(2026), 49)
        ));
    }
}
