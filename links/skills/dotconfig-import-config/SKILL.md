---
name: dotconfig-import-config
description: Import application settings into dotfiles safely.
version: 0.1.0
author: Cristian Bilu, Hermes Agent
license: MIT
platforms: [linux, macos, windows]
metadata:
  hermes:
    tags: [dotfiles, config-import, dotconfig]
    related_skills: [dotconfig-maintenance]
---

# Import application configuration

Use this procedure to add or repair a managed application configuration. It relies on the repository's `dotconfig link` command; it does not run full machine setup.

## When to Use

- Import or repair an application config under this repository's `links/`.
- Capture GUI overrides using an application's supported export.
- Don't use for installer code, CLI, or RON schema changes; use `dotconfig-maintenance`.

## Procedure

1. Locate the repository in the workspace or at `~/.dotfiles`; read `AGENTS.md` and relevant README guidance. The executable is `dotconfig` even if the user calls it dotlink or dotfiles.
2. Inspect the installed app's config and official documentation if paths, formats, config layers, or export behavior are uncertain. Separate curated settings and GUI overrides from caches, history, and runtime state. For Noctalia, follow [references/noctalia.md](references/noctalia.md).
3. Check `dotconfig --help` and the relevant source before relying on a potentially stale binary. Build with `make install` only if needed. Do not run `make`; it applies the full machine configuration.
4. Preview the exact import, inspect its output, and only then perform the authorized operation:

   `terminal(command="dotconfig link /absolute/path/to/app-config --dry-run")`

   `terminal(command="dotconfig link /absolute/path/to/app-config")`

   The same command imports a single file, such as `dotconfig link /absolute/path/to/app.conf`; it is stored as `links/app.conf`.

   If using another config file, put `--config /path/to/dotconfig.ron` before `link`.
5. Confirm the import: the original path is a symlink to `links/<name>`, contents are preserved, and RON contains exactly one matching platform entry. Note any app portability warnings and whether later GUI edits need another export.

## Constraints and Recovery

Imports of files and directories must remain below the user's home after resolving parent symlinks. Reject the home itself, repository overlaps, name or destination conflicts, and symlinks that do not resolve into `links/`. Preserve the source basename. New entries enable only the current OS, leaving other destinations empty; use the deepest supported placeholder (for example `%HOME%/src/test` or `%CONFIG%/test`). Preserve other-platform destinations when extending an existing RON entry. A correct existing symlink can be registered by rerunning `link`.

Only reset an import when explicitly requested. Verify its symlink target and preserve a recoverable snapshot first; restore the original directory, remove only its RON entry, then rerun `link`. Confirm dry-run changed neither directory nor RON.

## Verification

The import is complete only after checking preserved contents, the exact symlink target, and exactly one appropriate platform entry in `dotconfig.ron`.
