use super::AppButton;
use crate::i18n::{Mystery, MysterySet};
use crate::rosary_session::Decade;
use leptos::prelude::*;

/// Renders one mystery and delegates guided-session opening to the root owner.
#[component]
pub(super) fn MysteryCard(
    mystery: Mystery,
    mystery_set: MysterySet,
    decade: Decade,
    fruit_label: &'static str,
    pray_label: &'static str,
    open_guided: Callback<(MysterySet, Decade)>,
) -> impl IntoView {
    view! {
        <article class="mystery-card">
            <div class="mystery-art" aria-hidden="true">{mystery.icon}</div>
            <h4>{mystery.title}</h4>
            <p>{mystery.meditation}</p>
            <p class="mystery-fruit"><strong>{fruit_label}</strong><span>{mystery.fruit}</span></p>
            <AppButton
                class="mystery-pray-button"
                aria_label=format!("{pray_label}: {}", mystery.title)
                on_click=move |_| {
                    open_guided.run((mystery_set, decade));
                }
            >
                {pray_label}
            </AppButton>
        </article>
    }
}
