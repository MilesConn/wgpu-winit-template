all: |wasm serve

serve:
	python3 -m http.server --directory html/

wasm:
	wasm-pack build --target web

.PHONY: html-dev wasm all
