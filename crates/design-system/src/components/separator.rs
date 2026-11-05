use dioxus::prelude::*;

#[component]
pub fn Separator() -> Element {
    rsx! { div { class: "my-4 h-px w-full bg-border" } }
}
