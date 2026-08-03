use leptos::{ev::MouseEvent, prelude::*};

/// Visual treatments supported by the shared application button control.
#[derive(Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    IconPrimary,
    IconSecondary,
    IconAccent,
    Close,
}

impl ButtonVariant {
    const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "app-button--primary",
            Self::Secondary => "app-button--secondary",
            Self::IconPrimary => "app-button--icon-primary",
            Self::IconSecondary => "app-button--icon-secondary",
            Self::IconAccent => "app-button--icon-accent",
            Self::Close => "app-button--close",
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
    #[prop(optional)] aria_pressed: Option<Signal<bool>>,
    #[prop(optional)] title: Option<TextProp>,
    #[prop(optional)] disabled: Option<Signal<bool>>,
    on_click: impl Fn(MouseEvent) + 'static,
    children: Children,
) -> impl IntoView {
    let class = format!("app-button {} {class}", variant.class_name());

    view! {
        <button
            type="button"
            class=class
            aria-label=aria_label
            aria-pressed=move || aria_pressed.as_ref().map(|pressed| pressed.get().to_string())
            title=move || title.as_ref().map(TextProp::get)
            disabled=move || disabled.as_ref().is_some_and(|disabled| disabled.get())
            on:click=on_click
        >
            {children()}
        </button>
    }
}
