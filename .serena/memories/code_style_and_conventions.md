# Tombi Code Style and Conventions

## Rust Edition & Formatting
- Rust 2024 edition
- rustfmt 2024 style
- No `mod.rs` files (use Rust 2018+ style: `foo.rs` with `foo/` dir)

## Error Handling
- Use `thiserror` for error types
- Use `anyhow` for ad-hoc errors

## Naming Conventions
- Functions/variables: snake_case
- Types/traits: PascalCase

## Code Organization
- Imports: std first, external crates, then local crates (alphabetized)
- Types: use derive macros liberally (Debug, Clone, PartialEq)
- Comments: English only
- Max function args: 20

## Dependencies
- Use workspace dependencies from root Cargo.toml
- Async: tokio
- Tests: #[tokio::test]
