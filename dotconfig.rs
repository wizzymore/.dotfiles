use colored::Colorize;
use std::{env::current_dir, fs, path::PathBuf, process::Command};

fn main() {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let current_dir = current_dir().expect("Could not get the current directory");
    if !install_bun() {
        info(&"Bun already installed".into());
    }

    // ZSH
    dot_link(
        &current_dir.join("ZSH").join(".zshrc"),
        &home.join(".zshrc"),
    );

    // Sublime
    dot_link(
        &current_dir.join("Sublime").join("User"),
        &home
            .join("Library")
            .join("Application Support")
            .join("Sublime Text")
            .join("Packages")
            .join("User"),
    );
    dot_link(
        &current_dir.join("Sublime").join("Installed Packages"),
        &home
            .join("Library")
            .join("Application Support")
            .join("Sublime Text")
            .join("Installed Packages"),
    );
}

fn info(s: &String) {
    println!("{}\t{}", " INFO ".white().bold().on_bright_green(), s)
}

fn dot_link(from: &PathBuf, to: &PathBuf) {
    let meta = fs::symlink_metadata(to);
    if let Ok(m) = meta {
        if m.is_symlink() {
            info(&format!(
                "Skipping {}",
                from.clone().into_os_string().into_string().unwrap()
            ));
            return;
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
}

fn install_bun() -> bool {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let meta = fs::symlink_metadata(home.join(".bun"));
    if meta.is_ok() {
        return false;
    }
    Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://bun.sh/install | bash")
        .output()
        .expect("Failed to install bun");

    true
}
