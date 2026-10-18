---
name: dotconfig-maintenance
description: Maintain the Rust dotconfig installer and config schema.
version: 0.1.0
author: Cristian Bilu, Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [dotfiles, rust, installer, dotconfig]
    related_skills: [dotconfig-import-config]
---

# Maintain dotconfig

Use this skill for changes or fixes to the Rust installer, CLI, RON schema, or build behavior. For application settings imports use `dotconfig-import-config`.

## Procedure

1. Read repository `AGENTS.md`; inspect `dotconfig.rs`, `dotconfig.ron`, `Cargo.toml`, `Makefile`, and relevant README sections. Installer code and unit tests share `dotconfig.rs`.
2. Trace the affected behavior and usages. Preserve the invariants below and add focused regression tests alongside the implementation.
3. Run `terminal(command="cargo test --locked")`, `terminal(command="cargo fmt --check")`, and `terminal(command="cargo clippy --locked --all-targets -- -D warnings")` for installer changes.
4. Run `make install` only when the user needs the changed executable. It installs the release binary; it does not apply configuration. Never use `make` as a build check because it performs full setup and may execute dependency commands or modify machine configuration.
5. Update documentation for user-visible CLI/build changes and summarize verification and any remaining platform-specific limits.

## Behavioral Invariants

- No subcommand runs configured setup; `link <path>` imports only that directory or file and records its platform entry.
- `--dry-run` makes no filesystem or Git configuration changes and executes no installers.
- New optional RON fields have deserialization defaults. Serialization retains dependencies, aliases, copies, and unrelated platform mappings.
- Each `Bash` dependency names exactly one `check`, `Binary("name")` or `Directory("path")`, and is skipped only when that target exists. Keep the single required choice; parallel optional fields let an unrelated match skip an install.
- Imports stay below home after resolving parent symlinks. New destinations enable only the current OS and use the deepest supported placeholder. Never silently overwrite existing RON mappings, stored names, or destinations.
- Prepare and validate RON before moving data. Stage it beside the original for atomic replacement, close handles before renaming, and restore moved data if link creation or config commit fails. A correct symlink can be registered without moving it again.
- The default config path is embedded from `CARGO_MANIFEST_DIR`, while config contents are read from disk. Moving the executable must not change source resolution; `--config` remains an explicit override.
- Normal setup configures `git_aliases` through argument-based `git config --global` and reports failures. A single import does not run full setup.

## Testing Guidance

Use temporary directories and injected home paths; never mutate the real home or global Git config in tests. Cover relevant cases such as unchanged dry-runs, current-OS defaults, content preservation, conflicts, symlink escapes, reruns, and rollback. For embedded config paths, test a relocated binary from another working directory with a dry-run and test a separate explicit config override.

Use `make install` to install changed behavior when requested. README examples should use `make install` for builds, `make` for normal setup, and `dotconfig` for commands with arguments. Do not commit unless asked; if asked, review pre-existing edits and keep unrelated app changes unstaged.

## Verification

Report actual test, formatting, and Clippy results. Confirm no full configuration setup was applied as part of verification.
