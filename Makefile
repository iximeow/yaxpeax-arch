test: test-std test-no-std test-serde-no-std test-alloc-no-std

test-std:
	cargo test
test-no-std:
	cargo test --no-default-features
test-serde-no-std:
	cargo test --no-default-features --features "serde"
test-alloc-no-std:
	cargo test --no-default-features --features "alloc"
