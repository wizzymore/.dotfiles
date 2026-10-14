---
name: dotconfig-import-config
description: Import an installed application's configuration into this user's dotfiles repository using dotconfig link, with persistent OS-specific RON entries. Use when adding or repairing a managed app config, including Hyprland or Noctalia.
---

# Import application configuration

Locate the dotfiles repository in the workspace or at `~/.dotfiles`, and read its `AGENTS.md`. The executable is `dotconfig`; the user may call it dotlink or dotfiles. Inspect `bin/dotconfig --help` and the source before assuming a stale binary supports the current behavior. Build with `make install` when necessary. `make` applies the entire configuration and is not needed for a single import.

Find the application's actual config locally and consult its official documentation when paths, formats, or layers are uncertain. Distinguish curated settings, GUI overrides, plugins, and runtime state. Use a supported merged export when it is needed to capture the user's settings; do not import caches or runtime history just because they sit beside settings. For Noctalia, read [references/noctalia.md](references/noctalia.md).

Use the installed tool for the move and registration:

```sh
./bin/dotconfig link /absolute/path/to/app-config --dry-run
./bin/dotconfig link /absolute/path/to/app-config
```

Review the preview before applying the import within the user's authorized scope. If the binary needs another config, put `--config /path/to/dotconfig.ron` before `link`.

Imports must stay below the user's home after resolving parent symlinks. The home itself and directories overlapping the repository's `links/` are invalid. Preserve the directory basename and reject collisions. New RON entries enable only the current OS, with other destinations empty. Choose the deepest supported placeholder: `~/src/test` becomes `%HOME%/src/test`, and `~/.config/test` becomes `%CONFIG%/test` on Linux. Preserve other-platform destinations when registering an existing source.

The operation is complete when the contents are preserved, the original path is a symlink to `links/<name>`, and RON contains exactly one matching platform entry. A correct symlink imported by an older version can be registered by rerunning `link`. Document any later GUI changes that require a fresh export. Mention application validation warnings that affect portability.

When explicitly asked to reset and retry an import, verify the current symlink target and keep a recoverable config snapshot. Restore the original folder, remove only its RON entry, then rerun the tool. Compare contents and confirm dry-run did not mutate either the folder or RON. This retry procedure is conditional on that request, not a routine part of every import.
