use colored::Colorize;
use dirs::config_local_dir;
use serde::Deserialize;
use std::{env::current_dir, fmt, fmt::Debug, fs, path::Path};

#[derive(Debug, Deserialize)]
enum OsOrString {
    Os { macos: String, linux: String },
    String(String),
}

impl fmt::Display for OsOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsOrString::Os { macos, linux } => {
                write!(f, "macOS: {}, Linux: {}", macos, linux)
            }
            OsOrString::String(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Location {
    from: String,
    to: OsOrString,
}

#[derive(Debug, Deserialize)]
struct DotConfig {
    symlinks: Vec<Location>,
    copies: Vec<Location>,
}

static CONFIG_STRING: &str = include_str!("dotconfig.ron");

fn format_location(loc: &Location) -> String {
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    let config_dir = config_local_dir().expect("Could not get the config directory");

    let mut s = match &loc.to {
        OsOrString::String(s) => s.clone(),
        OsOrString::Os { macos, linux } => {
            let s;
            if cfg!(target_os = "macos") {
                s = macos.clone();
            } else {
                s = linux.clone();
            }

            s
        }
    };
    s = s.replace("$UNIX_CONFIG", home_dir.join(".config").to_str().unwrap());
    s = s.replace("$CONFIG", config_dir.to_str().unwrap());
    s = s.replace("$HOME", home_dir.to_str().unwrap());
    s
}

fn main() {
    let dot_config: DotConfig = ron::from_str(CONFIG_STRING).expect("Could not parse config file");

    for symlink in dot_config.symlinks {
        let to = format_location(&symlink);
        if to == "" {
            continue;
        }

        dot_link(symlink.from, to);
    }

    for cpy in dot_config.copies {
        let to = format_location(&cpy);
        if to == "" {
            continue;
        }

        copy(cpy.from, to);
    }
}

fn info(s: &str) {
    println!("{}\t{}", " INFO ".bright_green().bold(), s)
}

fn dot_link<T: AsRef<Path>, E: AsRef<Path>>(from: T, to: E) {
    let to = to.as_ref();
    let from_abs = current_dir().unwrap().join(&from);
    if !from_abs.exists() && !to.exists() {
        return;
    }
    if !from_abs.exists() {
        info(format!("Folder not setup, copying over {}", from_abs.display()).as_str());
        if fs::rename(to, &from_abs).is_err() {
            println!(
                "Could not move the {} folder to {}",
                to.display(),
                from_abs.display()
            );
            return;
        }
    }

    if let Ok(m) = fs::symlink_metadata(to) {
        if m.is_symlink() {
            let target = fs::read_link(&to).expect("Could not read the symlink");

            if target == from_abs {
                info(
                    format!(
                        "Skipping {}\n\t {}: \t{}",
                        from_abs.display(),
                        "To".bright_blue(),
                        to.display(),
                    )
                    .as_str(),
                );
                return;
            }
        }

        if m.is_dir() {
            fs::remove_dir_all(to).expect("could not delete the old configuration");
        } else {
            fs::remove_file(to).expect("could not delete the old configuration");
        }
    };

    if std::os::unix::fs::symlink(&from_abs, &to).is_err() {
        println!(
            "could not create symlink from `{}` to `{}`",
            from_abs.display(),
            to.display()
        );
        return;
    }
    info(format!("Linked {}", from_abs.display()).as_str());
}

fn copy<T: AsRef<Path>, E: AsRef<Path>>(from: T, to: E) {
    let meta = fs::symlink_metadata(&from).expect(
        format!(
            "Could not get metadata of copy from {}",
            from.as_ref().display()
        )
        .as_str(),
    );
    if meta.is_dir() {
        let files = fs::read_dir(&from).expect("Could not open fonts directory");
        for file in files.flatten().filter(|file| Path::is_file(&file.path())) {
            let target = to.as_ref().join(file.file_name());
            if let Err(e) = fs::copy(file.path(), &target) {
                println!(
                    "Could not copy {} to {}: {e}",
                    file.file_name().display(),
                    target.display(),
                );
                return;
            };
        }
    } else {
        fs::copy(&from, &to).unwrap_or_else(|e| {
            panic!(
                "Could not copy {} to {}: {e}",
                from.as_ref().display(),
                to.as_ref().display(),
            )
        });
    }

    info(
        format!(
            "Copied {}\n\t {}: \t{}",
            from.as_ref().display(),
            "To".bright_blue(),
            to.as_ref().display(),
        )
        .as_str(),
    );
}
