use dioxus::prelude::*;
use starter_design_system::{Button, IconPlaceholder};
use crate::{navigation::{NavItem, Sidebar}, pages::{DashboardPage, ProjectsPage, SettingsPage, ComponentsPage}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Dashboard,
    Projects,
    Components,
    Settings,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Overview",
            Self::Projects => "Projects",
            Self::Components => "Components",
            Self::Settings => "Settings",
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut active_page = use_signal(|| Page::Dashboard);
    let mut dark_mode = use_signal(|| false);

    let theme_class = if dark_mode() { "dark" } else { "" };

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "{theme_class} min-h-screen select-none bg-background text-foreground",
            div {
                class: "flex min-h-screen",
                Sidebar {
                    active: active_page(),
                    on_navigate: move |page| active_page.set(page),
                }
                main {
                    class: "min-w-0 flex-1",
                    header {
                        class: "sticky top-0 z-10 flex h-16 items-center justify-between border-b border-border bg-surface/90 px-6 backdrop-blur",
                        div {
                            p { class: "text-xs text-muted", "Workspace / Starter" }
                            h1 { class: "text-sm font-semibold", "{active_page().title()}" }
                        }
                        div {
                            class: "flex items-center gap-2",
                            Button {
                                variant: "secondary",
                                onclick: move |_| dark_mode.toggle(),
                                if dark_mode() { "Light theme" } else { "Dark theme" }
                            }
                        }
                    }
                    div {
                        class: "mx-auto w-full max-w-6xl p-6 lg:p-8",
                        match active_page() {
                            Page::Dashboard => rsx! { DashboardPage {} },
                            Page::Projects => rsx! { ProjectsPage {} },
                            Page::Components => rsx! { ComponentsPage {} },
                            Page::Settings => rsx! { SettingsPage { dark_mode: dark_mode() } },
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn nav_items() -> [NavItem; 4] {
    [
        NavItem { page: Page::Dashboard, label: "Overview", icon: "◫" },
        NavItem { page: Page::Projects, label: "Projects", icon: "▦" },
        NavItem { page: Page::Components, label: "Components", icon: "◇" },
        NavItem { page: Page::Settings, label: "Settings", icon: "⚙" },
    ]
}
