use dioxus::prelude::*;

#[component]
pub fn Badge(
    children: Element,
    #[props(default = "neutral".to_string())] tone: String,
) -> Element {
    let tone_class = match tone.as_str() {
        "success" => "bg-success/10 text-success",
        "warning" => "bg-warning/10 text-warning",
        "danger" => "bg-danger/10 text-danger",
        "info" => "bg-primary/10 text-primary",
        _ => "bg-surface-muted text-muted",
    };

    rsx! {
        span {
            class: "inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium {tone_class}",
            {children}
        }
    }
}
