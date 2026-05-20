# AGENTS.md

## Project structure

Personal website — full-stack Dioxus 0.7 app (Rust, edition 2024). Two feature sets:
- `web` (default) — WASM client
- `server` — SSR/SSG with Markdown blog rendering

Two binaries:
- `website` — main Dioxus app (entry: `src/main.rs`)
- `sitemap` — standalone sitemap/robots.txt generator (`src/bin/sitemap.rs`)

Blog posts live in `blog/*.md` (`.gitignore`d; mounted at runtime in production). Frontmatter uses `MM/DD/YYYY` dates.

## Commands

| Purpose | Command |
|---|---|
| Dev server (web only) | `dx serve` |
| Dev server (fullstack) | `dx serve --fullstack` |
| Production build + SSG | `dx bundle --release --web --ssg` |
| Sitemap binary | `cargo build --release --bin sitemap` |
| Nix build | `nix build` |
| Docker build | `docker build .` |

No tests, linters, or formatters are configured beyond what cargo provides.

## Architecture

- **Router**: Dioxus client-side router (`Route` enum in `src/main.rs`) with `#[layout(Navbar)]` wrapper
- **Blog**: Server functions (`src/backend.rs`) read `blog/*.md`, parse frontmatter via `gray_matter`, render HTML via `comrak` with `syntect` syntax highlighting
- **Styling**: Pico CSS v2 (CDN) + `assets/main.css` + `assets/theme_toggle.css`
- **Theme toggle**: WASM-only (`#[cfg(target_arch = "wasm32")]`) via `web-sys`, sets `data-theme` on `<html>`
- **Projects**: Static data in `src/views/projects.rs`

## CI / Deploy

Both workflows are `workflow_dispatch`-only (manual trigger):
- `.github/workflows/build.yml` — pushes to `ghcr.io`
- `.forgejo/workflows/build.yml` — pushes to `git.ruijiang.me`

Docker build uses `dx bundle --release --web --ssg` + `cargo build --release --bin sitemap`. Server expects `blog/` directory mounted at runtime.
