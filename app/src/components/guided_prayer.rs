use super::{AppButton, ButtonVariant, GuideBox};
use crate::i18n::{Language, Mystery, MysterySet, Translation};
use crate::rosary_session::{
    Decade, DecadePrayer, GuidedStep, OpeningStep, RosarySession, GUIDED_STEP_COUNT,
};
use leptos::{html, prelude::*};
use std::cell::Cell;

#[component]
pub fn GuidedPrayer(
    copy: Memo<Translation>,
    language: RwSignal<Language>,
    session: RwSignal<Option<RosarySession>>,
    intentions: RwSignal<Vec<String>>,
) -> impl IntoView {
    let step_heading = NodeRef::<html::H4>::new();
    let was_open = Cell::new(false);

    Effect::new(move |_| {
        let is_open = session.get().is_some();
        if is_open && !was_open.replace(is_open) {
            focus_step_heading(step_heading);
        }
    });

    view! {
        <Show when=move || session.get().is_some()>
            <GuideBox>
                <section class="guided-prayer" aria-labelledby="guided-prayer-title">
                    <header class="guided-prayer-header">
                        <h3 id="guided-prayer-title">{move || copy.get().guided_title}</h3>
                        <AppButton
                            variant=ButtonVariant::Close
                            aria_label=move || copy.get().guided_close_label
                            on_click=move |_| session.set(None)
                        >
                            <span aria-hidden="true">"×"</span>
                        </AppButton>
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

                    <Show when=move || session.get().is_some_and(|current| !current.is_complete())>
                        <p class="guided-progress" aria-live="polite" aria-atomic="true">
                            {move || progress_text(copy.get(), session.get())}
                        </p>
                    </Show>

                    <h4
                        id="guided-step-heading"
                        node_ref=step_heading
                        tabindex="-1"
                        aria-current=move || session.get()
                            .filter(|current| !current.is_complete())
                            .map(|_| "step")
                    >
                        {move || session.get().map(|current| {
                            if current.is_complete() {
                                copy.get().guided_completion_title
                            } else {
                                step_title(copy.get(), current)
                            }
                        }).unwrap_or("")}
                    </h4>

                    <Show
                        when=move || session.get().is_some_and(|current| current.is_complete())
                        fallback=move || view! {
                            <div class="guided-step-content">
                                <Show when=move || session.get().and_then(|current| current.active_decade()).is_some()>
                                    <p class="guided-active-decade">
                                        {move || active_decade_text(copy.get(), session.get())}
                                    </p>
                                </Show>
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

                            <nav class="guided-navigation" aria-label=move || copy.get().guided_title>
                                <AppButton
                                    variant=ButtonVariant::Secondary
                                    class="guided-secondary-button"
                                    aria_label=move || copy.get().guided_previous_label
                                    disabled=Signal::derive(move || session.get().is_none_or(|current| current.is_first_step()))
                                    on_click=move |_| {
                                        update_session(session, RosarySession::previous);
                                        focus_step_heading(step_heading);
                                    }
                                >
                                    {move || copy.get().guided_previous_label}
                                </AppButton>
                                <AppButton
                                    variant=ButtonVariant::Secondary
                                    class="guided-secondary-button"
                                    aria_label=move || copy.get().guided_reset_label
                                    on_click=move |_| {
                                        update_session(session, RosarySession::reset);
                                        focus_step_heading(step_heading);
                                    }
                                >
                                    {move || copy.get().guided_reset_label}
                                </AppButton>
                                <AppButton
                                    class="guided-primary-button"
                                    aria_label=move || session.get().map(|current| {
                                        if current.is_last_step() {
                                            copy.get().guided_finish_label
                                        } else {
                                            copy.get().guided_next_label
                                        }
                                    }).unwrap_or("")
                                    on_click=move |_| {
                                        update_session(session, RosarySession::next);
                                        focus_step_heading(step_heading);
                                    }
                                >
                                    {move || session.get().map(|current| {
                                        if current.is_last_step() {
                                            copy.get().guided_finish_label
                                        } else {
                                            copy.get().guided_next_label
                                        }
                                    }).unwrap_or("")}
                                </AppButton>
                            </nav>
                        }
                    >
                        <div class="guided-completion">
                            <p>{move || copy.get().guided_completion_message}</p>
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
                            <AppButton
                                class="guided-primary-button"
                                aria_label=move || copy.get().guided_restart_label
                                on_click=move |_| {
                                    update_session(session, RosarySession::reset);
                                    focus_step_heading(step_heading);
                                }
                            >
                                {move || copy.get().guided_restart_label}
                            </AppButton>
                        </div>
                    </Show>
                </section>
            </GuideBox>
        </Show>
    }
}

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

fn focus_step_heading(step_heading: NodeRef<html::H4>) {
    if let Some(heading) = step_heading.get() {
        let _ = heading.focus();
    }
}

fn progress_text(copy: Translation, session: Option<RosarySession>) -> String {
    session
        .map(|current| {
            format!(
                "{} {} {} {}",
                copy.guided_step_label,
                current.step_number(),
                copy.guided_of_label,
                GUIDED_STEP_COUNT
            )
        })
        .unwrap_or_default()
}

fn active_decade_text(copy: Translation, session: Option<RosarySession>) -> String {
    session
        .and_then(|current| {
            let decade = current.active_decade()?;
            let mystery = mystery_for(copy, current.selected_mystery_set(), decade)?;
            Some(format!(
                "{} {} {} {} · {}",
                copy.guided_decade_label,
                decade.number(),
                copy.guided_of_label,
                5,
                mystery.title
            ))
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
