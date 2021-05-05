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
	cd skia-bindings && cargo publish -vv --dry-run --features "gl,vulkan,textlayout"
	cd skia-bindings && cargo publish -vv --dry-run 

.PHONY: crate-bindings-build
crate-bindings-build: export FORCE_SKIA_BUILD=1
crate-bindings-build: 
	cd skia-bindings && cargo publish -vv --dry-run --features "gl,vulkan,textlayout,d3d"
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

.PHONY: azure-build-win
azure-build-win:
	cargo clean
	cd skia-safe && cargo build ${build-flags-win} --all-targets
	cd skia-org && cargo clippy ${build-flags-win} --all-targets -- -D warnings 
	cd skia-org && cargo test --all ${build-flags-win} --all-targets -- --nocapture
	cd skia-org && cargo run ${build-flags-win}

