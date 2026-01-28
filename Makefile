.PHONY: gate fmt clippy test

gate:
	./tools/quality_gate.sh

fmt:
	cargo fmt

clippy:
	cargo clippy -q --workspace --all-targets -- -D warnings

test:
	cargo test -q --workspace
