use super::AppButton;
use crate::i18n::{Mystery, MysterySet};
use crate::rosary_session::{Decade, RosarySession};
use leptos::prelude::*;

#[component]
pub(super) fn MysteryCard(
    mystery: Mystery,
    mystery_set: MysterySet,
    decade: Decade,
    fruit_label: &'static str,
    pray_label: &'static str,
    guided_session: RwSignal<Option<RosarySession>>,
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
                    guided_session.set(Some(RosarySession::start_for_mystery(
                        mystery_set,
                        decade,
                    )));
                }
            >
                {pray_label}
            </AppButton>
        </article>
    }
}
