all:
	make test
	make format
	make doc

test:
	RUST_BACKTRACE=1 cargo test

format:
	rustfmt --write-mode=overwrite --config-path=rustfmt.toml src/*.rs src/*/*.rs 2>&1 | grep -v "line exceeded maximum length" | grep .

doc:
	cargo doc
