# website

My personal website and blog, built with [Dioxus](https://dioxuslabs.com) 0.7.

This is a full-stack Rust web application that runs as a WASM client in the browser with server-side rendering (SSR) and static site generation (SSG) for content like blog posts and project pages.

## Features

- **Full-stack Dioxus** — client-side routing with SSR fallback, incremental static generation, and out-of-order streaming
- **Markdown blog engine** — blog posts are written as plain `.md` files with YAML frontmatter, parsed via `gray_matter` and rendered to HTML with `comrak`
- **Custom Nix syntax highlighting** — a `syntect`-based highlighter with a custom `.sublime-syntax` definition for the Nix language, applied to code blocks in blog posts
- **Static sitemap generation** — standalone binary (`src/bin/sitemap.rs`) that reads blog metadata and produces `sitemap.xml` and `robots.txt`
- **Responsive layout** — Pico CSS v2 with custom overrides, table-of-contents sidebar on blog posts for tagged headers
- **Nix flake build** — reproducible builds via `flakelight-rust` with the `dioxus-cli`, WASM target, and binaryen

## Reference only

This project is shared for reference only; it is not intended to be deployed directly in another environment. Blog content is managed out-of-band as `.md` files read from the server's filesystem at runtime, and the setup is tailored to my personal infrastructure.
