use colored::Colorize;
use dirs::config_local_dir;
use std::{env::current_dir, fs, path::PathBuf, process::Command};

fn main() {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let current_dir = current_dir().expect("Could not get the current directory");
    let config_dir = config_local_dir().expect("Could not get the config directory");
    if let None = install_bun() {
        info("Bun already installed".into());
    } else {
        info("Bun installed successfully".into());
    }

    info(format!(
        "Path is {}",
        config_dir.clone().into_os_string().into_string().unwrap()
    ));

    // Ghostty
    let mut ghostty_folder = "ghostty";
    if cfg!(target_os = "macos") {
        ghostty_folder = "com.mitchellh.ghostty";
    }
    if let Some(_) = dot_link(
        &current_dir.join("ghostty"),
        &config_dir.join(ghostty_folder),
    ) {
        info("Linking Ghostty config.".into());
    }

    // ZSH
    if let Some(_) = dot_link(
        &current_dir.join("ZSH").join(".zshrc"),
        &home.join(".zshrc"),
    ) {
        info("Linking ZSH config.".into());
    };

    // NVIM
    if let Some(_) = dot_link(
        &current_dir.join("nvim"),
        &home.join(".config").join("nvim"),
    ) {
        info("Linking NVIM config.".into());
    };
    // Sublime
    if let Some(_) = dot_link(
        &current_dir.join("Sublime").join("User"),
        &home
            .join("Library")
            .join("Application Support")
            .join("Sublime Text")
            .join("Packages")
            .join("User"),
    ) {
        info("Linking Sublime config.".into());
    }
    dot_link(
        &current_dir.join("Sublime").join("Installed Packages"),
        &home
            .join("Library")
            .join("Application Support")
            .join("Sublime Text")
            .join("Installed Packages"),
    );
}

fn info(s: String) {
    println!("{}\t{}", " INFO ".bright_green().bold(), s)
}

fn dot_link(from: &PathBuf, to: &PathBuf) -> Option<()> {
    let meta = fs::symlink_metadata(to);
    if let Ok(m) = meta {
        if m.is_symlink() {
            info(format!(
                "Skipping {}",
                from.clone().into_os_string().into_string().unwrap()
            ));
            return None;
        }

        if m.is_dir() {
            Command::new("rm")
                .arg("-rf")
                .arg(to)
                .status()
                .expect("could not delete old symlink");
        } else {
            Command::new("rm")
                .arg(to)
                .status()
                .expect("could not delete old symlink");
        }
    }

    Command::new("ln")
        .arg("-s")
        .arg(from)
        .arg(to)
        .status()
        .expect("could not create symlink");

    Some(())
}

fn install_bun() -> Option<()> {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let meta = fs::symlink_metadata(home.join(".bun"));
    if meta.is_ok() {
        return None;
    }
    Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://bun.sh/install | bash")
        .output()
        .expect("Failed to install bun");

    Some(())
}
