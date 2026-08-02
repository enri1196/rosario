use super::GuideBox;
use crate::calendar::{recommendation_for, CalendarDate, RecommendationBasis};
use crate::i18n::{Language, RecommendationBasisDescriptions, Translation};
use leptos::prelude::*;

#[component]
pub fn MysteryRecommendation(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
) -> impl IntoView {
    let selected_date = RwSignal::new(CalendarDate::today());
    let invalid_date = RwSignal::new(false);

    let recommendation = move || {
        let recommendation = recommendation_for(selected_date.get());
        let current_language = language.get();
        let current_copy = copy.get();
        DisplayRecommendation {
            date: format_date(recommendation.date, current_language),
            mystery: recommendation.mystery.label(current_language).to_owned(),
            reason: basis_description(
                recommendation.basis,
                current_copy.mystery_recommendation_basis,
            ),
        }
    };

    view! {
        <GuideBox>
            <aside class="mystery-recommendation" aria-labelledby="mystery-recommendation-title">
                <h3 id="mystery-recommendation-title">
                    {move || copy.get().mystery_recommendation_title}
                </h3>
                <div class="recommendation-picker">
                    <label for="mystery-recommendation-date">
                        {move || copy.get().mystery_recommendation_date_label}
                    </label>
                    <div class="recommendation-controls">
                        <input
                            id="mystery-recommendation-date"
                            type="date"
                            prop:value=move || selected_date.get().to_input_value()
                            aria-describedby=move || if invalid_date.get() {
                                "mystery-recommendation-date-help mystery-recommendation-date-error"
                            } else {
                                "mystery-recommendation-date-help"
                            }
                            aria-invalid=move || invalid_date.get().to_string()
                            on:input=move |event| {
                                if let Some(date) = CalendarDate::from_input_value(&event_target_value(&event)) {
                                    selected_date.set(date);
                                    invalid_date.set(false);
                                } else {
                                    invalid_date.set(true);
                                }
                            }
                        />
                        <button
                            type="button"
                            on:click=move |_| {
                                selected_date.set(CalendarDate::today());
                                invalid_date.set(false);
                            }
                        >
                            {move || copy.get().mystery_recommendation_today_label}
                        </button>
                    </div>
                    <p id="mystery-recommendation-date-help" class="recommendation-help">
                        {move || copy.get().mystery_recommendation_date_help}
                    </p>
                    <Show when=move || invalid_date.get()>
                        <p
                            id="mystery-recommendation-date-error"
                            class="recommendation-error"
                            role="alert"
                        >
                            {move || copy.get().mystery_recommendation_invalid_date}
                        </p>
                    </Show>
                </div>
                <div class="recommendation-result" aria-live="polite" aria-atomic="true">
                    <p class="recommendation-date">
                        <span class="recommendation-label">
                            {move || copy.get().mystery_recommendation_selected_date_label}
                        </span>
                        {move || recommendation().date}
                    </p>
                    <p class="recommendation-mystery">
                        <span class="recommendation-label">
                            {move || copy.get().mystery_recommendation_pray_label}
                        </span>
                        {move || recommendation().mystery}
                    </p>
                    <p class="recommendation-reason">
                        <span class="recommendation-label">
                            {move || copy.get().mystery_recommendation_reason_label}
                        </span>
                        {move || recommendation().reason}
                    </p>
                </div>
            </aside>
        </GuideBox>
    }
}

struct DisplayRecommendation {
    date: String,
    mystery: String,
    reason: &'static str,
}

fn basis_description(
    basis: RecommendationBasis,
    descriptions: RecommendationBasisDescriptions,
) -> &'static str {
    match basis {
        RecommendationBasis::Weekday => descriptions.weekday,
        RecommendationBasis::Advent => descriptions.advent,
        RecommendationBasis::ChristmasPeriod => descriptions.christmas_period,
        RecommendationBasis::Lent => descriptions.lent,
        RecommendationBasis::EasterSeason => descriptions.easter_season,
        RecommendationBasis::FeastOverride => descriptions.feast_override,
    }
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
