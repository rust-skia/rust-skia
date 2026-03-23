WASM_TARGET = wasm32-unknown-unknown
CRATE_WASM = wasm_unknown_example
RELEASE_INITIAL_MEMORY_BYTES ?= 16777216

.PHONY: build
build:
	cargo build --target $(WASM_TARGET)
	wasm-bindgen --target web --no-typescript --out-dir web/pkg target/$(WASM_TARGET)/debug/$(CRATE_WASM).wasm

.PHONY: build_release
build_release:
	RUSTFLAGS="$(RUSTFLAGS) -C link-arg=--initial-memory=$(RELEASE_INITIAL_MEMORY_BYTES)" cargo build --target $(WASM_TARGET) --release
	wasm-bindgen --target web --no-typescript --out-dir web/pkg target/$(WASM_TARGET)/release/$(CRATE_WASM).wasm

.PHONY: serve
serve:
	python3 -m http.server --directory web 8000
