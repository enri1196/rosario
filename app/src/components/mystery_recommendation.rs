use super::GuideBox;
use crate::calendar::{recommendation_for, CalendarDate};
use crate::i18n::{Language, Translation};
use leptos::prelude::*;

#[component]
pub fn MysteryRecommendation(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
) -> impl IntoView {
    let recommendation = move || {
        let recommendation = recommendation_for(CalendarDate::today());
        let current_language = language.get();
        DisplayRecommendation {
            date: format_date(recommendation.date, current_language),
            mystery: recommendation.mystery.label(current_language).to_owned(),
        }
    };

    view! {
        <GuideBox>
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
        </GuideBox>
    }
}

struct DisplayRecommendation {
    date: String,
    mystery: String,
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
