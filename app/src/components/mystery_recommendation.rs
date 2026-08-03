use super::{AppButton, GuideBox};
use crate::calendar::{recommendation_for, CalendarDate, RecommendationBasis};
use crate::i18n::{Language, RecommendationBasisDescriptions, Translation};
use leptos::prelude::*;

#[component]
pub fn MysteryRecommendation(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
) -> impl IntoView {
    let initial_date = CalendarDate::today();
    let selected_date = RwSignal::new(initial_date);
    let date_input_value = RwSignal::new(initial_date.to_input_value());
    let invalid_date = RwSignal::new(false);

    let recommendation = move || {
        let recommendation = recommendation_for(selected_date.get());
        let current_language = language.get();
        let current_copy = copy.get();
        DisplayRecommendation {
            date: recommendation.date.to_input_value(),
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
                            type="text"
                            inputmode="numeric"
                            autocomplete="off"
                            spellcheck="false"
                            maxlength="10"
                            pattern="[0-9]{4}-[0-9]{2}-[0-9]{2}"
                            placeholder="YYYY-MM-DD"
                            prop:value=move || date_input_value.get()
                            aria-describedby=move || if invalid_date.get() {
                                "mystery-recommendation-date-help mystery-recommendation-date-error"
                            } else {
                                "mystery-recommendation-date-help"
                            }
                            aria-invalid=move || invalid_date.get().to_string()
                            on:input=move |event| {
                                let value = event_target_value(&event);
                                date_input_value.set(value.clone());
                                if let Some(date) = CalendarDate::from_input_value(&value) {
                                    selected_date.set(date);
                                    invalid_date.set(false);
                                } else {
                                    invalid_date.set(true);
                                }
                            }
                        />
                        <AppButton
                            aria_label=move || copy.get().mystery_recommendation_today_label
                            on_click=move |_| {
                                let today = CalendarDate::today();
                                selected_date.set(today);
                                date_input_value.set(today.to_input_value());
                                invalid_date.set(false);
                            }
                        >
                            {move || copy.get().mystery_recommendation_today_label}
                        </AppButton>
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
                        {" "}
                        {move || recommendation().date}
                    </p>
                    <p class="recommendation-mystery">
                        <span class="recommendation-label">
                            {move || copy.get().mystery_recommendation_pray_label}
                        </span>
                        {" "}
                        {move || recommendation().mystery}
                    </p>
                    <p class="recommendation-reason">
                        <span class="recommendation-label">
                            {move || copy.get().mystery_recommendation_reason_label}
                        </span>
                        {" "}
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
