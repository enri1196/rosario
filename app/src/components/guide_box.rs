use leptos::prelude::*;

#[component]
pub fn GuideBox(children: Children) -> impl IntoView {
    view! {
        <div class="guide-box">{children()}</div>
    }
}
