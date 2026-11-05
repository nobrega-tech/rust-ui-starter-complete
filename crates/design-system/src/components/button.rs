use dioxus::prelude::*;

#[component]
pub fn Button(
    onclick: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = "primary".to_string())] variant: String,
) -> Element {
    let variant_class = match variant.as_str() {
        "secondary" => "border border-border bg-surface-muted text-foreground hover:opacity-80",
        "ghost" => "bg-transparent text-foreground hover:bg-surface-muted",
        _ => "bg-primary text-primary-foreground hover:opacity-90",
    };

    rsx! {
        button {
            r#type: "button",
            class: "inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium transition-opacity focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary {variant_class}",
            onclick: move |event| onclick.call(event),
            {children}
        }
    }
}
