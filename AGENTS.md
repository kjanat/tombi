# Agent Guidelines for Tombi

## Commands
- **Build**: `cargo build`
- **Test all**: `cargo nextest run` (or `cargo test`)
- **Single test**: `RUST_LOG=trace cargo nextest run -- <TEST_NAME>`
- **Lint**: `cargo clippy --all-targets`
- **Format**: `cargo fmt --all`
- **CLI dev**: `cargo tombi`

## Code Style
- Rust 2024 edition, rustfmt 2024 style
- **No `mod.rs`** - use Rust 2018+ style (`foo.rs` + `foo/` dir)
- Comments in English only
- Use `thiserror` for error types, `anyhow` for ad-hoc errors
- Prefer workspace dependencies from root `Cargo.toml`
- Imports: std first, then external crates, then local crates (alphabetized)
- Types: use `derive` macros liberally (`Debug`, `Clone`, `PartialEq`)
- Naming: snake_case for functions/variables, PascalCase for types/traits
- Max function args: 20 (clippy threshold)
- Async: use `tokio`, tests use `#[tokio::test]`
