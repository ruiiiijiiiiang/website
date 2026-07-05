# AGENTS.md

## Project Structure

Personal website — full-stack Dioxus 0.7 app (Rust, edition 2024). Two feature sets:
- `web` (default) — WASM client
- `server` — SSR/SSG with Markdown blog rendering

Two binaries:
- `website` — main Dioxus app (entry: `src/main.rs`)
- `sitemap` — standalone sitemap/robots.txt generator (`src/bin/sitemap.rs`)

Blog posts live in `blog/*.md` (`.gitignore`d; mounted at runtime in production). Frontmatter uses `MM/DD/YYYY` dates.

## Architecture

- **Router**: Dioxus client-side router (`Route` enum in `src/main.rs`) with `#[layout(Navbar)]` wrapper.
- **Views**:
  - `src/views/` containing UI components.
  - `src/views/projects.rs` holds static project metadata.
- **Blog Engine**: Server functions (`src/backend.rs`) read `blog/*.md`, parse frontmatter via `gray_matter`, and render HTML via `comrak` with custom `syntect` syntax highlighting.
- **Styling**: Driven by Pico CSS v2 (loaded via CDN). Custom overrides and stylesheets live in `assets/main.css`, `assets/blog.css`, `assets/blog_post.css`, `assets/home.css`, and `assets/theme_toggle.css`.
- **Theme Toggle**: Target-gated to WASM only (`#[cfg(target_arch = "wasm32")]`) using `web-sys` to toggle the `data-theme` attribute on the `<html>` tag.
- **Projects**: Static data in `src/views/projects.rs`.

## Commands

| Purpose | Command |
|---|---|
| Dev server (web only) | `dx serve` |
| Dev server (fullstack) | `dx serve --fullstack` |
| Production build + SSG | `dx bundle --release --web --ssg` |
| Sitemap binary | `cargo build --release --bin sitemap` |
| Nix build | `nix build` |
| Docker build | `docker build .` |

## Guardrails & Best Practices

1. **WASM Constraints**: Ensure any web-specific code in components (such as direct DOM interaction via `web-sys`) is gated with `#[cfg(target_arch = "wasm32")]`.
2. **Styling Rules**: Adhere to Pico CSS classes for layout and semantic markup. Customize styling using the files in the `assets/` directory instead of embedding ad-hoc inline styles. Do NOT use TailwindCSS unless explicitly requested. For any color-related style updates, make sure the light theme variant is updated if necessary (e.g. by adding light theme class overrides or updating inversion filters).
3. **No External Linters**: Rely solely on standard cargo tools (`cargo fmt` and `cargo clippy`). No extra linters or formatter configurations are configured in the workspace.

## CI / Deploy

Both workflows are `workflow_dispatch`-only (manual trigger):
- `.github/workflows/build.yml` — pushes to `ghcr.io`
- `.forgejo/workflows/build.yml` — pushes to `git.ruijiang.me`

Docker build uses `dx bundle --release --web --ssg` + `cargo build --release --bin sitemap`. Server expects `blog/` directory mounted at runtime.
