use dioxus::prelude::*;
use starter_design_system::{Card, CardHeader, CardTitle, Badge};

#[component]
pub fn SettingsPage(dark_mode: bool) -> Element {
    rsx! {
        div { class: "space-y-6",
            div {
                h2 { class: "text-2xl font-semibold", "Settings" }
                p { class: "mt-1 text-sm text-muted", "Example application preferences." }
            }
            Card {
                CardHeader {
                    CardTitle { "Appearance" }
                    p { class: "text-sm text-muted", "Theme state controlled by the app shell." }
                }
                div { class: "mt-4 flex items-center justify-between",
                    div {
                        p { class: "text-sm font-medium", "Current theme" }
                        p { class: "mt-1 text-xs text-muted", "Use the theme toggle in the top bar." }
                    }
                    Badge { tone: "info", if dark_mode { "Dark" } else { "Light" } }
                }
            }
        }
    }
}
