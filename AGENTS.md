# wie_app

Web-based feature phone emulator (WIPI/J2ME) using Rust/WebAssembly + TypeScript.

## Commands
- `npm install` - Install dependencies
- `npm run build:dev` - Development build
- `npm run build:prod` - Production build
- `npm start` - Dev server
- `cargo build` - Build Rust (wasm32-unknown-unknown target)
- `cargo test` - Run Rust tests
- `cargo test test_name` - Run single test

## Code Style
- TypeScript: ESNext, no framework (vanilla)
- Rust: Edition 2024, workspace structure
- Use webpack aliases: `@css/`, `@ts/`, `@pkg`
- CSS in `wie_web/src/css/`, HTML in `wie_web/src/html/`
- Korean UI text is acceptable
- camelCase for TS variables/functions, snake_case for Rust
- No comments unless absolutely necessary
