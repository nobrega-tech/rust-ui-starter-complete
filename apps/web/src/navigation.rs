use dioxus::prelude::*;
use crate::app::{nav_items, Page};

#[derive(Clone, Copy)]
pub(crate) struct NavItem {
    pub page: Page,
    pub label: &'static str,
    pub icon: &'static str,
}

#[component]
pub fn Sidebar(active: Page, on_navigate: EventHandler<Page>) -> Element {
    rsx! {
        aside {
            class: "hidden w-64 shrink-0 border-r border-border bg-surface p-4 md:flex md:flex-col",
            div {
                class: "flex items-center gap-3 px-2 py-3",
                div {
                    class: "flex h-9 w-9 items-center justify-center rounded-xl bg-primary text-sm font-bold text-primary-foreground",
                    "R"
                }
                div {
                    p { class: "text-sm font-semibold", "Rust UI Starter" }
                    p { class: "text-xs text-muted", "Workspace" }
                }
            }
            div { class: "my-5 h-px bg-border" }
            p { class: "px-3 pb-2 text-[10px] font-semibold uppercase tracking-widest text-muted", "Workspace" }
            nav { class: "flex flex-col gap-1",
                for item in nav_items() {
                    button {
                        key: "{item.label}",
                        class: if active == item.page {
                            "flex items-center gap-3 rounded-lg bg-surface-muted px-3 py-2.5 text-left text-sm font-medium"
                        } else {
                            "flex items-center gap-3 rounded-lg px-3 py-2.5 text-left text-sm text-muted hover:bg-surface-muted hover:text-foreground"
                        },
                        onclick: move |_| on_navigate.call(item.page),
                        span { class: "w-5 text-center", "{item.icon}" }
                        "{item.label}"
                    }
                }
            }
            div { class: "mt-auto rounded-xl border border-border bg-surface-muted p-3",
                p { class: "text-xs font-medium", "Starter template" }
                p { class: "mt-1 text-xs leading-5 text-muted", "A small foundation for your next Rust application." }
            }
        }
    }
}
