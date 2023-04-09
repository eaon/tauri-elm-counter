# Tauri (Vanilla) + Elm Counter Example

This is a quick and dirty proof-of-concept for passing messages from Tauri's Rust backend as well as a WebAssembly module to Elm's frontend (and vice versa) without relying on the typical JavaScript ecosystem.

## Dependencies

* Rust (… yes)
* Elm 0.19.1 binary in your `PATH`
* `wasm-pack` (i.e. `cargo install wasm-pack`)
* `create-tauri-app` (i.e. `cargo install create-tauri-app`)
* `make`

## Building

`cargo-tauri dev`
