# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Life Journal DX is a Dioxus-based web application built with Rust. It's a cross-platform app that supports web, desktop, and mobile targets. The app uses Dioxus 0.7 with integrated routing and Tailwind CSS for styling.

## Development Commands

### Running the Development Server
```bash
dx serve
```

For specific platforms:
```bash
dx serve --platform desktop
dx serve --platform web
```

### Building
```bash
cargo build
```

For release builds:
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Tailwind CSS

Dioxus 0.7+ includes automatic Tailwind support. The `tailwind.css` file in the project root is automatically processed during `dx serve`.

If manual Tailwind customization is needed:
```bash
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
```

## Code Architecture

### Entry Point & Application Structure

The app uses a component-based architecture with Dioxus:

- **Main Entry** (`src/main.rs`): Launches the Dioxus app with the `App` component, which sets up meta tags, stylesheets, and the router.

### Routing System

The routing is defined in `src/routers.rs`:

- Uses a layout wrapper pattern with `TabBarWrapper` for routes that need the tab bar
- **Routes with tab bar**: Home (`/`) and Profile (`/profile/`)
- **Routes without tab bar**: Task (`/task/`)
- The `TabBarWrapper` component in `src/components/tab_bar/component.rs` renders the `Outlet` for child routes and the persistent bottom tab bar

### Module Organization

```
src/
├── main.rs           # App entry point and root component
├── routers.rs        # Route definitions and layout hierarchy
├── pages/            # Page components (Home, Profile, Task)
│   └── mod.rs        # Re-exports all page components
└── components/       # Reusable components
    ├── tab_bar/      # Bottom navigation tab bar
    │   ├── mod.rs
    │   └── component.rs  # TabBar and TabBarWrapper components
    └── tabs/         # Custom tabs component wrapping dioxus-primitives
        ├── mod.rs
        └── component.rs  # Tabs, TabList, TabTrigger, TabContent
```

### Component Patterns

1. **Layout Wrapper Pattern**: The `TabBarWrapper` component demonstrates the layout wrapper pattern used in Dioxus routing. It renders child routes via `Outlet::<Route>` and adds persistent UI (the tab bar).

2. **Component Wrapping**: The `tabs` module wraps `dioxus-primitives::tabs` with custom styling and variant support. This pattern allows for consistent styling while leveraging primitive components.

3. **Styling Approach**: Components use Tailwind CSS classes. The tab bar includes `safe-bottom` class for iOS safe area handling.

### Dependencies

- **dioxus** (0.7.0): Core framework with router feature
- **dioxus-primitives**: Component library (from DioxusLabs/components GitHub)
- **reqwest**: HTTP client with JSON support
- **serde**: Serialization/deserialization

### Feature Flags

The project uses feature flags for platform targeting:
- `web` (default): Web platform
- `desktop`: Desktop platform
- `mobile`: Mobile platform

## Asset Management

Assets are located in the `assets/` directory and referenced using the `asset!()` macro:

```rust
document::Stylesheet { href: asset!("/assets/main.css") }
```

Current assets:
- `main.css`: Custom styles
- `tailwind.css`: Generated Tailwind output
- `dx-components-theme.css`: Dioxus components theme
- `favicon.ico`, `header.svg`: Static assets

## Key Conventions

1. **Navigation**: Use `Link { to: Route::RouteName{} }` for internal navigation
2. **Component Definition**: Use `#[component]` macro and return `Element`
3. **Styling**: Tailwind classes are preferred; component-specific CSS goes in local style files
4. **Route Layouts**: Use `#[layout(ComponentName)]` and `#[end_layout]` attributes in route definitions
