test: build-smoketest test-std test-no-std test-serde-no-std test-colors-no-std test-color-new-no-std test-alloc-no-std

build-smoketest:
	cargo build
	cargo build --no-default-features
	cargo build --no-default-features --target wasm32-wasi

test-std:
	cargo test
test-no-std:
	cargo test --no-default-features
test-serde-no-std:
	cargo test --no-default-features --features "serde"
test-colors-no-std:
	cargo test --no-default-features --features "colors"
test-color-new-no-std:
	cargo test --no-default-features --features "color-new"
test-alloc-no-std:
	cargo test --no-default-features --features "alloc"
