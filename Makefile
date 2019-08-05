# Publishes skia-bindings and skia-safe to crates.io
# This is temporary and should be automated.
# prerequisites:
#   .cargo/credentials

.PHONY: publish
publish: package
	cd skia-bindings && cargo publish -vv --no-verify
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

