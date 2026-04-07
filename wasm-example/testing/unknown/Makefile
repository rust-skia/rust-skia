WASM_TARGET = wasm32-unknown-unknown
CRATE_WASM = wasm_unknown_testing
DEBUG_INITIAL_MEMORY_BYTES ?= 134217728
DEBUG_MAX_MEMORY_BYTES ?= 2147483648
RELEASE_INITIAL_MEMORY_BYTES ?= 134217728
RELEASE_MAX_MEMORY_BYTES ?= 2147483648

.PHONY: build
build:
	RUSTFLAGS="$(RUSTFLAGS) -C link-arg=--initial-memory=$(DEBUG_INITIAL_MEMORY_BYTES) -C link-arg=--max-memory=$(DEBUG_MAX_MEMORY_BYTES)" cargo build --target $(WASM_TARGET)
	wasm-bindgen --target web --no-typescript --out-dir web/pkg target/$(WASM_TARGET)/debug/$(CRATE_WASM).wasm

.PHONY: build_release
build_release:
	RUSTFLAGS="$(RUSTFLAGS) -C link-arg=--initial-memory=$(RELEASE_INITIAL_MEMORY_BYTES) -C link-arg=--max-memory=$(RELEASE_MAX_MEMORY_BYTES)" cargo build --target $(WASM_TARGET) --release
	wasm-bindgen --target web --no-typescript --out-dir web/pkg target/$(WASM_TARGET)/release/$(CRATE_WASM).wasm

.PHONY: serve
serve:
	python3 -m http.server --directory web 8001
