test-expand:
	RUSTFLAGS='--cfg test' cargo expand

test:
	cargo test -- --nocapture