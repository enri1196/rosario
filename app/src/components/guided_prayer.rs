use super::{AppButton, ButtonVariant, GuideBox};
use crate::i18n::{Language, Mystery, MysterySet, Translation};
use crate::rosary_session::{
    Decade, DecadePrayer, GuidedStep, OpeningStep, RosarySession, DECADE_PRAYER_COUNT,
    GUIDED_STEP_COUNT,
};
use leptos::{html, prelude::*};
use std::cell::Cell;

/// Renders the shared guided Rosary session with responsive, accessible navigation.
#[component]
pub fn GuidedPrayer(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    session: RwSignal<Option<RosarySession>>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    let step_heading = NodeRef::<html::H4>::new();
    let step_panel = NodeRef::<html::Div>::new();
    let focused_step = Cell::new(None);

    Effect::new(move |_| {
        let current_step = session
            .get()
            .map(|current| (current.step_number(), current.is_complete()));
        if current_step.is_some() && focused_step.replace(current_step) != current_step {
            reset_step_panel_scroll(step_panel);
            focus_step_heading(step_heading);
        }
    });

    view! {
        <Show when=move || session.get().is_some()>
            <GuideBox>
                <section class="guided-prayer" aria-labelledby="guided-prayer-title">
                    <header class="guided-prayer-header">
                        <h3 id="guided-prayer-title">{move || copy.get().guided_title}</h3>
                        <div class="guided-prayer-actions">
                            <AppButton
                                variant=ButtonVariant::IconAccent
                                class="guided-restart-button"
                                aria_label=move || copy.get().guided_reset_label
                                title=TextProp::from(move || copy.get().guided_reset_label)
                                on_click=move |_| {
                                    update_session(session, RosarySession::reset);
                                }
                            >
                                <svg class="guided-control-icon" viewBox="0 0 24 24" aria-hidden="true">
                                    <path d="M4 7v5h5" />
                                    <path d="M5.5 16a8 8 0 1 0-.8-7.9L4 12" />
                                </svg>
                            </AppButton>
                            <AppButton
                                variant=ButtonVariant::Close
                                aria_label=move || copy.get().guided_close_label
                                title=TextProp::from(move || copy.get().guided_close_label)
                                on_click=move |_| session.set(None)
                            >
                                <svg class="guided-control-icon" viewBox="0 0 24 24" aria-hidden="true">
                                    <path d="m6 6 12 12M18 6 6 18" />
                                </svg>
                            </AppButton>
                        </div>
                    </header>

                    <div class="guided-session-meta">
                        <p>
                            <span>{move || copy.get().guided_mystery_set_label}</span>
                            {" "}
                            <strong>{move || {
                                session.get().map(|current| {
                                    current.selected_mystery_set().label(language.get())
                                }).unwrap_or("")
                            }}</strong>
                        </p>
                        <Show when=move || {
                            session.get().and_then(|current| current.focused_mystery()).is_some()
                        }>
                            <p class="guided-selected-mystery">
                                <span>{move || copy.get().guided_selected_mystery_label}</span>
                                {" "}
                                <strong>{move || focused_mystery(copy.get(), session.get()).map(|mystery| mystery.title).unwrap_or("")}</strong>
                            </p>
                        </Show>
                    </div>

                    <Show when=move || {
                        !intentions.get().is_empty()
                            && session.get().is_some_and(|current| {
                                current.is_first_step() || current.is_complete()
                            })
                    }>
                        <div class="guided-intentions">
                            <p class="guided-intention-label">
                                {move || copy.get().guided_intention_label}
                            </p>
                            <ul class="guided-intention-list">
                                <For
                                    each=move || intentions.get()
                                    key=|intention| intention.clone()
                                    children=move |intention| view! {
                                        <li class="guided-intention-tag">{intention}</li>
                                    }
                                />
                            </ul>
                        </div>
                    </Show>

                    <Show
                        when=move || session.get().is_some_and(|current| current.is_complete())
                        fallback=move || view! {
                            <div class="guided-step-layout">
                                <div class="guided-step-control guided-step-control--previous">
                                    <AppButton
                                        variant=ButtonVariant::IconSecondary
                                        class="guided-side-button guided-previous-button"
                                        aria_label=move || copy.get().guided_previous_label
                                        title=TextProp::from(move || copy.get().guided_previous_label)
                                        disabled=Signal::derive(move || session.get().is_none_or(|current| current.is_first_step()))
                                        on_click=move |_| {
                                            update_session(session, RosarySession::previous);
                                        }
                                    >
                                        <svg class="guided-control-icon" viewBox="0 0 24 24" aria-hidden="true">
                                            <path d="m15 18-6-6 6-6" />
                                        </svg>
                                        <span class="guided-control-label">
                                            {move || copy.get().guided_previous_label}
                                        </span>
                                    </AppButton>
                                </div>
                                <div class="guided-step-panel" node_ref=step_panel tabindex="0">
                                    <div class=move || step_transition_class(session.get())>
                                        <p class="guided-progress" aria-live="polite" aria-atomic="true">
                                            {move || progress_text(copy.get(), session.get())}
                                        </p>
                                        <h4
                                            id="guided-step-heading"
                                            node_ref=step_heading
                                            tabindex="-1"
                                            aria-current=move || session.get()
                                                .filter(|current| !current.is_complete())
                                                .map(|_| "step")
                                        >
                                            {move || session.get().map(|current| step_title(copy.get(), current)).unwrap_or("")}
                                        </h4>
                                        <div class="guided-step-content">
                                            <Show when=move || !step_body(copy.get(), session.get()).is_empty()>
                                                <p class="guided-prayer-text">
                                                    {move || step_body(copy.get(), session.get())}
                                                </p>
                                            </Show>
                                            <Show when=move || session.get().is_some_and(|current| {
                                                matches!(current.active_step(), GuidedStep::Closing)
                                            })>
                                                <p class="guided-closing-note">{move || copy.get().ending}</p>
                                            </Show>
                                        </div>
                                    </div>
                                </div>
                                <div class="guided-step-control guided-step-control--next">
                                    <AppButton
                                        variant=ButtonVariant::IconPrimary
                                        class="guided-side-button guided-next-button"
                                        aria_label=move || next_control_label(copy.get(), session.get())
                                        title=TextProp::from(move || next_control_label(copy.get(), session.get()))
                                        on_click=move |_| {
                                            update_session(session, RosarySession::next);
                                        }
                                    >
                                        <Show
                                            when=move || session.get().is_some_and(RosarySession::is_last_step)
                                            fallback=|| view! {
                                                <svg class="guided-control-icon" viewBox="0 0 24 24" aria-hidden="true">
                                                    <path d="m9 18 6-6-6-6" />
                                                </svg>
                                            }
                                        >
                                            <svg class="guided-control-icon" viewBox="0 0 24 24" aria-hidden="true">
                                                <path d="m5 12 4 4L19 6" />
                                            </svg>
                                        </Show>
                                        <span class="guided-control-label">
                                            {move || next_control_label(copy.get(), session.get())}
                                        </span>
                                    </AppButton>
                                </div>
                            </div>
                        }
                    >
                        <div class="guided-completion">
                            <h4
                                id="guided-step-heading"
                                node_ref=step_heading
                                tabindex="-1"
                            >
                                {move || copy.get().guided_completion_title}
                            </h4>
                            <p>{move || copy.get().guided_completion_message}</p>
                            <AppButton
                                variant=ButtonVariant::Primary
                                aria_label=move || copy.get().guided_restart_label
                                on_click=move |_| {
                                    update_session(session, RosarySession::reset);
                                }
                            >
                                {move || copy.get().guided_restart_label}
                            </AppButton>
                            <section
                                class="post-rosary-prayers"
                                aria-labelledby="post-rosary-prayers-title"
                            >
                                <h5 id="post-rosary-prayers-title">
                                    {move || copy.get().post_rosary_prayers_title}
                                </h5>
                                <For
                                    each=move || copy.get().post_rosary_prayers.iter().copied()
                                    key=|prayer| prayer.title
                                    children=move |prayer| view! {
                                        <article class="post-rosary-prayer">
                                            <h6>{prayer.title}</h6>
                                            <p>{prayer.text}</p>
                                        </article>
                                    }
                                />
                            </section>
                        </div>
                    </Show>
                </section>
            </GuideBox>
        </Show>
    }
}

/// Applies a mutation to the active session when one is open.
fn update_session(
    session: RwSignal<Option<RosarySession>>,
    update: impl FnOnce(&mut RosarySession),
) {
    session.update(|current| {
        if let Some(current) = current {
            update(current);
        }
    });
}

/// Focuses the current step heading without moving the surrounding page.
fn focus_step_heading(step_heading: NodeRef<html::H4>) {
    if let Some(heading) = step_heading.get() {
        let options = web_sys::FocusOptions::new();
        options.set_prevent_scroll(true);
        let _ = heading.focus_with_options(&options);
    }
}

/// Restores the internally scrollable prayer panel to its origin.
fn reset_step_panel_scroll(step_panel: NodeRef<html::Div>) {
    if let Some(panel) = step_panel.get() {
        panel.set_scroll_top(0);
    }
}

/// Alternates equivalent animation names so each newly active step transitions once.
fn step_transition_class(session: Option<RosarySession>) -> String {
    let transition = session
        .map(|current| current.step_number() % 2)
        .unwrap_or_default();
    format!("guided-step-panel-content guided-step-transition--{transition}")
}

/// Returns the translated label for advancing or completing the active session.
fn next_control_label(copy: Translation, session: Option<RosarySession>) -> &'static str {
    session
        .map(|current| {
            if current.is_last_step() {
                copy.guided_finish_label
            } else {
                copy.guided_next_label
            }
        })
        .unwrap_or("")
}

/// Formats localized full-session and decade-level progress for the polite live region.
fn progress_text(copy: Translation, session: Option<RosarySession>) -> String {
    session
        .map(|current| {
            let step_progress = format!(
                "{} {} {} {}",
                copy.guided_step_label,
                current.step_number(),
                copy.guided_of_label,
                GUIDED_STEP_COUNT
            );

            current
                .active_decade_progress()
                .map(|progress| {
                    format!(
                        "{step_progress} · {} {} {} 5 · {} {} {} {}",
                        copy.guided_decade_label,
                        progress.decade_number(),
                        copy.guided_of_label,
                        copy.guided_prayer_label,
                        progress.prayer_number(),
                        copy.guided_of_label,
                        DECADE_PRAYER_COUNT,
                    )
                })
                .unwrap_or(step_progress)
        })
        .unwrap_or_default()
}

fn focused_mystery(copy: Translation, session: Option<RosarySession>) -> Option<Mystery> {
    let current = session?;
    mystery_for(
        copy,
        current.selected_mystery_set(),
        current.focused_mystery()?,
    )
}

fn mystery_for(copy: Translation, set: MysterySet, decade: Decade) -> Option<Mystery> {
    copy.groups
        .iter()
        .find(|group| group.set == set)
        .and_then(|group| group.mysteries.get(decade.index()))
        .copied()
}

fn step_title(copy: Translation, session: RosarySession) -> &'static str {
    match session.active_step() {
        GuidedStep::Opening(OpeningStep::SignOfCross) => copy.steps[0],
        GuidedStep::Opening(OpeningStep::Creed) => copy.creed_title,
        GuidedStep::Opening(OpeningStep::OurFather) => copy.prayers[0].title,
        GuidedStep::Opening(OpeningStep::ThreeHailMarys) => copy.steps[3],
        GuidedStep::Opening(OpeningStep::GloryBe) => copy.prayers[2].title,
        GuidedStep::Decade {
            decade,
            prayer: DecadePrayer::Mystery,
        } => mystery_for(copy, session.selected_mystery_set(), decade)
            .map(|mystery| mystery.title)
            .unwrap_or(copy.steps[7]),
        GuidedStep::Decade {
            prayer: DecadePrayer::OurFather,
            ..
        } => copy.prayers[0].title,
        GuidedStep::Decade {
            prayer: DecadePrayer::TenHailMarys,
            ..
        } => copy.guided_ten_hail_marys_label,
        GuidedStep::Decade {
            prayer: DecadePrayer::GloryBe,
            ..
        } => copy.prayers[2].title,
        GuidedStep::Decade {
            prayer: DecadePrayer::FatimaPrayer,
            ..
        } => copy.prayers[3].title,
        GuidedStep::Closing => copy.ending_title,
    }
}

fn step_body(copy: Translation, session: Option<RosarySession>) -> &'static str {
    let Some(current) = session else {
        return "";
    };

    match current.active_step() {
        GuidedStep::Opening(OpeningStep::SignOfCross) => "",
        GuidedStep::Opening(OpeningStep::Creed) => copy.creed,
        GuidedStep::Opening(OpeningStep::OurFather) => copy.prayers[0].text,
        GuidedStep::Opening(OpeningStep::ThreeHailMarys) => copy.prayers[1].text,
        GuidedStep::Opening(OpeningStep::GloryBe) => copy.prayers[2].text,
        GuidedStep::Decade {
            decade,
            prayer: DecadePrayer::Mystery,
        } => mystery_for(copy, current.selected_mystery_set(), decade)
            .map(|mystery| mystery.meditation)
            .unwrap_or(""),
        GuidedStep::Decade {
            prayer: DecadePrayer::OurFather,
            ..
        } => copy.prayers[0].text,
        GuidedStep::Decade {
            prayer: DecadePrayer::TenHailMarys,
            ..
        } => copy.prayers[1].text,
        GuidedStep::Decade {
            prayer: DecadePrayer::GloryBe,
            ..
        } => copy.prayers[2].text,
        GuidedStep::Decade {
            prayer: DecadePrayer::FatimaPrayer,
            ..
        } => copy.prayers[3].text,
        GuidedStep::Closing => copy.prayers[4].text,
    }
}
