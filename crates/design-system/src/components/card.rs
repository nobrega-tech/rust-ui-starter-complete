use dioxus::prelude::*;

#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        section {
            class: "rounded-panel border border-border bg-surface p-5 shadow-sm",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-wrap items-start justify-between gap-3", {children} }
    }
}

#[component]
pub fn CardTitle(children: Element) -> Element {
    rsx! {
        h3 { class: "text-sm font-semibold", {children} }
    }
}
