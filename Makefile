target/release/cleanup: Cargo.toml src/main.rs
	cargo build --release

Cargo.toml:;$(error Cargo.toml is missing)
src/main.rs:;$(error src/main.rs is missing)
