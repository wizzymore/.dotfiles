# Dotfiles

Personal shell, editor, terminal, and font configuration for macOS, Linux, and
Windows. Platform destinations and dependencies are defined in `dotconfig.ron`;
some configurations, including Sublime Text, currently target macOS only.

## Build and preview

Install Rust with support for edition 2024, then run:

```sh
make install
dotconfig --dry-run
```

`make install` runs `cargo install --path . --locked`, building the release
binary and installing it into Cargo's bin directory (normally `~/.cargo/bin`).
Ensure that directory is on your `PATH`.

Dry-run lists dependency commands, links, copies, and backups without executing
installers or changing files. It checks configuration paths, but cannot predict
network failures or every write permission error.

To apply the configuration:

```sh
make
```

The default config path is embedded at build time and points to this repository's
`dotconfig.ron`. You can move the binary to `/usr/local/bin` and run it from any
directory without setting an environment variable. The repository must remain
at its original location; rebuild or use `--config /path/to/dotconfig.ron` if you
move it. The config contents are read from disk on every run.
Source paths are relative to the configuration file. Missing sources are errors;
normal setup does not import existing machine configuration; use `link` below.

To move one local directory or file into this repository's `links/` directory and
symlink it back to its original location, run:

```sh
dotconfig link /path/to/directory
dotconfig link /path/to/file
```

The path's existing name is used under `links/`. The command also updates
`dotconfig.ron` with a symlink entry for the current OS; other OS destinations
are left empty for new entries. Only paths below your home directory are
accepted, including when resolving symlinks in parent directories. The most
specific supported placeholder is used: for example, `~/src/test` becomes
`%HOME%/src/test`, and `~/.config/test` becomes `%CONFIG%/test` on Linux.
Data and documents directories use their corresponding placeholders.
It rewrites the config with standard RON formatting;
comments are not preserved. Use `--dry-run` to preview without changing files.
Name or destination conflicts are rejected. Rerunning the command on an existing
symlink into `links/` registers it without moving its contents again.

Normal setup also configures the global Git aliases `st` (`status`), `co`
(`checkout`), and `ci` (`commit`) from `dotconfig.ron`. Existing values for
those alias names are updated; `--dry-run` previews the `git config` commands.

The optional `Brewfile` lists additional macOS tools and can be applied with
`brew bundle`. Dotconfig installs its own configured dependencies on each run.

## Codex skills

Normal setup links the entire `links/skills/` directory to `~/.codex/skills/`
on Linux and macOS. It contains the reusable `dotconfig-import-config` and
`dotconfig-maintenance` skills. Edit or add skills in the repository directory.
Codex's generated `.system/` skills stay local and are excluded from Git tracking.

## Backups and recovery

Conflicting destinations are renamed beside the original path, for example
`.zshrc.dotconfig-backup.1`. Existing backup names are never intentionally reused.
An already-correct symlink is skipped. Directory copies merge recursively,
preserving unrelated destination files and backing up changed files individually;
unchanged file contents are skipped. Symlinks inside copied directories are
rejected: configure them as symlink entries instead.

To restore a configuration, move the newly installed destination aside and rename
its backup to the original destination. Keep backups until you have verified the
new configuration. A failed run may have completed other operations; it reports
errors and exits nonzero. Backups remain available after a partial run.

Sublime Text restores its packages on first launch through Package Control; see
[the Sublime setup notes](links/Sublime/README.md).

## Verification

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```
