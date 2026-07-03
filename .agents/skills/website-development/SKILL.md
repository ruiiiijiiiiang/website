---
name: website-development
description: Guide for developing, styling, and building the Dioxus 0.7 full-stack Rust website. Use when modifying client views, backend rendering, blog processing, styling with Pico CSS, or packaging the app.
---

# Website Development Skill

This skill assists with developing and maintaining the full-stack Dioxus 0.7 Rust website.

## Project Architecture

- **Client Router**: Uses the Dioxus client-side router (`Route` enum in `src/main.rs`) wrapped in a `#[layout(Navbar)]` component.
- **Views**:
  - `src/views/` containing UI components.
  - `src/views/projects.rs` holds static project metadata.
- **Blog Engine**:
  - Markdown posts live under `blog/*.md` (YAML frontmatter dates are in `MM/DD/YYYY` format).
  - Server functions in `src/backend.rs` parse YAML frontmatter via `gray_matter` and render HTML via `comrak` with custom `syntect` syntax highlighting.
- **Styling**:
  - Driven by **Pico CSS v2** (loaded via CDN).
  - Custom overrides and stylesheets live in `assets/main.css` and `assets/theme_toggle.css`.
  - **Do NOT use TailwindCSS** unless explicitly requested.
- **Theme Toggle**:
  - Target-gated to WASM only (`#[cfg(target_arch = "wasm32")]`) using `web-sys` to toggle the `data-theme` attribute on the `<html>` tag.

## Common Development Commands

| Task | Command |
|---|---|
| Run Web Dev Server | `dx serve` |
| Run Fullstack Dev Server | `dx serve --fullstack` |
| Compile Sitemap Utility | `cargo build --release --bin sitemap` |
| Production Bundle (SSG) | `dx bundle --release --web --ssg` |
| Run Nix Sandbox Build | `nix build` |
| Build Docker Container | `docker build .` |

## Guardrails & Best Practices

1. **WASM Constraints**: Ensure any web-specific code in components (such as direct DOM interaction via `web-sys`) is gated with `#[cfg(target_arch = "wasm32")]`.
2. **Styling Rules**: Adhere to Pico CSS classes for layout and semantic markup. Customize via `assets/main.css` instead of embedding ad-hoc inline styles.
3. **No External Linters**: Rely solely on standard cargo tools (`cargo fmt` and `cargo clippy`). No extra linters or formatter configurations exist in the workspace.
