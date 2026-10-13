# Repository Guidelines

## Project Structure

This repository manages personal configuration across macOS, Linux, and Windows. The Rust installer and its unit tests live in `dotconfig.rs`; `dotconfig.ron` defines dependencies, symlinks, and copies. `Cargo.toml` and `Cargo.lock` describe the Rust binary. Machine configuration files and assets live under `links/`, grouped by application (for example, `links/nvim/`, `links/Sublime/`, and `links/ZSH/`). `Brewfile` lists optional macOS packages. Do not add downloaded dependencies or generated build output to the repository.

## Build, Run, and Verify

- `cargo build --locked` builds the installer.
- `cargo run -- --dry-run` previews the configured actions without applying them.
- `cargo run -- --config /path/to/dotconfig.ron --dry-run` previews another configuration; its source paths are relative to that file.
- `cargo test --locked` runs the unit tests in `dotconfig.rs`.
- `cargo fmt --check` and `cargo clippy --locked --all-targets -- -D warnings` check formatting and lints.

Review the dry-run output before applying configuration on a machine. Installer dependencies in `dotconfig.ron` may execute shell commands.

## Style and Naming

Use standard Rust formatting (`cargo fmt`), idiomatic Rust `snake_case` for functions and variables, and descriptive names for configuration fields. Keep platform-specific paths in the corresponding `Os` fields in `dotconfig.ron`; use `%HOME%`, `%CONFIG%`, and other supported placeholders instead of personal absolute paths. Keep application settings in their matching `links/` subdirectory. Use two spaces for shell configuration indentation and quote paths that may contain spaces.

## Tests

Add focused Rust unit tests alongside the installer code in `dotconfig.rs`, using temporary directories for filesystem behavior. Name tests for their expected behavior, such as `copy_preserves_conflicts_and_is_repeatable`. Run `cargo test --locked` after changing installer logic, and run `cargo fmt --check` and Clippy before submitting.

## Commits and Pull Requests

Recent commits use short, imperative subjects with initial capitalization and no trailing punctuation (for example, `Remove vendored Rust dependencies`). Keep each commit focused. Pull requests should explain the user-visible effect, list verification commands and results, and call out platform-specific behavior. Include screenshots only when a visual configuration change benefits from them.
