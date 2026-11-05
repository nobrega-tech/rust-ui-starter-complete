// Theme values are primarily defined as CSS custom properties in apps/app/tailwind.css.
// Keep semantic tokens here when Rust-side theme configuration becomes necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}
