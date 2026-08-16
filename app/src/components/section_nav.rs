use super::{AppButton, ButtonVariant};
use crate::i18n::Translation;
use crate::navigation::AppSection;
use leptos::{ev::KeyboardEvent, prelude::*};
use wasm_bindgen::JsCast;

/// Renders the translated, manually activated tab list for workspace sections.
///
/// Arrow keys and Home/End move focus without changing the active panel;
/// Enter or Space activates the focused tab. Pointer activation selects the
/// clicked tab directly.
#[component]
pub fn SectionNav(active_section: RwSignal<AppSection>, copy: Memo<Translation>) -> impl IntoView {
    let focused_section = RwSignal::new(active_section.get_untracked());

    Effect::new(move |_| {
        focused_section.set(active_section.get());
    });

    view! {
        <nav class="section-nav" aria-label=move || copy.get().workspace_navigation_label>
            <div class="section-tablist" role="tablist" aria-orientation="horizontal">
                {AppSection::ALL.into_iter().map(|section| view! {
                    <SectionTab
                        section
                        active_section
                        focused_section
                        copy
                    />
                }).collect_view()}
            </div>
        </nav>
    }
}

/// Renders one roving-tabindex control in the workspace tab list.
#[component]
fn SectionTab(
    section: AppSection,
    active_section: RwSignal<AppSection>,
    focused_section: RwSignal<AppSection>,
    copy: Memo<Translation>,
) -> impl IntoView {
    let on_keydown = Callback::new(move |event: KeyboardEvent| {
        handle_tab_keydown(event, section, active_section, focused_section);
    });

    view! {
        <AppButton
            variant=ButtonVariant::Tab
            class="section-tab"
            id=section.control_id()
            role="tab"
            aria_controls=section.panel_id()
            aria_label=move || section_label(copy.get(), section)
            aria_selected=Signal::derive(move || active_section.get() == section)
            tab_index=Signal::derive(move || {
                if focused_section.get() == section { 0 } else { -1 }
            })
            on_keydown
            on_click=move |_| {
                focused_section.set(section);
                active_section.set(section);
            }
        >
            {move || section_label(copy.get(), section)}
        </AppButton>
    }
}

/// Applies the manual-activation keyboard contract for the section tab list.
fn handle_tab_keydown(
    event: KeyboardEvent,
    section: AppSection,
    active_section: RwSignal<AppSection>,
    focused_section: RwSignal<AppSection>,
) {
    let destination = match event.key().as_str() {
        "ArrowLeft" | "ArrowUp" => Some(section.previous()),
        "ArrowRight" | "ArrowDown" => Some(section.next()),
        "Home" => Some(AppSection::Guide),
        "End" => Some(AppSection::Prayers),
        _ => None,
    };

    if let Some(destination) = destination {
        event.prevent_default();
        focused_section.set(destination);
        focus_control(destination);
        return;
    }

    if matches!(event.key().as_str(), "Enter" | " " | "Spacebar") {
        event.prevent_default();
        active_section.set(focused_section.get_untracked());
    }
}

/// Focuses a section tab without scrolling the surrounding page.
fn focus_control(section: AppSection) {
    let Some(control) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(section.control_id()))
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };

    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = control.focus_with_options(&options);
}

/// Returns the localized short label for a workspace section.
fn section_label(copy: Translation, section: AppSection) -> &'static str {
    match section {
        AppSection::Guide => copy.guide_section_label,
        AppSection::Mysteries => copy.mysteries_section_label,
        AppSection::Prayers => copy.prayers_section_label,
    }
}
