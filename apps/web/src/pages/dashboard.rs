use dioxus::prelude::*;
use starter_design_system::{Badge, Card, CardHeader, CardTitle};

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: "space-y-8",
            div {
                p { class: "text-sm text-muted", "A clean starting point for your application." }
                h2 { class: "mt-1 text-2xl font-semibold tracking-tight", "Good to see you." }
            }
            div { class: "grid gap-4 sm:grid-cols-2 xl:grid-cols-3",
                Card {
                    CardHeader {
                        CardTitle { "Application" }
                        Badge { tone: "success", "Ready to customize" }
                    }
                    p { class: "mt-3 text-sm text-muted", "Dioxus 0.7 with shared Web and Desktop UI." }
                }
                Card {
                    CardHeader {
                        CardTitle { "Design system" }
                        Badge { tone: "info", "Starter components" }
                    }
                    p { class: "mt-3 text-sm text-muted", "Reusable components live in their own crate." }
                }
                Card {
                    CardHeader {
                        CardTitle { "Backend" }
                        Badge { tone: "neutral", "Optional" }
                    }
                    p { class: "mt-3 text-sm text-muted", "Axum scaffold is available when you need it." }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Getting started" }
                    p { class: "text-sm text-muted", "A suggested order for adapting this starter." }
                }
                ol { class: "mt-4 space-y-3 text-sm",
                    li { class: "flex gap-3", span { class: "font-semibold text-primary", "01" } "Replace the example navigation with your product areas." }
                    li { class: "flex gap-3", span { class: "font-semibold text-primary", "02" } "Define your design tokens and add components." }
                    li { class: "flex gap-3", span { class: "font-semibold text-primary", "03" } "Move business rules into the core crate." }
                    li { class: "flex gap-3", span { class: "font-semibold text-primary", "04" } "Connect an API only when a backend contract is defined." }
                }
            }
        }
    }
}
