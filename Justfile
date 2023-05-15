build-frontend-dev:
    wasm-pack build --dev --target web src-wasm --out-dir ../public/tauri-elm-counter-wasm
    elm make src-elm/Main.elm --output=public/main.js

build-frontend:
    wasm-pack build --release --target web src-wasm --out-dir ../public/tauri-elm-counter-wasm
    elm make src-elm/Main.elm --output=public/main.js
