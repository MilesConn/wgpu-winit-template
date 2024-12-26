all: |wasm serve

check-wasm:
	cargo check --target wasm32-unknown-unknown

serve:
	python3 -m http.server --directory html/

wasm:
	wasm-pack build --target web

.PHONY: html-dev wasm all check-wasm
