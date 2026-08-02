mod calendar;
mod components;
mod i18n;
mod intentions;
mod rosary_session;
mod theme;

use components::{Footer, Header, MysteriesSection, PrayerSidebar, RosaryGuide};
use i18n::{Language, Translation};
use intentions::load_intention;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use rosary_session::RosarySession;
use std::cell::Cell;
use theme::{apply_theme, persist_theme, Theme};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let language = RwSignal::new(Language::from_browser());
    let copy = Memo::new(move |_| Translation::for_language(language.get()));
    let theme = RwSignal::new(Theme::from_browser());
    let guided_session = RwSignal::<Option<RosarySession>>::new(None);
    let intention = RwSignal::new(load_intention());
    let is_initial_theme = Cell::new(true);

    Effect::new(move |_| {
        let theme = theme.get();
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            apply_theme(theme, &document);
        }

        // The system-derived initial mode remains transient until the user
        // explicitly changes it with the header control.
        if !is_initial_theme.replace(false) {
            persist_theme(theme);
        }
    });

    view! {
        <Title text=move || copy.get().page_title />
        <div class="app-shell">
            <Header language copy theme />
            <main id="main-content">
                <div class="main-grid">
                    <PrayerSidebar copy />
                <RosaryGuide copy language guided_session intention />
                </div>
                <MysteriesSection copy language guided_session />
            </main>
            <Footer copy />
        </div>
    }
}
