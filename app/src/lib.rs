mod calendar;
mod components;
mod i18n;
mod intentions;
mod navigation;
mod rosary_session;
mod theme;

use components::{Footer, Header, MysteriesSection, PrayersSection, RosaryGuide, SectionNav};
use i18n::{Language, MysterySet, Translation};
use intentions::load_intentions;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use navigation::AppSection;
use rosary_session::{Decade, RosarySession};
use std::cell::Cell;
use theme::{apply_theme, persist_theme, Theme};
use wasm_bindgen::JsCast;

/// Renders the application shell and owns all shared workspace state.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let language = RwSignal::new(Language::from_browser());
    let copy = Memo::new(move |_| Translation::for_language(language.get()));
    let theme = RwSignal::new(Theme::from_browser());
    let guided_session = RwSignal::<Option<RosarySession>>::new(None);
    let intentions = RwSignal::new(load_intentions());
    let active_section = RwSignal::new(AppSection::default());
    let is_initial_theme = Cell::new(true);
    let previous_workspace_state = Cell::new(None);

    let open_guided = Callback::new(move |(mystery_set, decade): (MysterySet, Decade)| {
        guided_session.set(Some(RosarySession::start_for_mystery(mystery_set, decade)));
        active_section.set(AppSection::Guide);
    });

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

    Effect::new(move |_| {
        let workspace_state = (active_section.get(), guided_session.get().is_some());
        let previous = previous_workspace_state.replace(Some(workspace_state));

        if let Some(previous) = previous {
            let section_changed = previous.0 != workspace_state.0;
            let session_closed =
                previous.0 == AppSection::Guide && previous.1 && !workspace_state.1;

            if section_changed || session_closed {
                focus_workspace_heading(workspace_state.0);
            }
        }
    });

    view! {
        <Title text=move || copy.get().page_title />
        <div class="app-shell">
            <Header language copy theme />
            <SectionNav active_section copy />
            <main id="main-content" class="workspace-main">
                {move || match active_section.get() {
                    AppSection::Guide => view! {
                        <RosaryGuide copy language guided_session intentions />
                    }.into_any(),
                    AppSection::Mysteries => view! {
                        <MysteriesSection copy language open_guided />
                    }.into_any(),
                    AppSection::Prayers => view! {
                        <PrayersSection copy />
                    }.into_any(),
                }}
            </main>
            <Footer copy />
        </div>
    }
}

/// Focuses the active workspace heading without moving it behind sticky navigation.
fn focus_workspace_heading(section: AppSection) {
    let Some(heading) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(section.heading_id()))
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };

    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = heading.focus_with_options(&options);
    heading.scroll_into_view();
}
