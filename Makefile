build-frontend-dev:
	wasm-pack build --target web src-wasm --out-dir ../public/tauri-elm-counter-wasm
	elm make src/Main.elm --output=public/main.js
