use colored::Colorize;
use serde::Deserialize;
use std::{
    env::current_dir,
    fmt::{self, Debug},
    fs,
    path::{Path, PathBuf},
    process::{Command, exit},
};

#[derive(Debug, Deserialize)]
enum OsOrString {
    Os {
        macos: String,
        linux: String,
        windows: String,
    },
    String(String),
}

impl fmt::Display for OsOrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsOrString::Os {
                macos,
                linux,
                windows,
            } => {
                write!(
                    f,
                    "macOS: {}, Linux: {}, Windows: {}",
                    macos, linux, windows
                )
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

fn yes() -> bool {
    true
}

#[derive(Debug, Deserialize)]
enum Dependency {
    Cargo {
        name: String,
        #[serde(default)]
        git: Option<String>,
        #[serde(default = "yes")]
        windows: bool,
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

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn format_location(loc: &Location) -> String {
    let s = match &loc.to {
        OsOrString::String(s) => s.clone(),
        #[cfg(target_os = "macos")]
        OsOrString::Os { macos, .. } => macos.clone(),
        #[cfg(target_os = "linux")]
        OsOrString::Os { linux, .. } => linux.clone(),
        #[cfg(target_os = "windows")]
        OsOrString::Os { windows, .. } => windows.clone(),
    };
    format_path(s)
}

fn format_path(s: String) -> String {
    let home_dir = dirs::home_dir().expect("Could not find the home directory");
    let config_dir = dirs::config_local_dir().expect("Could not get the config directory");
    let data_dir = dirs::data_dir().expect("Could not get the data directory");
    let data_local_dir = dirs::data_local_dir().expect("Could not get the local data directory");
    let documents_dir = dirs::document_dir().expect("Could not get the document directory");
    s.replace("%UNIX_CONFIG%", &path_to_string(&home_dir.join(".config")))
        .replace("%CONFIG%", &path_to_string(&config_dir))
        .replace("%HOME%", &path_to_string(&home_dir))
        .replace("%DATA%", &path_to_string(&data_dir))
        .replace("%DATA_LOCAL%", &path_to_string(&data_local_dir))
        .replace("%DOCUMENTS%", &path_to_string(&documents_dir))
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

fn error_help<T>(s: T, help: T)
where
    T: AsRef<str>,
{
    println!(
        "{}\t{}\n{}\t{}",
        " ERROR ".bright_red().bold(),
        s.as_ref(),
        " HELP ".bright_blue().bold(),
        help.as_ref()
    )
}

fn resolve_symlink_target(link: &Path) -> Option<PathBuf> {
    let target = fs::read_link(link).ok()?;
    if target.is_absolute() {
        Some(target)
    } else {
        Some(link.parent().unwrap_or(Path::new(".")).join(target))
    }
}

fn paths_equivalent(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

fn symlink_points_to(link: &Path, target: &Path) -> bool {
    resolve_symlink_target(link)
        .is_some_and(|resolved| paths_equivalent(&resolved, target))
}

fn dot_link<T: AsRef<Path>, E: AsRef<Path>>(from: T, to: E) {
    let to = to.as_ref();
    let from = from.as_ref();

    let from_abs = current_dir().unwrap().join(from);

    if !from_abs.exists() && !to.exists() {
        error("Neither source or destination locations exists!");
        error(format!("FROM: {}", from_abs.display()));
        error(format!("TO: {}", to.display()));
        info("---SKIPPING---");
        return;
    }

    if !from_abs.exists() {
        info(format!(
            "Source not setup, copying over {}",
            from_abs.display()
        ));
        if let Err(e) = fs::rename(to, &from_abs) {
            error(format!(
                "Could not move the {} folder to {}",
                to.display(),
                from_abs.display()
            ));
            error(e.to_string());
            return;
        }
    }

    if to.exists() {
        match fs::symlink_metadata(to) {
            Ok(m) => {
                if m.is_symlink() {
                    if symlink_points_to(to, &from_abs) {
                        info(format!(
                            "Skipping {}\n\t {}: \t{}",
                            from_abs.display(),
                            "To".bright_blue(),
                            to.display(),
                        ));
                        return;
                    }
                    if let Err(e) = fs::remove_file(to) {
                        error(format!("Could not delete old symlink {}", to.display()));
                        error(e.to_string());
                        return;
                    }
                } else if m.is_dir() {
                    if let Err(e) = fs::remove_dir_all(to) {
                        error(format!("Could not delete old directory {}", to.display()));
                        error(e.to_string());
                        return;
                    }
                } else if let Err(e) = fs::remove_file(to) {
                    error(format!("Could not delete old file {}", to.display()));
                    error(e.to_string());
                    return;
                }
            }
            Err(e) => {
                error(format!("Unexpected error checking {}: {}", to.display(), e));
                return;
            }
        }
    }

    if let Some(parent) = to.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error(format!(
            "Could not create parent directories for {}",
            to.display()
        ));
        error(e.to_string());
        return;
    }

    #[cfg(unix)]
    let result = std::os::unix::fs::symlink(&from_abs, to);

    #[cfg(windows)]
    let result = {
        let Ok(meta) = fs::symlink_metadata(&from_abs) else {
            error(format!(
                "Could not get metadata of the source file {}",
                from_abs.display()
            ));
            return;
        };
        if meta.is_dir() {
            std::os::windows::fs::symlink_dir(&from_abs, &to)
        } else {
            std::os::windows::fs::symlink_file(&from_abs, &to)
        }
    };

    match result {
        Ok(_) => info(format!(
            "Linked {}\n\t {}: \t{}",
            &from_abs.display(),
            "To".bright_blue(),
            &to.display(),
        )),
        Err(e) => {
            error(format!(
                "could not create symlink from `{}` to `{}`",
                from_abs.display(),
                to.display()
            ));
            error(e.to_string());
        }
    }
}

fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    let Ok(meta) = fs::symlink_metadata(&from) else {
        error(format!(
            "Could not get metadata of copy from {}",
            from.as_ref().display()
        ));
        exit(1);
    };

    if meta.is_dir() {
        match fs::exists(&to) {
            Ok(false) | Err(_) => {
                if let Err(e) = fs::create_dir_all(&to) {
                    error(format!(
                        "Could not create the target directory: {}",
                        to.as_ref().display()
                    ));
                    error(e.to_string());
                    return;
                }
            }
            _ => (),
        }

        let Ok(files) = fs::read_dir(&from) else {
            error(format!(
                "Could not read directory: {}",
                from.as_ref().display()
            ));
            return;
        };
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
    let Ok(config_file) = fs::read_to_string("dotconfig.ron") else {
        error_help(
            "Could not find dotconfig.ron.",
            "Make sure you run dotconfig from the folder where dotconfig.ron is present.",
        );
        exit(1);
    };
    let dot_config: DotConfig = match ron::from_str(&config_file) {
        Ok(config) => config,
        Err(e) => {
            error(format!(
                "Could not parse the config file: {} {}",
                e.span, e.code
            ));
            exit(1);
        }
    };

    for dep in dot_config.dependencies {
        match dep {
            Dependency::Cargo { name, git, windows } => {
                if !windows && cfg!(windows) {
                    info(format!("Skipping cargo dep {name} on windows"));
                    continue;
                }
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
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
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
                if cfg!(windows) {
                    continue;
                }

                let mut exists = false;
                if let Some(binary) = binary
                    && which::which(&binary).is_ok()
                {
                    exists = true;
                }
                if let Some(directory) = directory {
                    let dir = std::path::PathBuf::from(format_path(directory));
                    if dir.exists() {
                        exists = true;
                    }
                }
                if exists {
                    info(format!("Dependency {name} already installed, skipping"));
                    continue;
                }
                info(format!("Running bash command: `{command}`"));
                match Command::new("bash")
                    .arg("-c")
                    .arg(&command)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::piped())
                    .output()
                {
                    Ok(output) => {
                        if output.status.success() {
                            info(format!("Successfully installed {name}"));
                        } else {
                            error(format!(
                                "Command `{command}` failed with error status `{}`",
                                output.status.code().unwrap_or(-1)
                            ));
                            if !output.stderr.is_empty() {
                                let err = String::from_utf8_lossy(&output.stderr);
                                error(err);
                            }
                        }
                    }
                    Err(e) => {
                        error(format!("Could not run `{command}`: {e}"));
                    }
                }
            }
        }
    }

    // Make sure the config directory exists
    #[cfg(unix)]
    {
        let config_dir = dirs::home_dir()
            .expect("Could not get the home directory")
            .join(".config");

        if let Err(e) = fs::create_dir_all(config_dir)
            && e.kind() != std::io::ErrorKind::AlreadyExists
        {
            error(format!("Could not create the config directory: {}", e));
            exit(1);
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

    #[cfg(windows)]
    {
        println!("Press any key to exit the program...");
        use std::io::Read;
        std::io::stdin().read(&mut [0; 1]).unwrap();
    }
}
