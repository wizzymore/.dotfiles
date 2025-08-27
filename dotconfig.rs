use colored::Colorize;
use dirs::config_local_dir;
use serde::Deserialize;
use std::{
    env::current_dir,
    fmt::{self, Debug},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

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
enum Dependency {
    Cargo {
        name: String,
        #[serde(default)]
        git: Option<String>,
    },
    Bash {
        name: String,
        command: String,
        #[serde(default)]
        binary: Option<String>,
        #[serde(default)]
        directory: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
struct DotConfig {
    symlinks: Vec<Location>,
    copies: Vec<Location>,
    dependencies: Vec<Dependency>,
}

static CONFIG_STRING: &str = include_str!("dotconfig.ron");

fn format_location(loc: &Location) -> String {
    let s = match &loc.to {
        OsOrString::String(s) => s.clone(),
        #[cfg(target_os = "macos")]
        OsOrString::Os { macos, linux: _ } => macos.clone(),
        #[cfg(target_os = "linux")]
        OsOrString::Os { macos: _, linux } => linux.clone(),
    };
    format_path(s)
}

fn format_path(s: String) -> String {
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    let config_dir = config_local_dir().expect("Could not get the config directory");
    let mut s = s;
    s = s.replace("$UNIX_CONFIG", home_dir.join(".config").to_str().unwrap());
    s = s.replace("$CONFIG", config_dir.to_str().unwrap());
    s = s.replace("$HOME", home_dir.to_str().unwrap());
    s
}

fn info<T>(s: T)
where
    T: AsRef<str>,
{
    println!("{}\t{}", " INFO ".bright_green().bold(), s.as_ref())
}

fn error<T>(s: T)
where
    T: AsRef<str>,
{
    println!("{}\t{}", " ERROR ".bright_red().bold(), s.as_ref())
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
            error(format!(
                "Could not move the {} folder to {}",
                to.display(),
                from_abs.display()
            ));
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
        error(format!(
            "could not create symlink from `{}` to `{}`",
            from_abs.display(),
            to.display()
        ));
        return;
    }
    info(format!("Linked {}", from_abs.display()));
}

fn copy<T: AsRef<Path>, E: AsRef<Path>>(from: T, to: E) {
    let meta = fs::symlink_metadata(&from).unwrap_or_else(|_| {
        panic!(
            "Could not get metadata of copy from {}",
            from.as_ref().display()
        )
    });

    if meta.is_dir() {
        let files = fs::read_dir(&from).expect("Could not open fonts directory");
        for file in files.flatten().filter(|file| Path::is_file(&file.path())) {
            let target = to.as_ref().join(file.file_name());
            if let Err(e) = fs::copy(file.path(), &target) {
                error(format!(
                    "Could not copy {} to {}: {e}",
                    file.file_name().display(),
                    target.display(),
                ));
                return;
            };
        }
    } else if let Err(e) = fs::copy(&from, &to) {
        error(format!(
            "Could not copy {} to {}: {e}",
            from.as_ref().display(),
            to.as_ref().display(),
        ));
        return;
    }

    info(format!(
        "Copied {}\n\t {}: \t{}",
        from.as_ref().display(),
        "To".bright_blue(),
        to.as_ref().display(),
    ));
}

fn main() {
    let dot_config: DotConfig = ron::from_str(CONFIG_STRING).expect("Could not parse config file");

    // Make sure the config directory exists
    {
        let config_dir = config_local_dir().expect("Could not get the config directory");

        if let Err(e) = fs::create_dir(config_dir) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                error(format!("Could not create the config directory: {}", e));
                return;
            }
        }
    }

    for symlink in dot_config.symlinks {
        let to = format_location(&symlink);
        if to.is_empty() {
            continue;
        }

        dot_link(symlink.from, to);
    }

    for cpy in dot_config.copies {
        let to = format_location(&cpy);
        if to.is_empty() {
            continue;
        }

        copy(cpy.from, to);
    }

    for dep in dot_config.dependencies {
        match dep {
            Dependency::Cargo { name, git } => {
                info(format!("Installing cargo dep {name}"));
                let mut command = Command::new("cargo");
                command.arg("install");
                if let Some(git) = git {
                    command.arg("--git").arg(git);
                } else {
                    command.arg(&name);
                }
                if let Err(e) = command
                    // Print to the same stdout/stderr as this program
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .status()
                {
                    error(format!("Could not install cargo dep {name}: {e}"));
                }
            }
            Dependency::Bash {
                name,
                command,
                binary,
                directory,
            } => {
                if let Some(binary) = binary {
                    if which::which(&binary).is_ok() {
                        info(format!("Dependency {name} already installed, skipping"));
                        continue;
                    }
                }
                if let Some(directory) = directory {
                    let dir = PathBuf::from(format_path(directory));
                    if dir.exists() {
                        info(format!("Dependency {name} already installed, skipping"));
                        continue;
                    }
                }
                info(format!("Running bash command: `{command}`"));
                if let Err(e) = Command::new("bash")
                    .arg("-c")
                    .arg(format!("{command} | bash"))
                    .stdout(std::process::Stdio::inherit())
                    .status()
                {
                    error(format!("Could not run `{command}`: {e}"));
                }
            }
        }
    }
}
