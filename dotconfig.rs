use colored::Colorize;
use dirs::config_local_dir;
use std::{
    env::current_dir,
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

static CARGO_PACKAGES: &[&str] = &["starship"];

fn main() {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let current_dir = current_dir().expect("Could not get the current directory");
    let config_dir = config_local_dir().expect("Could not get the config directory");
    if let None = install_bun() {
        info("Bun already installed");
    } else {
        info("Bun installed successfully");
    }

    for c in CARGO_PACKAGES {
        if let None = install_cargo_dep(c) {
            info(format!("{} already installed", capitalize(c)).as_str());
        } else {
            info(format!("{} installed successfully", capitalize(c)).as_str());
        }
    }

    // Ghostty
    let mut ghostty_folder = "ghostty";
    if cfg!(target_os = "macos") {
        ghostty_folder = "com.mitchellh.ghostty";
    }
    if let Some(_) = dot_link(
        &current_dir.join("ghostty"),
        &config_dir.join(ghostty_folder),
    ) {
        info("Linking Ghostty config.");
    }

    if dot_link(&current_dir.join("zed"), &home.join(".config").join("zed")).is_some() {
        info("Linking Zed config.")
    }

    // ZSH
    if let Some(_) = dot_link(&current_dir.join("ZSH").join("zshrc"), &home.join(".zshrc")) {
        info("Linking ZSH config.");
    };

    // NVIM
    if let Some(_) = dot_link(
        &current_dir.join("nvim"),
        &home.join(".config").join("nvim"),
    ) {
        info("Linking NVIM config.");
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
        info("Linking Sublime config.");
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

fn info(s: &str) {
    println!("{}\t{}", " INFO ".bright_green().bold(), s)
}

fn dot_link(from: &PathBuf, to: &PathBuf) -> Option<()> {
    if !from.exists() && !to.exists() {
        return None;
    }
    if !from.exists() && to.exists() {
        info(format!("Folder not setup, copying over {}", from.display()).as_str());
        fs::rename(to, from).expect(
            format!(
                "Could not move the {} folder to {}",
                to.display(),
                from.display()
            )
            .as_str(),
        );
    }
    let meta = fs::symlink_metadata(to);
    if let Ok(m) = meta {
        if m.is_symlink() && to.exists() {
            info(format!("Skipping {}", from.display()).as_str());
            return None;
        }

        if m.is_dir() {
            fs::remove_dir_all(to).expect("could not delete the old configuration");
        } else {
            fs::remove_file(to).expect("could not delete the old configuration");
        }
    }

    std::os::unix::fs::symlink(from, to).expect("could not create symlink");

    Some(())
}

fn install_bun() -> Option<()> {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let meta = fs::symlink_metadata(home.join(".bun"));
    if meta.is_ok() {
        return None;
    }

    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://bun.sh/install | bash")
        .spawn()
        .expect("Failed to install bun");

    if let Some(stdout) = cmd.stdout.as_mut() {
        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            println!("{}", line.unwrap())
        }
    }

    cmd.wait().unwrap();

    Some(())
}

fn install_cargo_dep(dep: &str) -> Option<()> {
    let cmd = Command::new("cargo")
        .arg("install")
        .arg(dep)
        .output()
        .expect("Failed to install starship");

    if let Some(code) = cmd.status.code() {
        if code != 0 {
            return None;
        }
    }

    Some(())
}

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
