# wie_app

Web-based feature phone emulator (WIPI/J2ME) using Rust + WebAssembly.

## Project Structure

- `wie_web/` - Web frontend
  - `src/html/` - HTML templates
  - `src/css/` - Stylesheets
  - `src/ts/` - TypeScript source
  - `src/rust/` - Rust/WASM source
  - `pkg/` - Generated WASM package
- `wie_tauri/` - Tauri desktop app

## Tech Stack

- Rust + wasm-bindgen for core emulator
- TypeScript for web UI
- Webpack with html-bundler-webpack-plugin
- No frontend framework (vanilla JS/TS)

## Build

```bash
npm run build:dev   # development build
npm run build:prod  # production build
npm start           # dev server
```

## Code Conventions

- Use webpack aliases: `@css/`, `@ts/`, `@pkg`
- CSS in separate files under `src/css/`
- Korean UI text is acceptable
