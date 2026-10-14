# Noctalia configuration layers

This workflow was confirmed for Noctalia v5. Verify the installed version and current [official configuration documentation](https://docs.noctalia.dev/noctalia/configuration/) before using it for a different release.

- Curated settings: `~/.config/noctalia/config.toml` and other top-level `*.toml` files.
- GUI overrides: `~/.local/state/noctalia/settings.toml`, loaded after curated settings.
- Runtime history, clipboard data, plugin caches, and UI state are separate from the curated config.
- XDG and Noctalia-specific environment overrides can change these locations; inspect the active installation.

Use `noctalia config export` to capture merged user settings. `export full` additionally pins built-in defaults, so use it only when a full snapshot is desired. Export to a temporary file, validate it, and retain the original curated config before replacing it. Avoid redirecting export directly into a file the app reads as input, because shell redirection truncates it before the export runs.

```sh
noctalia config export > /tmp/noctalia-config.toml
noctalia config validate /tmp/noctalia-config.toml
```

After a successful export and validation, import `~/.config/noctalia` using `dotconfig link`; save the merged snapshot to the imported `config.toml` if capturing GUI settings is part of the task. Leave the app's state files in place. Compare parsed merged exports before and after to confirm the effective settings stayed the same.

Later GUI changes still go into the state override file and need re-exporting to enter the repository. A plugin reference in exported settings does not install the plugin on another machine. Report plugin validation warnings and document the required plugin rather than copying its entire cache.
