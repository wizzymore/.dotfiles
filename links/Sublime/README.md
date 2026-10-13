# Sublime Text

Dotconfig links `User` into Sublime Text's `Packages/User` directory on macOS.
Package archives stay on the machine and are not tracked in this repository.

On first launch, `bootstrap_package_control.py` invokes Sublime Text's built-in
Package Control installer if Package Control is missing. Package Control then
automatically installs the packages listed in `User/Package Control.sublime-settings`.
Allow installation to finish; an internet connection is required. If installation
fails, reopen Sublime Text to retry or use **Tools → Install Package Control**.

See [Package Control's syncing documentation](https://packagecontrol.io/docs/syncing).
