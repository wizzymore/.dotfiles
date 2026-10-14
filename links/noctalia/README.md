# Noctalia

Linux configuration linked to `%CONFIG%/noctalia` by dotconfig.
`config.toml` contains a snapshot exported with `noctalia config export`,
including the GUI settings present when it was imported.

Noctalia saves later GUI changes in `~/.local/state/noctalia/settings.toml`,
which overrides this config. To capture those changes in the repository:

```sh
noctalia config export > /tmp/noctalia-config.toml
noctalia config validate /tmp/noctalia-config.toml
cp /tmp/noctalia-config.toml ~/.config/noctalia/config.toml
```

The export and validation must succeed before copying the file.
Plugin settings reference `fel/agent-glow`; the plugin must be installed on
the target machine. Clipboard history, notifications, caches, and other runtime
state are stored separately from this configuration.

See the [Noctalia configuration documentation](https://docs.noctalia.dev/noctalia/configuration/).
