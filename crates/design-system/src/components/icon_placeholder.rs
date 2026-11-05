use dioxus::prelude::*;

// Temporary icon slot. Replace with your chosen Lucide integration or local SVG assets.
#[component]
pub fn IconPlaceholder(label: String) -> Element {
    rsx! {
        span {
            class: "inline-flex h-5 w-5 items-center justify-center text-xs text-muted",
            aria_label: "{label}",
            "◇"
        }
    }
}
