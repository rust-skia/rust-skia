EMSDK ?= $(shell printenv EMSDK)
EMSDK_EXPANDED = $(shell eval echo $(EMSDK))
EMCC ?= $(EMSDK_EXPANDED)/upstream/emscripten/emcc
EMXX ?= $(EMSDK_EXPANDED)/upstream/emscripten/em++
EMAR ?= $(EMSDK_EXPANDED)/upstream/emscripten/emar

WASM_TARGET = wasm32-unknown-emscripten
CRATE_EMCC = wasm-emscripten-testing
EXPORTED_FUNCTIONS = _main,_test_plain_rust_alloc,_test_surface_alloc,_test_surface_alloc_sequential
EMCC_CFLAGS_BASE = -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s MODULARIZE=1 -s ALLOW_MEMORY_GROWTH=1 -s EXPORT_NAME=createRustSkiaEmscriptenTesting -s EXPORTED_FUNCTIONS=[$(EXPORTED_FUNCTIONS)]
WEB_PKG_DIR = web/pkg
DEBUG_JS = target/$(WASM_TARGET)/debug/$(CRATE_EMCC).js
DEBUG_WASM = target/$(WASM_TARGET)/debug/wasm_emscripten_testing.wasm
RELEASE_JS = target/$(WASM_TARGET)/release/$(CRATE_EMCC).js
RELEASE_WASM = target/$(WASM_TARGET)/release/wasm_emscripten_testing.wasm


.PHONY: check
check:
	EMCC_CFLAGS="$(EMCC_CFLAGS_BASE)" EMSDK=$(EMSDK_EXPANDED) CLANGCC=$(EMCC) CLANGCXX=$(EMXX) CC=$(EMCC) CXX=$(EMXX) AR=$(EMAR) cargo check --target $(WASM_TARGET)

.PHONY: build
build:
	@if [ -z "$(EMSDK_EXPANDED)" ]; then echo "EMSDK is not set. Export EMSDK to your emsdk root before running make build."; exit 1; fi
	EMCC_CFLAGS="$(EMCC_CFLAGS_BASE)" EMSDK=$(EMSDK_EXPANDED) CLANGCC=$(EMCC) CLANGCXX=$(EMXX) CC=$(EMCC) CXX=$(EMXX) AR=$(EMAR) cargo build --target $(WASM_TARGET)
	mkdir -p $(WEB_PKG_DIR)
	cp $(DEBUG_JS) $(WEB_PKG_DIR)/wasm-emscripten-testing.js
	cp $(DEBUG_WASM) $(WEB_PKG_DIR)/wasm_emscripten_testing.wasm

.PHONY: build_release
build_release:
	@if [ -z "$(EMSDK_EXPANDED)" ]; then echo "EMSDK is not set. Export EMSDK to your emsdk root before running make build_release."; exit 1; fi
	EMCC_CFLAGS="$(EMCC_CFLAGS_BASE)" EMSDK=$(EMSDK_EXPANDED) CLANGCC=$(EMCC) CLANGCXX=$(EMXX) CC=$(EMCC) CXX=$(EMXX) AR=$(EMAR) cargo build --target $(WASM_TARGET) --release
	mkdir -p $(WEB_PKG_DIR)
	cp $(RELEASE_JS) $(WEB_PKG_DIR)/wasm-emscripten-testing.js
	cp $(RELEASE_WASM) $(WEB_PKG_DIR)/wasm_emscripten_testing.wasm

.PHONY: serve
serve:
	python3 -m http.server --directory web 8002
