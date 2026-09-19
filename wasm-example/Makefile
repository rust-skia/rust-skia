# Resolve the Emscripten SDK root:
# 1. $EMSDK from the environment wins (all assignments below are skipped then).
# 2. A ~/emsdk checkout. The SDK layout (classic/flat/activated) inside is
#    detected by the skia-bindings build script itself, so only the root is
#    probed here.
# 3. The asdf-managed install (~/.asdf/installs/emsdk/<version>), if asdf has emsdk.
ifeq ($(EMSDK),)
EMSDK := $(shell test -d "$(HOME)/emsdk" && echo "$(HOME)/emsdk")
endif
ifeq ($(EMSDK),)
EMSDK := $(shell command -v asdf >/dev/null 2>&1 && v=`asdf current emsdk 2>/dev/null | tr -s ' ' | cut -d ' ' -f 2` && test -n "$$v" && echo "$$HOME/.asdf/installs/emsdk/$$v")
endif
ifeq ($(EMSDK),)
$(warning could not find the Emscripten SDK; set it with 'make build EMSDK=<sdk-root>' or export EMSDK)
endif

# emsdk_env.sh sets up the environment (EMSDK, PATH with em++, ...) itself,
# independent of the SDK layout inside the root.
BUILD = EMCC_CFLAGS="-s ERROR_ON_UNDEFINED_SYMBOLS=0 -s MAX_WEBGL_VERSION=2 -s MODULARIZE=1 -s EXPORT_NAME=createRustSkiaModule -s EXPORTED_RUNTIME_METHODS=GL" source "$$EMSDK/emsdk_env.sh" >/dev/null && EMSDK=$$EMSDK cargo build --target wasm32-unknown-emscripten

.PHONY: build
build:
	$(BUILD)

.PHONY: build_release
build_release:
	$(BUILD) --release

.PHONY: serve
serve:
	python3 -m http.server
