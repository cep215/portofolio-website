# Portfolio Website

Static single-page portfolio built with [Leptos](https://leptos.dev/) (Rust → WASM), deployed to GitHub Pages.

## Prerequisites

- [Rust](https://rustup.rs/) with the `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/) (WASM bundler)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Development

```bash
trunk serve          # http://127.0.0.1:8080 with live-reload
```

## Production Build

```bash
trunk build --release
```

Output goes to `dist/` — ready to deploy as a static site.

## Deploy

Push to `main` and the GitHub Actions workflow (`.github/workflows/deploy.yml`) will build and deploy to GitHub Pages automatically.

## Editing Content

All text, URLs, and links live in **`src/content.rs`**. Change them there — no need to touch component code.

## Project Structure

```
├── src/
│   ├── main.rs          # Leptos components (Header, Hero, Work, etc.)
│   └── content.rs       # All editable text & URLs
├── index.html           # Trunk entry template
├── style.css            # Styles
├── images/              # Static assets (hero image, etc.)
├── CNAME                # Custom domain
└── .github/workflows/   # CI/CD
```
