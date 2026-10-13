"""Install Package Control on first launch; it restores installed_packages."""

import os

import sublime


def ensure_package_control():
    archive = os.path.join(
        sublime.installed_packages_path(), "Package Control.sublime-package"
    )
    unpacked = os.path.join(sublime.packages_path(), "Package Control")
    if not os.path.exists(archive) and not os.path.isdir(unpacked):
        # Use Sublime's installer, including its package signature verification.
        sublime.run_command("install_package_control")


def plugin_loaded():
    # Wait until Sublime has registered the commands from its Default package.
    sublime.set_timeout(ensure_package_control, 1000)
