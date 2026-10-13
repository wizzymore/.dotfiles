use clap::Parser;
use colored::Colorize;
use serde::Deserialize;
use std::{
    fmt::{self, Debug},
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
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

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Install configured dotfiles and dependencies",
    after_help = "Existing configurations are backed up beside their destinations."
)]
struct Args {
    /// Load configuration from this RON file.
    #[arg(long, value_name = "PATH", default_value = "dotconfig.ron")]
    config: PathBuf,

    /// Preview actions without installing dependencies or changing files.
    #[arg(long)]
    dry_run: bool,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn format_location(loc: &Location) -> Result<String> {
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

fn format_path(mut s: String) -> Result<String> {
    type DirectoryResolver = fn() -> Option<PathBuf>;
    let tokens: [(&str, DirectoryResolver); 6] = [
        ("%HOME%", dirs::home_dir),
        ("%UNIX_CONFIG%", || {
            dirs::home_dir().map(|p| p.join(".config"))
        }),
        ("%CONFIG%", dirs::config_local_dir),
        ("%DATA%", dirs::data_dir),
        ("%DATA_LOCAL%", dirs::data_local_dir),
        ("%DOCUMENTS%", dirs::document_dir),
    ];
    for (token, resolve) in tokens {
        if s.contains(token) {
            let path = resolve().ok_or_else(|| format!("Could not resolve {token}"))?;
            s = s.replace(token, &path.to_string_lossy());
        }
    }
    if s.contains('%') {
        return Err(format!("Unresolved placeholder in {s}").into());
    }
    Ok(s)
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
    eprintln!("{}\t{}", " ERROR ".bright_red().bold(), s.as_ref())
}

fn metadata(path: &Path) -> Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(meta) => Ok(Some(meta)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("Cannot inspect {}: {e}", path.display()).into()),
    }
}

fn backup_path(path: &Path) -> Result<PathBuf> {
    let name = path.file_name().ok_or("Cannot back up a filesystem root")?;
    for index in 1.. {
        let mut backup_name = name.to_os_string();
        backup_name.push(format!(".dotconfig-backup.{index}"));
        let candidate = path.with_file_name(backup_name);
        if metadata(&candidate)?.is_none() {
            return Ok(candidate);
        }
    }
    unreachable!()
}

fn backup(path: &Path, dry_run: bool) -> Result<Option<PathBuf>> {
    if metadata(path)?.is_none() {
        return Ok(None);
    }
    let target = backup_path(path)?;
    info(format!(
        "{} {} -> {}",
        if dry_run {
            "Would back up"
        } else {
            "Backing up"
        },
        path.display(),
        target.display()
    ));
    if !dry_run {
        fs::rename(path, &target)?;
    }
    Ok(Some(target))
}

fn create_symlink(from: &Path, to: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(from, to)
    }
    #[cfg(windows)]
    {
        if fs::metadata(from)?.is_dir() {
            std::os::windows::fs::symlink_dir(from, to)
        } else {
            std::os::windows::fs::symlink_file(from, to)
        }
    }
}

fn dot_link(from: &Path, to: &Path, dry_run: bool) -> Result<()> {
    fs::metadata(from).map_err(|e| format!("Cannot read source {}: {e}", from.display()))?;
    if let Some(meta) = metadata(to)?
        && meta.is_symlink()
    {
        let target = fs::read_link(to)?;
        let resolved = if target.is_absolute() {
            target
        } else {
            to.parent().unwrap().join(target)
        };
        if fs::canonicalize(resolved).ok() == Some(fs::canonicalize(from)?) {
            info(format!("Already linked {}", to.display()));
            return Ok(());
        }
    }
    let saved = backup(to, dry_run)?;
    info(format!(
        "{} {} -> {}",
        if dry_run { "Would link" } else { "Linking" },
        to.display(),
        from.display()
    ));
    if dry_run {
        return Ok(());
    }
    let result = (|| -> std::io::Result<()> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        create_symlink(from, to)
    })();
    if let Err(e) = result {
        if let Some(saved) = saved {
            fs::rename(saved, to)?;
        }
        return Err(format!("Cannot link {}: {e}", to.display()).into());
    }
    Ok(())
}

fn copy(from: &Path, to: &Path, dry_run: bool) -> Result<()> {
    let source = fs::symlink_metadata(from)?;
    if source.is_symlink() {
        return Err(format!(
            "Copy source {} is a symlink; use a symlink entry instead",
            from.display()
        )
        .into());
    }
    if source.is_dir() {
        let target = metadata(to)?;
        if target
            .as_ref()
            .is_some_and(|m| !m.is_dir() || m.is_symlink())
        {
            backup(to, dry_run)?;
        }
        if !dry_run {
            fs::create_dir_all(to)?;
        }
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            copy(&entry.path(), &to.join(entry.file_name()), dry_run)?;
        }
    } else if source.is_file() {
        // Avoid backups and writes on repeated runs when contents are unchanged.
        if metadata(to)?.is_some_and(|m| m.is_file() && !m.is_symlink())
            && fs::read(from)? == fs::read(to)?
        {
            return Ok(());
        }
        let saved = backup(to, dry_run)?;
        info(format!(
            "{} {} -> {}",
            if dry_run { "Would copy" } else { "Copying" },
            from.display(),
            to.display()
        ));
        if dry_run {
            return Ok(());
        }
        let result = (|| -> std::io::Result<()> {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(from, to)?;
            Ok(())
        })();
        if let Err(e) = result {
            if let Some(saved) = saved {
                if metadata(to)?.is_some() {
                    fs::remove_file(to)?;
                }
                fs::rename(saved, to)?;
            }
            return Err(format!("Cannot copy to {}: {e}", to.display()).into());
        }
    } else {
        return Err(format!("Unsupported copy source {}", from.display()).into());
    }
    Ok(())
}

fn run_command(command: &mut Command, dry_run: bool) -> Result<()> {
    info(format!(
        "{} {command:?}",
        if dry_run { "Would run" } else { "Running" }
    ));
    if dry_run {
        return Ok(());
    }
    let status = command
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        return Err(format!("{command:?} failed with {status}").into());
    }
    Ok(())
}

fn install_dependency(dep: Dependency, dry_run: bool) -> Result<()> {
    match dep {
        Dependency::Cargo { name, git, windows } => {
            if !windows && cfg!(windows) {
                return Ok(());
            }
            let mut command = Command::new("cargo");
            command.arg("install");
            if let Some(git) = git {
                command.arg("--git").arg(git);
            }
            command.arg(name);
            run_command(&mut command, dry_run)
        }
        Dependency::Bash {
            name,
            command,
            binary,
            directory,
        } => {
            if cfg!(windows) {
                return Ok(());
            }
            let binary_exists = binary.is_some_and(|b| which::which(b).is_ok());
            let directory_exists = match directory {
                Some(d) => Path::new(&format_path(d)?).try_exists()?,
                None => false,
            };
            if binary_exists || directory_exists {
                info(format!("Dependency {name} already installed, skipping"));
                return Ok(());
            }
            // Propagate download failures from pipelines such as curl | bash.
            run_command(
                Command::new("bash")
                    .arg("-o")
                    .arg("pipefail")
                    .arg("-c")
                    .arg(command),
                dry_run,
            )
        }
    }
}

fn run(args: Args) -> Result<()> {
    let config_path = fs::canonicalize(args.config)?;
    let root = config_path
        .parent()
        .ok_or("Config has no parent directory")?;
    let config: DotConfig = ron::from_str(&fs::read_to_string(&config_path)?)?;
    // Resolve and check all configured sources before running installers or writing files.
    let mut operations = Vec::new();
    for (locations, link) in [(config.symlinks, true), (config.copies, false)] {
        for location in locations {
            let destination = format_location(&location)?;
            if destination.is_empty() {
                continue;
            }
            let from = fs::canonicalize(root.join(location.from))?;
            let to = PathBuf::from(destination);
            if !to.is_absolute()
                || to.file_name().is_none()
                || to
                    .components()
                    .any(|c| c == std::path::Component::ParentDir)
                || from.starts_with(&to)
                || to.starts_with(&from)
            {
                return Err(format!("Unsafe destination: {}", to.display()).into());
            }
            if operations.iter().any(|(_, existing, _)| existing == &to) {
                return Err(format!("Duplicate destination: {}", to.display()).into());
            }
            operations.push((from, to, link));
        }
    }
    let mut failures = 0;
    for dep in config.dependencies {
        if let Err(e) = install_dependency(dep, args.dry_run) {
            error(e.to_string());
            failures += 1;
        }
    }
    for (from, to, link) in operations {
        let result = if link {
            dot_link(&from, &to, args.dry_run)
        } else {
            copy(&from, &to, args.dry_run)
        };
        if let Err(e) = result {
            error(format!("{}: {e}", to.display()));
            failures += 1;
        }
    }
    if failures != 0 {
        return Err(format!("Setup incomplete: {failures} operation(s) failed").into());
    }
    info(if args.dry_run {
        "Dry run complete; no changes made"
    } else {
        "Setup complete"
    });
    Ok(())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error(e.to_string());
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Sandbox(PathBuf);
    impl Sandbox {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = env::temp_dir().join(format!(
                "dotconfig-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }
    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn copy_preserves_conflicts_and_is_repeatable() {
        let s = Sandbox::new();
        let source = s.path("source");
        let target = s.path("target");
        fs::create_dir_all(source.join("nested")).unwrap();
        fs::write(source.join("nested/file"), "new").unwrap();
        fs::create_dir_all(target.join("nested")).unwrap();
        fs::write(target.join("nested/file"), "old").unwrap();
        fs::write(target.join("unrelated"), "keep").unwrap();
        copy(&source, &target, false).unwrap();
        assert_eq!(
            fs::read_to_string(target.join("nested/file.dotconfig-backup.1")).unwrap(),
            "old"
        );
        assert_eq!(
            fs::read_to_string(target.join("nested/file")).unwrap(),
            "new"
        );
        assert_eq!(
            fs::read_to_string(target.join("unrelated")).unwrap(),
            "keep"
        );
        copy(&source, &target, false).unwrap();
        assert!(!target.join("nested/file.dotconfig-backup.2").exists());
    }

    #[test]
    fn dry_run_does_not_write_or_execute_commands() {
        let s = Sandbox::new();
        let source = s.path("source");
        let target = s.path("target");
        fs::write(&source, "new").unwrap();
        fs::write(&target, "old").unwrap();
        copy(&source, &target, true).unwrap();
        dot_link(&source, &target, true).unwrap();
        copy(&source, &s.path("missing/target"), true).unwrap();
        run_command(&mut Command::new("dotconfig-nonexistent-command"), true).unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "old");
        assert!(!s.path("missing").exists());
        assert!(!s.path("target.dotconfig-backup.1").exists());
    }

    #[cfg(unix)]
    #[test]
    fn linking_backs_up_directories_and_broken_links() {
        let s = Sandbox::new();
        let source = s.path("source");
        let target = s.path("target");
        fs::write(&source, "new").unwrap();
        fs::create_dir(&target).unwrap();
        fs::write(target.join("old"), "keep").unwrap();
        dot_link(&source, &target, false).unwrap();
        assert_eq!(
            fs::read_to_string(s.path("target.dotconfig-backup.1/old")).unwrap(),
            "keep"
        );
        dot_link(&source, &target, false).unwrap();
        assert!(!s.path("target.dotconfig-backup.2").exists());
        let broken = s.path("broken");
        create_symlink(&s.path("missing"), &broken).unwrap();
        dot_link(&source, &broken, false).unwrap();
        assert_eq!(
            fs::read_link(s.path("broken.dotconfig-backup.1")).unwrap(),
            s.path("missing")
        );
        assert_eq!(fs::read_link(&broken).unwrap(), source);
    }

    #[cfg(unix)]
    #[test]
    fn child_command_failure_is_an_error() {
        assert!(run_command(Command::new("sh").arg("-c").arg("exit 7"), false).is_err());
        assert!(
            run_command(
                Command::new("bash")
                    .arg("-o")
                    .arg("pipefail")
                    .arg("-c")
                    .arg("false | true"),
                false
            )
            .is_err()
        );
    }

    #[test]
    fn missing_source_does_not_move_destination() {
        let s = Sandbox::new();
        let target = s.path("target");
        fs::write(&target, "keep").unwrap();
        assert!(dot_link(&s.path("missing"), &target, false).is_err());
        assert_eq!(fs::read_to_string(target).unwrap(), "keep");
        assert!(!s.path("target.dotconfig-backup.1").exists());
    }

    #[test]
    fn unknown_placeholders_are_errors() {
        assert!(format_path("%UNKNOWN%/file".into()).is_err());
        assert_eq!(format_path(String::new()).unwrap(), "");
    }

    #[test]
    fn cli_parses_config_and_dry_run_options() {
        let defaults = Args::try_parse_from(["dotconfig"]).unwrap();
        assert_eq!(defaults.config, PathBuf::from("dotconfig.ron"));
        assert!(!defaults.dry_run);

        let args =
            Args::try_parse_from(["dotconfig", "--config", "custom.ron", "--dry-run"]).unwrap();
        assert_eq!(args.config, PathBuf::from("custom.ron"));
        assert!(args.dry_run);
        assert!(Args::try_parse_from(["dotconfig", "--unknown"]).is_err());
    }

    #[test]
    fn config_sources_resolve_relative_to_config_file() {
        let sandbox = Sandbox::new();
        let config_dir = sandbox.path("profile");
        fs::create_dir(&config_dir).unwrap();
        let source = config_dir.join("source");
        let destination = sandbox.path("destination");
        fs::write(&source, "configured").unwrap();
        let config = config_dir.join("dotconfig.ron");
        fs::write(
            &config,
            format!(
                "DotConfig(dependencies: [], symlinks: [Location(from: \"source\", to: String({:?}))], copies: [])",
                destination.to_string_lossy()
            ),
        )
        .unwrap();

        run(Args {
            config,
            dry_run: false,
        })
        .unwrap();

        assert_eq!(fs::read_to_string(destination).unwrap(), "configured");
    }

    #[test]
    fn invalid_config_is_rejected_before_dependency_commands_run() {
        let sandbox = Sandbox::new();
        let source = sandbox.path("source");
        let config = sandbox.path("dotconfig.ron");
        let marker = sandbox.path("dependency-ran");
        let destination = sandbox.path("destination");
        fs::write(&source, "content").unwrap();
        fs::write(
            &config,
            format!(
                "DotConfig(dependencies: [Bash(name: \"marker\", command: \"touch {}\", binary: None, directory: None)], symlinks: [Location(from: \"source\", to: String({:?})), Location(from: \"source\", to: String({:?}))], copies: [])",
                marker.display(),
                destination.to_string_lossy(),
                destination.to_string_lossy()
            ),
        )
        .unwrap();

        let error = run(Args {
            config,
            dry_run: false,
        })
        .unwrap_err();

        assert!(error.to_string().contains("Duplicate destination"));
        assert!(!marker.exists());
    }

    #[cfg(unix)]
    #[test]
    fn failed_dependency_command_makes_setup_fail() {
        let sandbox = Sandbox::new();
        let config = sandbox.path("dotconfig.ron");
        fs::write(
            &config,
            "DotConfig(dependencies: [Bash(name: \"failure\", command: \"exit 9\", binary: None, directory: None)], symlinks: [], copies: [])",
        )
        .unwrap();

        let error = run(Args {
            config,
            dry_run: false,
        })
        .unwrap_err();
        assert!(error.to_string().contains("Setup incomplete"));
    }
}
