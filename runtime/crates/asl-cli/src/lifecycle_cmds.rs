use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Detects the host target triple for pre-compiled binary releases
fn detect_target() -> Result<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
        ("macos", "x86_64") => Ok("x86_64-apple-darwin"),
        ("linux", "x86_64") => Ok("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Ok("aarch64-unknown-linux-gnu"),
        ("windows", "x86_64") => Ok("x86_64-pc-windows-msvc"),
        _ => anyhow::bail!("Unsupported platform for auto-update: {}-{}", os, arch),
    }
}

/// Resolves the latest version tag from GitHub Releases without rate limiting
fn fetch_latest_version() -> Result<String> {
    let output = Command::new("curl")
        .args([
            "-sIL",
            "-o",
            "/dev/null",
            "-w",
            "%{url_effective}",
            "https://github.com/asl-lang/asl/releases/latest",
        ])
        .output()
        .context("Failed to check latest ASL version via curl")?;

    let effective_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if let Some(tag) = effective_url.rsplit('/').next() {
        let ver = tag.strip_prefix('v').unwrap_or(tag);
        if !ver.is_empty() && ver.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return Ok(ver.to_string());
        }
    }
    anyhow::bail!("Could not parse latest version from: {}", effective_url)
}

/// Handles `asl update`: downloads and installs the latest binary release
pub fn handle_update(check_only: bool, force: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("🔍 Current version: v{}", current_version);
    println!("🌐 Checking for updates from official release channel...");

    let latest_version = fetch_latest_version()?;
    println!("🚀 Latest version:  v{}", latest_version);

    if latest_version == current_version && !force {
        println!("\n✨ ASL is already up to date!");
        return Ok(());
    }

    if check_only {
        println!("\n💡 An update is available! Run 'asl update' to install v{}.", latest_version);
        return Ok(());
    }

    let target = detect_target()?;
    let tarball_name = format!("asl-v{}-{}.tar.gz", latest_version, target);
    let download_url = format!(
        "https://github.com/asl-lang/asl/releases/download/v{}/{}",
        latest_version, tarball_name
    );

    println!("\n📦 Downloading pre-compiled release for {}...", target);
    let current_exe = std::env::current_exe().context("Failed to locate current executable")?;
    let parent_dir = current_exe
        .parent()
        .context("Failed to determine binary parent directory")?;

    let temp_dir = std::env::temp_dir().join(format!("asl_update_{}", std::process::id()));
    fs::create_dir_all(&temp_dir)?;
    let archive_path = temp_dir.join(&tarball_name);

    let curl_status = Command::new("curl")
        .args(["-fsSL", &download_url, "-o", &archive_path.to_string_lossy()])
        .status()
        .context("Failed to execute curl download")?;

    if !curl_status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        anyhow::bail!("Download failed for URL: {}", download_url);
    }

    println!("✂️  Extracting binary...");
    let tar_status = Command::new("tar")
        .args([
            "-xzf",
            &archive_path.to_string_lossy(),
            "-C",
            &temp_dir.to_string_lossy(),
        ])
        .status()
        .context("Failed to extract update tarball")?;

    if !tar_status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        anyhow::bail!("Failed to extract archive: {}", tarball_name);
    }

    let candidate_dir = temp_dir.join(format!("asl-v{}-{}", latest_version, target));
    let new_bin = if candidate_dir.join("asl").exists() {
        candidate_dir.join("asl")
    } else if temp_dir.join("asl").exists() {
        temp_dir.join("asl")
    } else if candidate_dir.join("asl.exe").exists() {
        candidate_dir.join("asl.exe")
    } else {
        temp_dir.join("asl.exe")
    };

    if !new_bin.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
        anyhow::bail!("Binary file 'asl' not found in downloaded release");
    }

    // Replace running executable in-place
    install_binary(&new_bin, &current_exe, parent_dir)?;
    let _ = fs::remove_dir_all(&temp_dir);

    println!("\n🎉 Successfully updated ASL to v{}!", latest_version);
    println!("   Installed path: {:?}", current_exe);
    Ok(())
}

fn install_binary(new_bin: &Path, current_exe: &Path, parent_dir: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(new_bin, fs::Permissions::from_mode(0o755));
        let temp_dest = parent_dir.join(format!(".asl_new_{}", std::process::id()));
        fs::copy(new_bin, &temp_dest)?;
        let _ = fs::set_permissions(&temp_dest, fs::Permissions::from_mode(0o755));
        fs::rename(&temp_dest, current_exe)
            .context("Failed to atomically replace executable binary")?;
    }
    #[cfg(windows)]
    {
        let backup = parent_dir.join(format!("asl_old_{}.exe", std::process::id()));
        let _ = fs::rename(current_exe, &backup);
        fs::copy(new_bin, current_exe)
            .context("Failed to replace binary on Windows")?;
    }
    Ok(())
}

/// Handles `asl uninstall`: cleanly stops daemons, removes services, and deletes the binary
pub fn handle_uninstall(yes: bool, purge: bool) -> Result<()> {
    let current_exe = std::env::current_exe().context("Failed to locate current executable")?;

    println!("========================================================");
    println!("⚠️  ASL Self-Uninstall");
    println!("========================================================");
    println!("This will uninstall Agent Skill Language from your system:");
    println!("  • Stop and remove background services (launchd / systemd / tasks)");
    println!("  • Delete the binary at: {:?}", current_exe);
    if purge {
        println!("  • Purge user configuration and daemon logs (~/.asl)");
    }
    println!();

    if !yes {
        print!("Are you sure you want to proceed? [y/N]: ");
        use std::io::Write;
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let trimmed = input.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("🛑 Uninstallation cancelled. No changes were made.");
            return Ok(());
        }
    }

    println!("\n🛑 Stopping background daemon...");
    let _ = crate::daemon_cmds::handle_daemon_stop();
    println!("🗑️  Deregistering system background services...");
    let _ = crate::daemon_cmds::handle_daemon_uninstall();

    if purge {
        if let Some(home) = dirs_home() {
            let asl_home = home.join(".asl");
            if asl_home.exists() {
                let _ = fs::remove_dir_all(&asl_home);
                println!("🧹 Purged runtime directory {:?}", asl_home);
            }
        }
    }

    println!("🗑️  Deleting executable binary: {:?}", current_exe);
    #[cfg(unix)]
    {
        fs::remove_file(&current_exe).context("Failed to remove ASL binary file")?;
    }
    #[cfg(windows)]
    {
        let exe_str = current_exe.to_string_lossy().to_string();
        let _ = Command::new("cmd")
            .args(["/C", "ping 127.0.0.1 -n 2 > NUL & del /F /Q", &exe_str])
            .spawn();
    }

    println!("\n🎉 ASL has been completely uninstalled from this system.");
    println!("ℹ️  Thank you for using Agent Skill Language (ASL)!");
    Ok(())
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}
