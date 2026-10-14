---
name: dotconfig-maintenance
description: Maintain this user's Rust dotconfig installer, CLI, RON schema, and build behavior. Use for installer feature changes or fixes in the dotfiles repository; app configuration imports use dotconfig-import-config.
---

# Maintain dotconfig

Read repository `AGENTS.md` and inspect `dotconfig.rs`, `dotconfig.ron`, `Cargo.toml`, `Makefile`, and the relevant README sections. Installer code and unit tests share `dotconfig.rs`. Build with `make install`, which runs `cargo install --path . --locked` and installs the release binary into Cargo's bin directory (normally `~/.cargo/bin`). `make` runs normal setup and can execute dependency commands and write machine configuration. Use the single-directory subcommand when the task is a single import.

Preserve these behaviors when extending the installer:

- No subcommand runs configured setup; `link <dir>` imports only that directory and records its platform entry.
- `--dry-run` writes no filesystem or Git configuration changes and executes no installers.
- New optional RON fields need deserialization defaults so old configs still load. Serialization must retain dependencies, aliases, copies, and unrelated platform mappings.
- Imports stay below home after resolving parent symlinks. New destinations enable only the current OS and use the deepest supported placeholder. Existing RON entries and stored directory names must not be silently overwritten.
- Prepare and validate config before moving data. Stage config beside the original for atomic replacement, close file handles before renaming, and restore the moved directory if link creation or config commit fails. A correct existing symlink can be registered without moving it again.
- The default config path is embedded from `CARGO_MANIFEST_DIR`; the config itself is read from disk. Moving the executable must not change source resolution. `--config` remains an explicit override.
- Normal setup configures `git_aliases` through argument-based `git config --global` commands and reports failures. An import does not invoke this full setup.

Add focused tests for behavior affected by the change, using temporary directories and injected home paths instead of modifying the real home or global Git config. Useful cases include unchanged dry-runs, current-OS defaults, preserved directory contents, conflict rejection, symlink escapes, reruns, and rollback. For path embedding, test a relocated binary from a different working directory using a dry-run and a separate explicit config override.

Run the repository's required Rust tests, formatting, and Clippy checks when changing installer logic. Reinstall `dotconfig` with `make install` when the user will run the new behavior. Update README examples to use `make install` for builds, `make` for normal setup, and `dotconfig` for commands needing arguments.

If commits are requested, group changes by coherent features, include their tests and documentation, and keep imported application files with their RON entry. Review pre-existing edits before staging. Do not commit unrelated application changes or apply full setup merely to validate a CLI feature.
