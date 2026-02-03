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
cd packages/mobile && dx serve
```

Build for release:
```bash
cd packages/web && dx build --release
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
- Routes are defined using the `#[derive(Routable)]` macro with `#[route("/path")]` attributes
- Layouts use `#[layout(Component)]` attribute and `Outlet::<Route> {}` for child rendering
- Assets are referenced using `asset!("/assets/filename")` macro

## Dioxus 0.7 Specifics

This project uses Dioxus 0.7 which has significant API changes from earlier versions:
- No `cx`, `Scope`, or `use_state` - use `use_signal` for local state instead
- Components are functions annotated with `#[component]` that return `Element`
- Use `use_signal(|| initial_value)` for state, `.read()` for reference, `.write()` for mutation
- Props must be owned values (`String` not `&str`), implement `PartialEq` and `Clone`
- Use `ReadOnlySignal<T>` wrapper for reactive props
- RSX uses `rsx! {}` macro with element syntax like `div { class: "foo", "content" }`

See `AGENTS.md` for complete Dioxus 0.7 API reference including signals, context, async, routing, and fullstack features.
