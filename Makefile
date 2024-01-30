test-expand:
	cd test && RUSTFLAGS='--cfg test' cargo expand

test:
	cd test && cargo test -- --nocapture