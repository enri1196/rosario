use leptos::{
    ev::{KeyboardEvent, MouseEvent},
    prelude::*,
};

/// Visual treatments supported by the shared application button control.
#[derive(Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    IconPrimary,
    IconSecondary,
    IconAccent,
    Close,
    Tab,
}

impl ButtonVariant {
    const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "app-button--primary",
            Self::IconPrimary => "app-button--icon-primary",
            Self::IconSecondary => "app-button--icon-secondary",
            Self::IconAccent => "app-button--icon-accent",
            Self::Close => "app-button--close",
            Self::Tab => "app-button--tab",
        }
    }
}

/// Renders an application button with the shared interaction and focus style.
///
/// Use an icon variant only with an accessible `aria_label`; layout-specific
/// classes may be supplied without replacing the shared button treatment.
#[component]
pub fn AppButton(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: String,
    #[prop(into)] aria_label: TextProp,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] role: Option<&'static str>,
    #[prop(optional)] aria_controls: Option<&'static str>,
    #[prop(optional)] aria_pressed: Option<Signal<bool>>,
    #[prop(optional)] aria_selected: Option<Signal<bool>>,
    #[prop(optional)] title: Option<TextProp>,
    #[prop(optional)] disabled: Option<Signal<bool>>,
    #[prop(optional)] tab_index: Option<Signal<i32>>,
    #[prop(optional)] on_keydown: Option<Callback<KeyboardEvent>>,
    on_click: impl Fn(MouseEvent) + 'static,
    children: Children,
) -> impl IntoView {
    let class = format!("app-button {} {class}", variant.class_name());

    view! {
        <button
            type="button"
            class=class
            id=id
            role=role
            aria-label=aria_label
            aria-controls=aria_controls
            aria-pressed=move || aria_pressed.as_ref().map(|pressed| pressed.get().to_string())
            aria-selected=move || aria_selected.as_ref().map(|selected| selected.get().to_string())
            title=move || title.as_ref().map(TextProp::get)
            disabled=move || disabled.as_ref().is_some_and(|disabled| disabled.get())
            tabindex=move || tab_index.as_ref().map(Signal::get)
            on:click=on_click
            on:keydown=move |event| {
                if let Some(on_keydown) = on_keydown {
                    on_keydown.run(event);
                }
            }
        >
            {children()}
        </button>
    }
}
