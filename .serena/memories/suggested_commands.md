# Suggested Commands for Tombi Development

## Build & Compilation
- `cargo build` - Build the project
- `cargo tombi` - CLI dev tool

## Testing
- `cargo nextest run` - Run all tests
- `cargo test` - Alternative test runner
- `RUST_LOG=trace cargo nextest run -- <TEST_NAME>` - Run single test with logs

## Code Quality
- `cargo clippy --all-targets` - Lint/static analysis
- `cargo fmt --all` - Format code

## Special Commands
- `cargo xtask codegen jsonschema` - Build Tombi JSON schemas
- `cargo xtask toml-test` - Test against toml-test suite
