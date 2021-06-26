# Publishes skia-bindings and skia-safe to crates.io
# This is temporary and should be automated.
# prerequisites:
#   .cargo/credentials

.PHONY: all
all:
	@echo "make publish: publish the rust-skia packages to crates.io"
	@echo "make publish-only: do not verify or build packages, only publish the the packages"

# test various configuration from inside crates.

.PHONY: crate-tests
crate-tests: crate-bindings-binaries crate-bindings-build

.PHONY: crate-bindings-binaries
crate-bindings-binaries: export FORCE_SKIA_BINARIES_DOWNLOAD=1
crate-bindings-binaries:
	cd skia-bindings && cargo publish -vv --dry-run --features "gl,vulkan,textlayout,binary-cache"
	cd skia-bindings && cargo publish -vv --dry-run 

.PHONY: crate-bindings-build
crate-bindings-build: export FORCE_SKIA_BUILD=1
crate-bindings-build: 
	cd skia-bindings && cargo publish -vv --dry-run --no-default-features --features "gl,vulkan,textlayout,d3d"
	cd skia-bindings && cargo publish -vv --dry-run 

.PHONY: publish
publish: package-bindings package-safe publish-bindings wait publish-safe

.PHONY: publish-only
publish-only: publish-bindings wait publish-safe

.PHONY: publish-bindings
publish-bindings:
	cd skia-bindings && cargo publish -vv --no-verify

.PHONY: publish-safe
publish-safe:
	cd skia-safe && cargo publish -vv --no-verify --allow-dirty

.PHONY: package
package: clean-packages package-bindings package-safe

# bindings are not verifiable, so we do build them by hand.
.PHONY: package-bindings
package-bindings: 
	rm -f target/package/skia-bindings-*.crate
	cd skia-bindings && cargo package -vv --no-verify 
	cd target/package && tar xzf skia-bindings-*.crate
	cd target/package && cargo build -vv --release

.PHONY: package-safe
package-safe:
	rm -f target/package/skia-safe-*.crate
	cd skia-safe && cargo package -vv --no-verify --allow-dirty

.PHONY: clean-packages
clean-packages:
	rm -rf target/package


.PHONY: wait
wait: 
	@echo "published a package, Waiting for crates.io to catch up before publishing the next"
	sleep 20

.PHONY: update-doc
update-doc:
	cargo clean
	rm -rf rust-skia.github.io
	git clone git@github.com:rust-skia/rust-skia.github.io.git
	cd skia-safe && cargo doc --no-deps --lib --features gl,vulkan,d3d,textlayout
	cp -r target/doc rust-skia.github.io/doc
	cd rust-skia.github.io && git add --all
	cd rust-skia.github.io && git commit -m"Auto-Update of /doc" || true
	cd rust-skia.github.io && git push origin master	
	rm -rf rust-skia.github.io

build-flags-win=--release --features "gl,vulkan,d3d,textlayout,webp"

.PHONY: github-build-win
github-build-win:
	cargo clean
	cargo build -p skia-safe ${build-flags-win} --all-targets
	cd cargo clippy ${build-flags-win} --all-targets -- -D warnings 
	cd cargo test --all ${build-flags-win} --all-targets -- --nocapture
	cd cargo run ${build-flags-win}

.PHONY: workflows
workflows:
	cargo run -p mk-workflows

# Tests local builds based on the env vars `SKIA_BUILD_DEFINES` and `SKIA_LIBRARY_SEARCH_PATH`.
#
# This builds a set of libraries, copies them away and then tries to build with the libraries
# referenced through `SKIA_LIBRARY_SEARCH_PATH`.
#
# https://github.com/rust-skia/rust-skia/pull/527

local-build-features=gl,vulkan,webp,textlayout

.PHONY: test-local-build prepare-local-build build-local-build
test-local-build: prepare-local-build build-local-build

prepare-local-build:
	cargo clean
	cargo build --release --features ${local-build-features}
	rm -rf tmp/
	mkdir -p tmp/
	find target -name "libsk*.a" -type f -exec cp {} tmp/ \;
	find target -name "skia-defines.txt" -type f -exec cp {} tmp/ \;
	# Windows
	find target -name "sk*.lib" -type f -exec cp {} tmp/ \;
	find target -name "icudtl.dat" -type f -exec cp {} tmp/ \;

build-local-build:
	cargo clean
	SKIA_SOURCE_DIR=$(shell pwd)/skia-bindings/skia SKIA_BUILD_DEFINES=`cat tmp/skia-defines.txt` SKIA_LIBRARY_SEARCH_PATH=$(shell pwd)/tmp cargo build --release --no-default-features -vv --features ${local-build-features}

