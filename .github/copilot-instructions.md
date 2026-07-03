# Copilot instructions for this repository

## Build, test, and lint commands
- `cargo test --quiet` — runs the Rust test suite (currently compiles/runs with 0 tests).
- `cargo test <test_name>` — run a single test by name substring.
- `cargo build --quiet --bin sitemap` — build the sitemap generator binary used at container startup.
- Production build flow (matches `Dockerfile`): `dx build --ssg --release && cargo build --release --bin sitemap`.
- Lint/format checks for Rust edits: `cargo fmt -- --check` and `cargo clippy --all-targets --all-features`.

## High-level architecture
- This is a **Dioxus fullstack** app (`default_platform = "fullstack"` in `Dioxus.toml`), with routes defined in `src/main.rs`:
  - `/` → `Home`
  - `/blog/:slug` → `Blog`
  - Shared `Navbar` layout wraps routed pages.
- Blog content is file-backed from `./blog/*.md`:
  - `src/backend.rs` exposes `#[server]` functions for blog loading (`get_blog_data`) and latest post lookup (`get_latest_post_slug`).
  - Markdown is rendered with `comrak`, frontmatter is parsed with `gray_matter`, and heading links (`h2`, `h3`) are extracted for the table of contents.
  - Blog metadata types are in `src/models.rs`.
- Syntax highlighting is custom:
  - `src/utils/highlighter.rs` plugs a `CustomHighlighter` into comrak and injects custom Nix syntax from `nix-syntax.yml`.
- Sitemap generation is a separate binary:
  - `src/bin/sitemap.rs` reads blog frontmatter and writes `public/sitemap.xml` and `public/robots.txt`.
  - `start.sh` runs `/app/sitemap` before launching `/app/website`.
- Deployment/build pipelines are container-first:
  - `.github/workflows/build.yml` and `.forgejo/workflows/build.yml` both build and push Docker images via manual dispatch.

## Key conventions
- Blog slugs are expected to be lowercase ASCII/digits/hyphen (`backend.rs` validation); keep filenames and route params aligned with that format.
- Frontmatter date format is `MM/DD/YYYY` (shared `serde` date adapter in `src/utils/date.rs` and mirrored in `src/bin/sitemap.rs`).
- Keep server-only logic behind `#[cfg(feature = "server")]` and `#[server]` functions, following the current split in `src/backend.rs`.
- Blog pages render markdown HTML via `dangerous_inner_html`; keep content sanitization and rendering behavior within the existing comrak pipeline rather than bypassing it in components.
