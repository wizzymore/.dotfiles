use colored::Colorize;
use dirs::config_local_dir;
use std::{
    env::current_dir,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};
use thiserror::Error;

static CARGO_PACKAGES: &[&dyn CargoPackage] = &[&Starship {}];

#[derive(Error, Debug)]
pub enum DotConfigError<'a> {
    #[error("Could not install package `{0}`")]
    FailedToInstall(&'a str),
    #[error("{0} already installed")]
    AlreadyInstalled(&'a str),
}

trait CargoPackage: Sync {
    fn get_package_name(&self) -> &str;
    fn after_install(&self) {}
    fn install_package(&self) -> Result<(), DotConfigError> {
        let Ok(cmd) = Command::new("cargo")
            .arg("install")
            .arg(self.get_package_name())
            .output()
        else {
            return Err(DotConfigError::FailedToInstall(self.get_package_name()));
        };

        if let Some(code) = cmd.status.code() {
            if code != 0 {
                return Err(DotConfigError::FailedToInstall(self.get_package_name()));
            }
        }

        self.after_install();

        Ok(())
    }
}

struct Starship;

impl CargoPackage for Starship {
    fn get_package_name(&self) -> &str {
        "starship"
    }
    fn after_install(&self) {
        let home = dirs::home_dir().expect("Could not find the home directory");
        if dot_link("starship/starship.toml", home.join(".config/starship.toml")).is_some() {
            info("Linking Starship config.");
        }
    }
}

fn main() {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let config_dir = config_local_dir().expect("Could not get the config directory");

    match install_bun() {
        Ok(()) => info("Bun installed successfully"),
        Err(err) => info(&format!("{err}")),
    }

    for crt in CARGO_PACKAGES {
        let name = capitalize(crt.get_package_name());
        match crt.install_package() {
            Ok(_) => info(&format!("{name} already installed",)),
            Err(_) => info(&format!("{name} installed successfully",)),
        }
    }

    let ghostty_folder = if cfg!(target_os = "macos") {
        "com.mitchellh.ghostty"
    } else {
        "ghostty"
    };

    if dot_link("ghostty", config_dir.join(ghostty_folder)).is_some() {
        info("Linking Ghostty config.");
    }

    if dot_link("zed", home.join(".config/zed")).is_some() {
        info("Linking Zed config.")
    }

    // ZSH
    if dot_link("ZSH/zshrc", home.join(".zshrc")).is_some() {
        info("Linking ZSH config.");
    };

    // NVIM
    if dot_link("nvim", home.join(".config/nvim")).is_some() {
        info("Linking NVIM config.");
    };
    // Sublime
    if dot_link(
        "Sublime/User",
        home.join("Library/Application Support/Sublime Text/Packages/User"),
    )
    .is_some()
    {
        info("Linking Sublime config.");
    }

    dot_link(
        "Sublime/Installed Packages",
        home.join("Library")
            .join("Application Support")
            .join("Sublime Text")
            .join("Installed Packages"),
    );

    #[cfg(target_os = "macos")]
    copy_fonts("fonts", home.join("Library/Fonts"));
}

fn info(s: &str) {
    println!("{}\t{}", " INFO ".bright_green().bold(), s)
}

fn dot_link<T: AsRef<Path>, E: AsRef<Path>>(from: T, to: E) -> Option<()> {
    let current_dir = current_dir().expect("Could not fetch current directory");
    let from = current_dir.join(from);
    let to = to.as_ref();
    if !from.exists() && !to.exists() {
        return None;
    }
    if !from.exists() {
        info(format!("Folder not setup, copying over {}", from.display()).as_str());
        fs::rename(to, &from).unwrap_or_else(|_| {
            panic!(
                "Could not move the {} folder to {}",
                to.display(),
                from.display()
            )
        });
    }

    if let Ok(m) = fs::symlink_metadata(to) {
        if m.is_symlink() {
            info(format!("Skipping {}", from.display()).as_str());
            return None;
        }

        if m.is_dir() {
            fs::remove_dir_all(to).expect("could not delete the old configuration");
        } else {
            fs::remove_file(to).expect("could not delete the old configuration");
        }
    }

    std::os::unix::fs::symlink(&from, &to).expect("could not create symlink");
    Some(())
}

fn install_bun() -> Result<(), DotConfigError<'static>> {
    let home = dirs::home_dir().expect("Could not find the home directory");
    let meta = fs::symlink_metadata(home.join(".bun"));
    if meta.is_ok() {
        return Err(DotConfigError::AlreadyInstalled("Bun"));
    }

    let Ok(mut cmd) = Command::new("sh")
        .arg("-c")
        .arg("curl -fsSL https://bun.sh/install | bash")
        .spawn()
    else {
        return Err(DotConfigError::FailedToInstall("Bun"));
    };

    if let Some(stdout) = cmd.stdout.as_mut() {
        let stdout_reader = BufReader::new(stdout);

        for line in stdout_reader.lines() {
            println!("{}", line.unwrap())
        }
    }

    cmd.wait().unwrap();

    Ok(())
}

/// Capitalizes the first character in s.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn copy_fonts<T: AsRef<Path>, E: AsRef<Path>>(path: T, target_dir: E) {
    let files = fs::read_dir(&path).expect("Could not open fonts directory");
    for file in files.flatten().filter(|file| Path::is_file(&file.path())) {
        let target = target_dir.as_ref().join(file.file_name());
        fs::copy(file.path(), &target).unwrap_or_else(|e| {
            panic!(
                "Could not copy {} to {}: {e}",
                file.file_name().display(),
                target.display(),
            )
        });
    }
}
