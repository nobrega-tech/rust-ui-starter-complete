# Rust UI Starter — Complete

A complete-but-lightweight Dioxus starter with:

- Dioxus 0.7
- Tailwind CSS 4
- Cargo Workspace
- Shared Web/Desktop app entry point
- Sidebar navigation and example pages
- Light/dark theme toggle
- Small reusable design system
- Domain/application core crate
- Optional Axum API scaffold (excluded from workspace by default)

## Requirements

- Rust stable via rustup
- Dioxus CLI (`dx`)
- Windows Desktop: WebView2 Runtime
- Web target: `wasm32-unknown-unknown`

## Install

```powershell
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --locked
```

## Run

Web:

```powershell
dx serve --platform web
```

Desktop:

```powershell
dx serve --platform desktop
```

## Validate

```powershell
cargo check --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets
```

## Tailwind CSS 4

`apps/app/tailwind.css` is the input stylesheet. Dioxus CLI integrates the Tailwind watcher during `dx serve` when it detects the stylesheet. Generated CSS is written under `apps/app/assets/`.

If your installed CLI does not detect the stylesheet automatically, check the current Dioxus Tailwind setup documentation for the version you installed.

## Rust/UI

Rust/UI is an independent, experimental, shadcn-inspired component registry. It is not a required runtime dependency and is not an official Dioxus package. Its copy-source workflow is intentional: copy a component into `crates/design-system/src/components/`, review its imports/dependencies, and adapt it to this project's tokens and conventions.

Catalog: https://rust-ui.com/

## Optional Axum backend

The API crate is scaffolded under `crates/api` but excluded from the default workspace so UI-only development does not compile server dependencies.

To activate it:

1. Add `"crates/api"` to the root workspace `members`.
2. Remove `exclude = ["crates/api"]`.
3. Run `cargo check -p starter-api`.
4. Run `cargo run -p starter-api`.

The API is intentionally not connected to the frontend. Add an API client at the application boundary when a real backend contract exists.

## Architecture rules

- `apps/app`: composition root, app shell, navigation and pages.
- `crates/design-system`: reusable visual components and theme tokens.
- `crates/core`: domain models and application-level rules; no Dioxus dependency.
- `crates/api`: optional server application; do not place UI code here.

Keep business rules out of page components. Keep platform-specific code at the app boundary.
