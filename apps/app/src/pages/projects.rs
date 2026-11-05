use dioxus::prelude::*;
use starter_design_system::{Badge, Button, Card, CardHeader, CardTitle};

#[component]
pub fn ProjectsPage() -> Element {
    let projects = [
        ("Website redesign", "Design", "In progress"),
        ("Customer portal", "Engineering", "Planning"),
        ("Internal tools", "Operations", "Review"),
    ];

    rsx! {
        div { class: "space-y-6",
            div { class: "flex flex-wrap items-end justify-between gap-3",
                div {
                    h2 { class: "text-2xl font-semibold", "Projects" }
                    p { class: "mt-1 text-sm text-muted", "Example project list — replace with your domain data." }
                }
                Button { onclick: move |_| {}, "New project" }
            }
            Card {
                div { class: "divide-y divide-border",
                    for (name, team, status) in projects {
                        div { class: "flex flex-wrap items-center justify-between gap-3 py-4 first:pt-0 last:pb-0",
                            div {
                                p { class: "text-sm font-medium", "{name}" }
                                p { class: "mt-1 text-xs text-muted", "{team}" }
                            }
                            Badge { tone: "neutral", "{status}" }
                        }
                    }
                }
            }
        }
    }
}
