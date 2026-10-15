# Repository Guidelines

## Project Structure

This repository manages personal configuration across macOS, Linux, and Windows. The Rust installer and its unit tests live in `dotconfig.rs`; `dotconfig.ron` defines dependencies, symlinks, copies, and global Git aliases. `Cargo.toml` and `Cargo.lock` describe the Rust binary. Machine configuration files and assets live under `links/`, grouped by application (for example, `links/nvim/`, `links/Sublime/`, and `links/ZSH/`). `Brewfile` lists optional macOS packages. Do not add downloaded dependencies or generated build output to the repository.

## Build, Run, and Verify

- `make install` runs `cargo install --path . --locked` to install the release binary into Cargo's bin directory (normally `~/.cargo/bin`), then runs `cargo clean --release` to remove release build artifacts; it does not apply configuration. Debug artifacts stay in `target/debug` for later test and build runs.
- `make` runs normal setup using `dotconfig` from `PATH`. Use `dotconfig --dry-run` for a preview and `dotconfig link /path/to/directory` for a single import.
- `cargo build --locked` builds the installer.
- `cargo run -- --dry-run` previews the configured actions without applying them.
- `cargo run -- --config /path/to/dotconfig.ron --dry-run` previews another configuration; its source paths are relative to that file.
- `cargo test --locked` runs the unit tests in `dotconfig.rs`.
- `cargo fmt --check` and `cargo clippy --locked --all-targets -- -D warnings` check formatting and lints.

Review the dry-run output before applying configuration on a machine. Installer dependencies in `dotconfig.ron` may execute shell commands.

## Importing Application Configuration

Use `dotconfig link` to import directories; the executable is named `dotconfig`, even when the user calls it dotlink or dotfiles. The command moves the directory into `links/`, creates its symlink back, and registers it in `dotconfig.ron`. Moving files and making a symlink alone does not complete an import.

Only directories below the current user's home are accepted. Resolve parent symlinks to prevent escapes; reject the home directory itself, repository overlaps, and conflicting names or destinations. New entries enable only the current OS and leave other OS destinations empty. Use the deepest matching supported placeholder: `%HOME%/src/test`, `%CONFIG%/test`, or a matching data or documents placeholder. Preserve any existing destinations for other platforms when extending an entry.

Check official application documentation and the local installation for config layers, GUI overrides, and state. Capture user settings using the app's supported export when appropriate; preserve runtime state separately. Noctalia's curated config is in `~/.config/noctalia`, while GUI overrides are in `~/.local/state/noctalia/settings.toml`; see `links/noctalia/README.md` for the export workflow.

For import changes, maintain dry-run behavior and rollback if symlink creation or the config update fails. Prepare and validate the RON before moving data; replace it atomically. Rerunning on a correct symlink can register an older import without moving contents again. Verify the contents, symlink, and exactly one appropriate platform entry. RON serialization can reformat the file and does not preserve comments.

The binary embeds its default `dotconfig.ron` path from the build repository. Moving the binary is supported; moving the repository requires rebuilding or `--config`. Config contents remain on disk. Normal setup also applies the optional `git_aliases` configuration to global Git settings; the single-directory `link` operation does not run full setup.

## Agent Skills

- `links/skills/dotconfig-import-config/SKILL.md` covers importing application settings, including Noctalia's GUI export workflow.
- `links/skills/dotconfig-maintenance/SKILL.md` covers changes to the Rust installer, CLI, RON schema, and build behavior.
- `links/skills/` is linked to `%HOME%/.codex/skills` on Linux and macOS by `dotconfig.ron`; Hermes skill copies are installed under `$HERMES_HOME/skills/`. Edit repository sources and refresh installed Hermes copies when changing these skills. Generated built-in skills under `links/skills/.system/` stay local and are excluded from Git tracking.

## Style and Naming

Use standard Rust formatting (`cargo fmt`), idiomatic Rust `snake_case` for functions and variables, and descriptive names for configuration fields. Keep platform-specific paths in the corresponding `Os` fields in `dotconfig.ron`; use `%HOME%`, `%CONFIG%`, and other supported placeholders instead of personal absolute paths. Keep application settings in their matching `links/` subdirectory. Use two spaces for shell configuration indentation and quote paths that may contain spaces.

## Tests

Add focused Rust unit tests alongside the installer code in `dotconfig.rs`, using temporary directories for filesystem behavior. Name tests for their expected behavior, such as `copy_preserves_conflicts_and_is_repeatable`. Run `cargo test --locked` after changing installer logic, and run `cargo fmt --check` and Clippy before submitting.

## Commits and Pull Requests

Recent commits use short, imperative subjects with initial capitalization and no trailing punctuation (for example, `Remove vendored Rust dependencies`). Keep each commit focused. Pull requests should explain the user-visible effect, list verification commands and results, and call out platform-specific behavior. Include screenshots only when a visual configuration change benefits from them.
