mod components;
mod i18n;

use components::{Footer, Header, MysteriesSection, PrayerSidebar, RosaryGuide};
use i18n::{Language, Translation};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let language = RwSignal::new(Language::Italian);
    let copy = Memo::new(move |_| Translation::for_language(language.get()));

    view! {
        <Title text=move || copy.get().page_title />
        <div class="app-shell">
            <Header language copy />
            <main id="main-content">
                <div class="main-grid">
                    <PrayerSidebar copy />
                    <RosaryGuide copy />
                </div>
                <MysteriesSection copy />
            </main>
            <Footer copy />
        </div>
    }
}
