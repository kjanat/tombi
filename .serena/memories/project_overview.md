# Tombi Project Overview

## Purpose
Tombi is a TOML toolkit providing:
- Formatter
- Linter
- Language Server

Supports multiple editor extensions (VS Code, JetBrains, Zed) and available as CLI tool, Python package, and npm package.

## Tech Stack
- **Language**: Rust (2024 edition)
- **Build**: Cargo
- **Test**: cargo nextest
- **Async runtime**: tokio

## Code Structure
- Main crates in `crates/` directory
- Extensions in `extensions/` (tombi-extension-cargo, tombi-extension-uv, etc.)
- Language server in `crates/tombi-lsp/`
- Editors: VSCode, IntelliJ, Zed
- Python and Rust implementations
