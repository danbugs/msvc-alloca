.PHONY: build-rust-lib
build-rust-lib:
	cd msvc-alloca && cargo build

.PHONY: build-rust-example
build-rust-example:
	cd msvc-alloca-example && cargo build