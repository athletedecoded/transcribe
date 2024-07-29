format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

tests:
	cargo test

binary:
	cargo build --release

cargo-lambda:
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall cargo-lambda
	chmod +x zig-installer.sh && ./zig-installer.sh
