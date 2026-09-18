use anyhow::{Context, Result};
use asl_parser::CommonMarkYamlParser;
use clap::Subcommand;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Subcommand, Debug, Clone)]
pub enum DaemonAction {
    /// Starts the daemon watcher loop
    Start {
        /// Directory to monitor (defaults to detected workspace or dev dir)
        #[arg(long)]
        watch_dir: Option<PathBuf>,

        /// Run detached in the background
        #[arg(short, long)]
        detach: bool,
    },
    /// Installs and activates native background OS service (launchd / systemd / Windows task)
    Install {
        /// Directory to monitor (defaults to detected workspace or dev dir)
        #[arg(long)]
        watch_dir: Option<PathBuf>,
    },
    /// Stops the running daemon
    Stop,
    /// Checks status of the background daemon
    Status,
    /// Uninstalls native background OS service
    Uninstall,
}

/// Default watch directory discovery
pub fn resolve_default_watch_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("ASL_WATCH_DIR") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return p;
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        if let Some(home) = dirs_home() {
            if cwd != home && cwd.exists() {
                return cwd;
            }
        }
    }

    if let Some(home) = dirs_home() {
        for candidate in &["Documents/dev", "dev", "projects", "workspace"] {
            let p = home.join(candidate);
            if p.exists() {
                return p;
            }
        }
        return home;
    }

    PathBuf::from(".")
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn get_asl_home_dir() -> Result<PathBuf> {
    let home = dirs_home().context("Could not determine user home directory")?;
    let asl_home = home.join(".asl");
    fs::create_dir_all(&asl_home)?;
    Ok(asl_home)
}

fn get_pid_file_path() -> Result<PathBuf> {
    Ok(get_asl_home_dir()?.join("daemon.pid"))
}

fn get_current_exe_path() -> Result<PathBuf> {
    std::env::current_exe().context("Failed to get current executable path")
}

/// Starts the daemon watcher loop in the current process
pub fn handle_daemon_start(watch_dir: &Path) -> Result<()> {
    let asl_home = get_asl_home_dir()?;
    let pid_file = asl_home.join("daemon.pid");
    let current_pid = std::process::id();
    fs::write(&pid_file, current_pid.to_string())?;

    println!("⚡ ASL Zero-Touch Daemon started [PID: {}]", current_pid);
    println!("👀 Monitoring directory: {:?}", watch_dir);
    println!("   Shadow markdown (.md) will be projected automatically on .skill changes.\n");

    let parser = CommonMarkYamlParser::new();
    let sleep_dur = std::time::Duration::from_millis(500);

    use std::io::Write;
    loop {
        if let Err(e) = crate::shadow_cmds::handle_sync_shadows_quiet(watch_dir, &parser) {
            eprintln!("⚠️ [ASL Daemon Error] {}", e);
        }
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
        std::thread::sleep(sleep_dur);
    }
}

/// Checks daemon running status
pub fn handle_daemon_status() -> Result<()> {
    let pid_file = get_pid_file_path()?;
    if pid_file.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file) {
            let pid = pid_str.trim();
            if is_pid_running(pid) {
                println!("✅ ASL Zero-Touch Daemon is RUNNING (PID: {})", pid);
                return Ok(());
            }
        }
    }
    println!("⭕ ASL Zero-Touch Daemon is STOPPED (not running)");
    Ok(())
}

/// Stops the running daemon
pub fn handle_daemon_stop() -> Result<()> {
    let pid_file = get_pid_file_path()?;
    if pid_file.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file) {
            let pid = pid_str.trim();
            if is_pid_running(pid) {
                kill_pid(pid)?;
                println!("🛑 ASL Zero-Touch Daemon stopped (PID: {})", pid);
            }
        }
        let _ = fs::remove_file(&pid_file);
    } else {
        println!("ℹ️  No active ASL daemon found.");
    }
    Ok(())
}

/// Spawns the daemon in a detached background process
pub fn spawn_detached_daemon(exe: &Path, watch_dir: &Path) -> Result<u32> {
    let asl_home = get_asl_home_dir()?;
    let log_path = asl_home.join("daemon.log");
    let log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;
    let err_file = log_file.try_clone()?;

    let child = Command::new(exe)
        .arg("daemon")
        .arg("start")
        .arg("--watch-dir")
        .arg(watch_dir)
        .stdout(log_file)
        .stderr(err_file)
        .spawn()
        .context("Failed to spawn background ASL daemon process")?;

    let pid = child.id();
    let pid_file = asl_home.join("daemon.pid");
    fs::write(&pid_file, pid.to_string())?;

    println!("⚡ ASL Zero-Touch Daemon running in background [PID: {}]", pid);
    println!("👀 Monitoring directory: {:?}", watch_dir);
    println!("   Log output: {:?}", log_path);

    Ok(pid)
}

/// Installs and activates the native OS background service (launchd / systemd / Windows task)
pub fn handle_daemon_install(watch_dir: &Path) -> Result<()> {
    let exe = get_current_exe_path()?;
    let os = std::env::consts::OS;

    println!("📦 Registering ASL Zero-Touch Daemon for OS: {}...", os);
    let _ = handle_daemon_stop();

    match os {
        "macos" => install_macos_launchd(&exe, watch_dir)?,
        "linux" => install_linux_systemd(&exe, watch_dir)?,
        "windows" => install_windows_task(&exe, watch_dir)?,
        _ => {
            println!("⚠️  Native service manager not detected for {}. Use 'asl daemon start --detach'.", os);
        }
    }

    // Always spawn background process for instantaneous active zero-touch ingestion
    let _ = spawn_detached_daemon(&exe, watch_dir);

    Ok(())
}

/// Uninstalls the native OS service
pub fn handle_daemon_uninstall() -> Result<()> {
    let os = std::env::consts::OS;
    let _ = handle_daemon_stop();

    match os {
        "macos" => uninstall_macos_launchd()?,
        "linux" => uninstall_linux_systemd()?,
        "windows" => uninstall_windows_task()?,
        _ => {}
    }

    println!("🗑️  ASL Zero-Touch Daemon service uninstalled.");
    Ok(())
}

fn install_macos_launchd(exe: &Path, watch_dir: &Path) -> Result<()> {
    let home = dirs_home().context("Home dir not found")?;
    let agents_dir = home.join("Library/LaunchAgents");
    fs::create_dir_all(&agents_dir)?;

    let plist_path = agents_dir.join("org.asl-lang.daemon.plist");
    let log_file = get_asl_home_dir()?.join("daemon.log");

    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.asl-lang.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>daemon</string>
        <string>start</string>
        <string>--watch-dir</string>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>LowPriorityIO</key>
    <false/>
    <key>StandardOutPath</key>
    <string>{}</string>
    <key>StandardErrorPath</key>
    <string>{}</string>
</dict>
</plist>
"#,
        exe.to_string_lossy(),
        watch_dir.to_string_lossy(),
        log_file.to_string_lossy(),
        log_file.to_string_lossy()
    );

    fs::write(&plist_path, plist_content)?;

    // Unload previous instance if any, then load new
    let _ = Command::new("launchctl")
        .args(["unload", &plist_path.to_string_lossy()])
        .output();

    let output = Command::new("launchctl")
        .args(["load", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl load")?;

    if output.status.success() {
        println!("🎉 macOS launchd agent registered successfully!");
        println!("   Service path: {:?}", plist_path);
        println!("   Zero-Touch shadow projection is now ACTIVE in background across macOS.");
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("⚠️  launchctl warning: {}", err.trim());
    }

    Ok(())
}

fn uninstall_macos_launchd() -> Result<()> {
    if let Some(home) = dirs_home() {
        let plist_path = home.join("Library/LaunchAgents/org.asl-lang.daemon.plist");
        if plist_path.exists() {
            let _ = Command::new("launchctl")
                .args(["unload", &plist_path.to_string_lossy()])
                .output();
            let _ = fs::remove_file(plist_path);
        }
    }
    Ok(())
}

fn install_linux_systemd(exe: &Path, watch_dir: &Path) -> Result<()> {
    if let Some(home) = dirs_home() {
        let systemd_user = home.join(".config/systemd/user");
        fs::create_dir_all(&systemd_user)?;

        let service_path = systemd_user.join("asl-watcher.service");
        let service_content = format!(
            r#"[Unit]
Description=ASL Zero-Touch Shadow Projection Daemon
After=network.target

[Service]
Type=simple
ExecStart={} daemon start --watch-dir {}
Restart=on-failure
RestartSec=3

[Install]
WantedBy=default.target
"#,
            exe.to_string_lossy(),
            watch_dir.to_string_lossy()
        );

        fs::write(&service_path, service_content)?;

        let _ = Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .output();

        let output = Command::new("systemctl")
            .args(["--user", "enable", "--now", "asl-watcher.service"])
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                println!("🎉 Linux systemd user service registered and started!");
                println!("   Service path: {:?}", service_path);
                return Ok(());
            }
        }
        println!("ℹ️  systemd not running for user. Service file created at {:?}", service_path);
    }
    Ok(())
}

fn uninstall_linux_systemd() -> Result<()> {
    if let Some(home) = dirs_home() {
        let service_path = home.join(".config/systemd/user/asl-watcher.service");
        if service_path.exists() {
            let _ = Command::new("systemctl")
                .args(["--user", "stop", "asl-watcher.service"])
                .output();
            let _ = Command::new("systemctl")
                .args(["--user", "disable", "asl-watcher.service"])
                .output();
            let _ = fs::remove_file(service_path);
        }
    }
    Ok(())
}

fn install_windows_task(exe: &Path, watch_dir: &Path) -> Result<()> {
    let task_cmd = format!(
        "\"{}\" daemon start --watch-dir \"{}\"",
        exe.to_string_lossy(),
        watch_dir.to_string_lossy()
    );

    let output = Command::new("schtasks")
        .args([
            "/Create",
            "/TN",
            "ASLWatcher",
            "/TR",
            &task_cmd,
            "/SC",
            "ONLOGON",
            "/RL",
            "LIMITED",
            "/F",
        ])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            println!("🎉 Windows Scheduled Task 'ASLWatcher' created successfully!");
            let _ = Command::new("schtasks").args(["/Run", "/TN", "ASLWatcher"]).output();
        }
    }
    Ok(())
}

fn uninstall_windows_task() -> Result<()> {
    let _ = Command::new("schtasks")
        .args(["/Delete", "/TN", "ASLWatcher", "/F"])
        .output();
    Ok(())
}

fn is_pid_running(pid: &str) -> bool {
    #[cfg(unix)]
    {
        if let Ok(status) = Command::new("kill").args(["-0", pid]).status() {
            return status.success();
        }
    }
    #[cfg(windows)]
    {
        if let Ok(out) = Command::new("tasklist").args(["/FI", &format!("PID eq {}", pid)]).output() {
            return String::from_utf8_lossy(&out.stdout).contains(pid);
        }
    }
    false
}

fn kill_pid(pid: &str) -> Result<()> {
    #[cfg(unix)]
    {
        let _ = Command::new("kill").args(["-TERM", pid]).status();
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill").args(["/F", "/PID", pid]).output();
    }
    Ok(())
}
