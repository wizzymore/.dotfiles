# Dotfiles

Personal shell, editor, terminal, and font configuration for macOS, Linux, and
Windows. Platform destinations and dependencies are defined in `dotconfig.ron`;
some configurations, including Sublime Text, currently target macOS only.

## Build and preview

Install Rust with support for edition 2024, then run:

```sh
cargo build --locked
./target/debug/dotconfig --dry-run
```

Dry-run lists dependency commands, links, copies, and backups without executing
installers or changing files. It checks configuration paths, but cannot predict
network failures or every write permission error.

To apply the configuration:

```sh
./target/debug/dotconfig
```

Use `--config /absolute/path/to/dotconfig.ron` to run from another directory.
Source paths are relative to the configuration file. Missing sources are errors;
the installer does not import existing machine configuration into the repository.

The optional `Brewfile` lists additional macOS tools and can be applied with
`brew bundle`. Dotconfig installs its own configured dependencies on each run.

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
