# AGENTS.md

## UI/UX Design

- **Aesthetic**: The website targets a TUI-like (Terminal User Interface) aesthetic.
- **Typography/Fonts**:
  - **Fira Mono** is used in most places to maintain the terminal/command-line feel.
  - **Fira Sans** is used specifically in blog posts to reduce reading fatigue during long articles.
- **Code Blocks**: The custom syntax highlighter injects a Vim-inspired UI for all rendered code blocks.
- **CLI Prompt & Fastfetch**: The Home page includes a component (`div`) that mimics a CLI prompt (styled with a Starship layout) and displays a `fastfetch` output utilizing real-time server stats fetched from the host.

## Guardrails & Best Practices

1. **WASM Constraints**: Ensure any web-specific code in components (such as direct DOM interaction via `web-sys`) is gated with `#[cfg(target_arch = "wasm32")]`.
2. **Styling Rules**: Adhere to Pico CSS classes for layout and semantic markup. Customize styling using the files in the `assets/` directory instead of embedding ad-hoc inline styles. Do NOT use TailwindCSS unless explicitly requested. For any color-related style updates, make sure the light theme variant is updated if necessary (e.g. by adding light theme class overrides or updating inversion filters).
3. **No External Linters**: Rely solely on standard cargo tools (`cargo fmt` and `cargo clippy`). No extra linters or formatter configurations are configured in the workspace.
4. **Blog Slugs**: Slugs are expected to be lowercase ASCII/digits/hyphen (`backend.rs` validation); keep filenames and route params aligned with that format.
5. **Date Format**: Frontmatter date format is `MM/DD/YYYY` (shared `serde` date adapter in `src/utils/date.rs` and mirrored in `src/bin/sitemap.rs`).
6. **Server Logic**: Keep server-only logic behind `#[cfg(feature = "server")]` and `#[server]` functions, following the current split in `src/backend.rs`.
7. **Blog Rendering**: Blog pages render markdown HTML via `dangerous_inner_html`; keep content sanitization and rendering behavior within the existing comrak pipeline rather than bypassing it in components.
