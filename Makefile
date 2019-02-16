.PHONY: all
all:
	echo "take a look at the Makefile"

.PHONY: show-bindings
show-bindings: build
    # code will reopen the file while we are formatting it, so
	# better use a temporary file. 
	cp src/bindings.rs /tmp/bindings_inflight.rs
	-rustfmt /tmp/bindings_inflight.rs --force
	cp /tmp/bindings_inflight.rs /tmp/bindings.rs
	rm /tmp/bindings_inflight.rs
	code /tmp/bindings.rs

.PHONY: build
build:
	cargo build

