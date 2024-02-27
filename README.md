# Tauri (Vanilla) + Elm Counter Example

This is a quick and dirty proof-of-concept for passing messages from Tauri's Rust backend as well as a WebAssembly module to Elm's frontend (and vice versa) without relying on the typical JavaScript ecosystem.

In this project, the counter is incremented via Tauri's Rust backend, decremented via WebAssembly, and reset via Elm.

## Dependencies

* Rust (… yes)
* [Elm](https://elm-lang.org/) 0.19.1 binary in your `PATH`
* [`wasm-pack`](https://crates.io/crates/wasm-pack) (i.e. `cargo install wasm-pack`)
* [`create-tauri-app`](https://crates.io/crates/create-tauri-app) (i.e. `cargo install create-tauri-app`)
* [`just`](https://crates.io/crates/just) (i.e. `cargo install just`)

## Getting started

```sh
just run-dev
```
