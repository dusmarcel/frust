# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

This is a Dioxus 0.7 cross-platform application. Install the Dioxus CLI first:
```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

Serve a platform (run from the platform's directory):
```bash
cd packages/web && dx serve
cd packages/desktop && dx serve
cd packages/mobile && dx serve --platform android
cd packages/mobile && dx serve --platform ios
```

Build for release:
```bash
cd packages/web && dx build --release
```

Bundle for production (from project root):
```bash
dx bundle --package web --release
```

Docker development (serves on http://localhost:8073):
```bash
docker compose -f compose.dev.yaml up --build   # dev with hot reload
docker compose up --build                        # production build with nginx
```

## Architecture

**Workspace Structure:**
- `packages/web/` - Web platform entry point and web-specific views
- `packages/desktop/` - Desktop platform entry point and desktop-specific views
- `packages/mobile/` - Mobile platform entry point and mobile-specific views
- `packages/ui/` - Shared UI components used across all platforms

**Key Patterns:**
- Each platform crate has its own `main.rs` with a `Route` enum and platform-specific router wrapper
- Shared components (`Hero`, `Navbar`) live in the `ui` crate and are imported by platform crates
- Each platform wraps shared components with platform-specific wrappers (e.g., `WebNavbar` wraps `Navbar`) to inject the platform's `Route` type
- Routes are defined using the `#[derive(Routable)]` macro with `#[route("/path")]` attributes
- Layouts use `#[layout(Component)]` attribute and `Outlet::<Route> {}` for child rendering
- Assets are referenced using `asset!("/assets/filename")` macro
- Platform-specific styles go in `packages/{platform}/assets/`, shared styles in `packages/ui/assets/styling/`
- Global CSS is injected via `document::Link { rel: "stylesheet", href: asset!(...) }` in the `App` component

**Adding a new shared component:**
1. Create the component in `packages/ui/src/` and re-export from `lib.rs`
2. Add styles in `packages/ui/assets/styling/`
3. Create platform-specific wrappers in each platform crate if the component needs the platform's `Route` type

**Adding a new route:**
1. Add a variant to the platform's `Route` enum in `main.rs` with `#[route("/path")]`
2. Create the view component in `packages/{platform}/src/views/` and re-export from `mod.rs`

## Dioxus 0.7 Specifics

This project uses Dioxus 0.7 which has significant API changes from earlier versions:
- No `cx`, `Scope`, or `use_state` - use `use_signal` for local state instead
- Components are functions annotated with `#[component]` that return `Element`
- Use `use_signal(|| initial_value)` for state, `.read()` for reference, `.write()` for mutation
- Props must be owned values (`String` not `&str`), implement `PartialEq` and `Clone`
- Use `ReadOnlySignal<T>` wrapper for reactive props
- RSX uses `rsx! {}` macro with element syntax like `div { class: "foo", "content" }`

See `AGENTS.md` for complete Dioxus 0.7 API reference including signals, context, async, routing, and fullstack features.
