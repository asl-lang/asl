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

/// Discovers all default directories to watch (AI tool directories and dev workspaces)
pub fn resolve_watch_dirs(custom_dir: Option<&Path>) -> Vec<PathBuf> {
    if let Some(custom) = custom_dir {
        if custom.exists() {
            return vec![custom.to_path_buf()];
        }
    }

    if let Ok(dir) = std::env::var("ASL_WATCH_DIR") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return vec![p];
        }
    }

    let mut dirs = Vec::new();

    if let Some(home) = dirs_home() {
        // 1. AI agent tool directories (Claude Code, Cursor, Gemini, ASL)
        for ai_sub in &[".claude", ".cursor", ".gemini", ".asl"] {
            let p = home.join(ai_sub);
            if p.exists() {
                dirs.push(p);
            }
        }

        // 2. Dev and project directories
        for candidate in &["Documents/dev", "dev", "projects", "workspace"] {
            let p = home.join(candidate);
            if p.exists() {
                dirs.push(p);
            }
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        if cwd != Path::new("/") && Some(&cwd) != dirs_home().as_ref() {
            dirs.push(cwd);
        }
    }

    if dirs.is_empty() {
        if let Some(home) = dirs_home() {
            dirs.push(home);
        } else {
            dirs.push(PathBuf::from("."));
        }
    }

    dirs.sort();
    dirs.dedup();
    dirs
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

/// Starts the daemon watcher loop in the current process across all watch roots
pub fn handle_daemon_start(custom_dir: Option<&Path>) -> Result<()> {
    let watch_dirs = resolve_watch_dirs(custom_dir);
    let asl_home = get_asl_home_dir()?;
    let pid_file = asl_home.join("daemon.pid");
    let current_pid = std::process::id();
    fs::write(&pid_file, current_pid.to_string())?;

    println!("⚡ ASL Zero-Touch Daemon started [PID: {}]", current_pid);
    for dir in &watch_dirs {
        println!("👀 Monitoring directory: {:?}", dir);
    }
    println!("   Shadow markdown (.md) will be projected automatically on .skill changes.\n");

    let parser = CommonMarkYamlParser::new();
    let sleep_dur = std::time::Duration::from_millis(500);

    use std::io::Write;
    loop {
        for dir in &watch_dirs {
            if dir.exists() {
                if let Err(e) = crate::shadow_cmds::handle_sync_shadows_quiet(dir, &parser) {
                    eprintln!("⚠️ [ASL Daemon Error in {:?}] {}", dir, e);
                }
            }
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
pub fn spawn_detached_daemon(exe: &Path, custom_dir: Option<&Path>) -> Result<u32> {
    let asl_home = get_asl_home_dir()?;
    let log_path = asl_home.join("daemon.log");
    let log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;
    let err_file = log_file.try_clone()?;

    let mut cmd = Command::new(exe);
    cmd.arg("daemon").arg("start");
    if let Some(dir) = custom_dir {
        cmd.arg("--watch-dir").arg(dir);
    }
    let child = cmd
        .stdout(log_file)
        .stderr(err_file)
        .spawn()
        .context("Failed to spawn background ASL daemon process")?;

    let pid = child.id();
    fs::write(asl_home.join("daemon.pid"), pid.to_string())?;

    println!("⚡ ASL Zero-Touch Daemon running in background [PID: {}]", pid);
    if let Some(dir) = custom_dir {
        println!("👀 Monitoring directory: {:?}", dir);
    } else {
        println!("👀 Monitoring all AI tools and dev workspaces");
    }
    println!("   Log output: {:?}", log_path);
    Ok(pid)
}

/// Installs and activates native background OS service (launchd / systemd / Windows task)
pub fn handle_daemon_install(custom_dir: Option<&Path>) -> Result<()> {
    let exe = get_current_exe_path()?;
    let os = std::env::consts::OS;
    println!("📦 Registering ASL Zero-Touch Daemon for OS: {}...", os);
    let _ = handle_daemon_stop();

    match os {
        "macos" => install_macos_launchd(&exe, custom_dir)?,
        "linux" => install_linux_systemd(&exe, custom_dir)?,
        "windows" => install_windows_task(&exe, custom_dir)?,
        _ => println!("⚠️  Native service manager not detected for {}. Use 'asl daemon start --detach'.", os),
    }

    let _ = spawn_detached_daemon(&exe, custom_dir);
    Ok(())
}

/// Uninstalls the native OS service
pub fn handle_daemon_uninstall() -> Result<()> {
    let _ = handle_daemon_stop();
    match std::env::consts::OS {
        "macos" => uninstall_macos_launchd()?,
        "linux" => uninstall_linux_systemd()?,
        "windows" => uninstall_windows_task()?,
        _ => {}
    }
    println!("🗑️  ASL Zero-Touch Daemon service uninstalled.");
    Ok(())
}

/// Interactive guided setup for configuring zero-touch background daemon
pub fn handle_setup() -> Result<()> {
    println!("========================================================");
    println!("💡 Guided Configuration: Automatic Zero-Touch Sync");
    println!("========================================================");
    println!("ASL includes a lightweight background watcher that automatically");
    println!("projects and updates .md files whenever a .skill is created or edited");
    println!("in Claude Code, Cursor, Gemini, or your workspaces.\n");
    print!("Enable automatic background sync? (Recommended) [Y/n]: ");
    use std::io::Write;
    let _ = std::io::stdout().flush();

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let trimmed = input.trim().to_lowercase();

    if trimmed == "n" || trimmed == "no" {
        let _ = handle_daemon_stop();
        let _ = handle_daemon_uninstall();
        println!("\nℹ️  Automatic background daemon disabled (operating in clean, on-demand mode).");
        println!("💡 Useful on-demand commands:");
        println!("    asl sync <path>       # Project .md from .skill files on demand");
        println!("    asl watch <path>      # Run temporary watcher in foreground");
        println!("    asl daemon install    # Activate background service later anytime");
    } else {
        println!("\n⚡ Installing native background service...");
        handle_daemon_install(None)?;
        println!("✅ Automatic background sync is enabled and active across your OS!");
        println!("ℹ️  Manage it anytime with:");
        println!("    asl daemon status");
        println!("    asl daemon stop");
        println!("    asl daemon uninstall");
    }
    Ok(())
}

fn install_macos_launchd(exe: &Path, custom_dir: Option<&Path>) -> Result<()> {
    let home = dirs_home().context("Home dir not found")?;
    let agents_dir = home.join("Library/LaunchAgents");
    fs::create_dir_all(&agents_dir)?;

    let plist_path = agents_dir.join("org.asl-lang.daemon.plist");
    let log_file = get_asl_home_dir()?.join("daemon.log");
    let watch_args = match custom_dir {
        Some(d) => format!("<string>--watch-dir</string>\n        <string>{}</string>", d.to_string_lossy()),
        None => String::new(),
    };

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
        {}
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>StandardOutPath</key>
    <string>{}</string>
    <key>StandardErrorPath</key>
    <string>{}</string>
</dict>
</plist>"#,
        exe.to_string_lossy(),
        watch_args,
        log_file.to_string_lossy(),
        log_file.to_string_lossy()
    );

    fs::write(&plist_path, plist_content)?;
    let _ = Command::new("launchctl").args(["unload", &plist_path.to_string_lossy()]).output();
    let output = Command::new("launchctl")
        .args(["load", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl load")?;

    if output.status.success() {
        println!("🎉 macOS launchd agent registered successfully!\n   Service path: {:?}", plist_path);
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
            let _ = Command::new("launchctl").args(["unload", &plist_path.to_string_lossy()]).output();
            let _ = fs::remove_file(plist_path);
        }
    }
    Ok(())
}

fn install_linux_systemd(exe: &Path, custom_dir: Option<&Path>) -> Result<()> {
    if let Some(home) = dirs_home() {
        let systemd_user = home.join(".config/systemd/user");
        fs::create_dir_all(&systemd_user)?;
        let service_path = systemd_user.join("asl-watcher.service");
        let arg = custom_dir.map_or(String::new(), |d| format!(" --watch-dir {}", d.to_string_lossy()));
        let service_content = format!(
            "[Unit]\nDescription=ASL Zero-Touch Shadow Projection Daemon\nAfter=network.target\n\n[Service]\nType=simple\nExecStart={} daemon start{}\nRestart=on-failure\nRestartSec=3\n\n[Install]\nWantedBy=default.target\n",
            exe.to_string_lossy(), arg
        );
        fs::write(&service_path, service_content)?;
        let _ = Command::new("systemctl").args(["--user", "daemon-reload"]).output();
        if let Ok(out) = Command::new("systemctl").args(["--user", "enable", "--now", "asl-watcher.service"]).output() {
            if out.status.success() {
                println!("🎉 Linux systemd user service registered and started!\n   Service path: {:?}", service_path);
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
            let _ = Command::new("systemctl").args(["--user", "stop", "asl-watcher.service"]).output();
            let _ = Command::new("systemctl").args(["--user", "disable", "asl-watcher.service"]).output();
            let _ = fs::remove_file(service_path);
        }
    }
    Ok(())
}

fn install_windows_task(exe: &Path, custom_dir: Option<&Path>) -> Result<()> {
    let arg = custom_dir.map_or(String::new(), |d| format!(" --watch-dir \\\"{}\\\"", d.to_string_lossy()));
    let task_cmd = format!("\"{}\" daemon start{}", exe.to_string_lossy(), arg);
    let output = Command::new("schtasks").args([
        "/Create", "/TN", "ASLWatcher", "/TR", &task_cmd, "/SC", "ONLOGON", "/RL", "LIMITED", "/F",
    ]).output();
    if let Ok(out) = output {
        if out.status.success() {
            println!("🎉 Windows Scheduled Task 'ASLWatcher' created successfully!");
            let _ = Command::new("schtasks").args(["/Run", "/TN", "ASLWatcher"]).output();
        }
    }
    Ok(())
}

fn uninstall_windows_task() -> Result<()> {
    let _ = Command::new("schtasks").args(["/Delete", "/TN", "ASLWatcher", "/F"]).output();
    Ok(())
}

fn is_pid_running(pid: &str) -> bool {
    let pid = pid.trim();
    if pid.is_empty() || !pid.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    #[cfg(unix)]
    {
        if let Ok(status) = Command::new("kill").args(["-0", pid]).stderr(std::process::Stdio::null()).status() {
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
    let pid = pid.trim();
    if pid.is_empty() || !pid.chars().all(|c| c.is_ascii_digit()) {
        return Ok(());
    }
    #[cfg(unix)]
    {
        let _ = Command::new("kill").args(["-TERM", pid]).stderr(std::process::Stdio::null()).status();
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill").args(["/F", "/PID", pid]).output();
    }
    Ok(())
}
