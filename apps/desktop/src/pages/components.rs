use dioxus::prelude::*;
use starter_design_system::{Badge, Button, Card, CardHeader, CardTitle, Separator};

#[component]
pub fn ComponentsPage() -> Element {
    let mut clicks = use_signal(|| 0);

    rsx! {
        div { class: "space-y-6",
            div {
                h2 { class: "text-2xl font-semibold", "Components" }
                p { class: "mt-1 text-sm text-muted", "A small preview of the local design system." }
            }
            Card {
                CardHeader {
                    CardTitle { "Buttons" }
                    p { class: "text-sm text-muted", "Variants and interaction example." }
                }
                div { class: "mt-4 flex flex-wrap items-center gap-2",
                    Button { onclick: move |_| clicks += 1, "Primary" }
                    Button { variant: "secondary", onclick: move |_| clicks += 1, "Secondary" }
                    Button { variant: "ghost", onclick: move |_| clicks += 1, "Ghost" }
                }
                p { class: "mt-3 text-xs text-muted", "Clicked {clicks} times" }
                Separator {}
                div { class: "mt-4 flex flex-wrap gap-2",
                    Badge { tone: "success", "Success" }
                    Badge { tone: "warning", "Warning" }
                    Badge { tone: "danger", "Danger" }
                    Badge { tone: "info", "Info" }
                    Badge { tone: "neutral", "Neutral" }
                }
            }
        }
    }
}
