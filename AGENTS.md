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

- **Router**: Dioxus client-side router (`Route` enum in `src/main.rs`) with `#[layout(Navbar)]` wrapper. The main routes are:
  - `/` → `Home`
  - `/blog/:slug` → `Blog`
- **Views**:
  - `src/views/` containing UI components.
  - `src/views/projects.rs` holds static project metadata.
- **Blog Engine**: Server functions (`src/backend.rs`) read `blog/*.md`, parse frontmatter via `gray_matter`, and render HTML via `comrak` with custom `syntect` syntax highlighting. It exposes `#[server]` functions for blog loading (`get_blog_data`) and latest post lookup (`get_latest_post_slug`). Heading links (`h2`, `h3`) are extracted for the table of contents. Blog metadata types are located in `src/models.rs`.
- **Syntax Highlighting**: Powered by a custom syntax highlighter in `src/utils/highlighter.rs` which plugs into `comrak` and injects Nix syntax from `nix-syntax.yml`.
- **Styling**: Driven by Pico CSS v2 (loaded via CDN). Custom overrides and stylesheets live in `assets/main.css`, `assets/blog.css`, `assets/blog_post.css`, `assets/home.css`, and `assets/theme_toggle.css`.
- **Theme Toggle**: Target-gated to WASM only (`#[cfg(target_arch = "wasm32")]`) using `web-sys` to toggle the `data-theme` attribute on the `<html>` tag.
- **Sitemap Generator**: Standalone binary `src/bin/sitemap.rs` reads blog frontmatter and writes `public/sitemap.xml` and `public/robots.txt`. The startup script `start.sh` runs `/app/sitemap` before launching `/app/website`.
- **Projects**: Static data in `src/views/projects.rs`.

## UI/UX Design

- **Aesthetic**: The website targets a TUI-like (Terminal User Interface) aesthetic.
- **Typography/Fonts**:
  - **Fira Mono** is used in most places to maintain the terminal/command-line feel.
  - **Fira Sans** is used specifically in blog posts to reduce reading fatigue during long articles.
- **Code Blocks**: The custom syntax highlighter injects a Vim-inspired UI for all rendered code blocks.
- **CLI Prompt & Fastfetch**: The Home page includes a component (`div`) that mimics a CLI prompt (styled with a Starship layout) and displays a `fastfetch` output utilizing real-time server stats fetched from the host.

## Commands

| Purpose | Command |
|---|---|
| Dev server (web only) | `dx serve` |
| Dev server (fullstack) | `dx serve --fullstack` |
| Run test suite | `cargo test --quiet` |
| Run single test | `cargo test <test_name>` |
| Production build + SSG | `dx bundle --release --web --ssg` |
| Sitemap binary | `cargo build --release --bin sitemap` |
| Nix build | `nix build` |
| Docker build | `docker build .` |
| Check formatting | `cargo fmt -- --check` |
| Lint code | `cargo clippy --all-targets --all-features` |

## Guardrails & Best Practices

1. **WASM Constraints**: Ensure any web-specific code in components (such as direct DOM interaction via `web-sys`) is gated with `#[cfg(target_arch = "wasm32")]`.
2. **Styling Rules**: Adhere to Pico CSS classes for layout and semantic markup. Customize styling using the files in the `assets/` directory instead of embedding ad-hoc inline styles. Do NOT use TailwindCSS unless explicitly requested. For any color-related style updates, make sure the light theme variant is updated if necessary (e.g. by adding light theme class overrides or updating inversion filters).
3. **No External Linters**: Rely solely on standard cargo tools (`cargo fmt` and `cargo clippy`). No extra linters or formatter configurations are configured in the workspace.
4. **Blog Slugs**: Slugs are expected to be lowercase ASCII/digits/hyphen (`backend.rs` validation); keep filenames and route params aligned with that format.
5. **Date Format**: Frontmatter date format is `MM/DD/YYYY` (shared `serde` date adapter in `src/utils/date.rs` and mirrored in `src/bin/sitemap.rs`).
6. **Server Logic**: Keep server-only logic behind `#[cfg(feature = "server")]` and `#[server]` functions, following the current split in `src/backend.rs`.
7. **Blog Rendering**: Blog pages render markdown HTML via `dangerous_inner_html`; keep content sanitization and rendering behavior within the existing comrak pipeline rather than bypassing it in components.

## CI / Deploy

Both workflows are `workflow_dispatch`-only (manual trigger):
- `.github/workflows/build.yml` — pushes to `ghcr.io`
- `.forgejo/workflows/build.yml` — pushes to `git.ruijiang.me`

Docker build uses `dx bundle --release --web --ssg` + `cargo build --release --bin sitemap`. Server expects `blog/` directory mounted at runtime.
