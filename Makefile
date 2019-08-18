# Publishes skia-bindings and skia-safe to crates.io
# This is temporary and should be automated.
# prerequisites:
#   .cargo/credentials

.PHONY: all
all:
	@echo "make publish: publish the rust-skia packages to crates.io"
	@echo "make publish-only: do not verify or build packages, only publish the the packages"

.PHONY: publish
publish: package-bindings package-safe publish-bindings wait publish-safe

.PHONY: publish-only
publish-only: publish-bindings publish-safe

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
	cd skia-bindings && cargo package -vv --no-verify 
	cd target/package && tar xzf skia-bindings-*.crate
	cd target/package && cargo build -vv --release

.PHONY: package-safe
package-safe:
	cd skia-safe && cargo package -vv --no-verify --allow-dirty

.PHONY: clean-packages
clean-packages:
	rm -rf target/package


.PHONY: wait
wait: 
	@echo "published a package, Waiting for crates.io to catch up before publishing the next"
	sleep 10
