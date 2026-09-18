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

/// Computes SHA-256 hex digest of a local file
fn compute_file_sha256(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(path).context("Failed to read file for checksum")?;
    let hash = Sha256::digest(&bytes);
    Ok(hex::encode(hash))
}

/// Fetches remote checksums.sha256 from GitHub release
fn fetch_remote_checksums(version: &str) -> Option<String> {
    let url = format!(
        "https://github.com/asl-lang/asl/releases/download/v{}/asl-v{}-checksums.sha256",
        version, version
    );
    let output = Command::new("curl")
        .args(["-fsSL", &url])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

/// Path to local installation receipt
fn receipt_file_path() -> Option<PathBuf> {
    dirs_home().map(|h| h.join(".asl").join("installed_build_sha"))
}

/// Reads recorded build SHA from previous installation
fn read_installed_receipt() -> Option<String> {
    let p = receipt_file_path()?;
    fs::read_to_string(p).ok().map(|s| s.trim().to_string())
}

/// Writes build SHA receipt
fn write_installed_receipt(sha: &str) -> Result<()> {
    if let Some(p) = receipt_file_path() {
        if let Some(parent) = p.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(p, sha.trim())?;
    }
    Ok(())
}

/// Handles `asl update`: downloads and installs the latest binary release
pub fn handle_update(check_only: bool, force: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    let current_exe = std::env::current_exe().context("Failed to locate current executable")?;
    let local_bin_sha = compute_file_sha256(&current_exe).ok();

    println!("🔍 Current version: v{}", current_version);
    println!("🌐 Checking for updates from official release channel...");

    let latest_version = fetch_latest_version()?;
    println!("🚀 Latest release:  v{}", latest_version);

    let target = detect_target()?;
    let tarball_name = format!("asl-v{}-{}.tar.gz", latest_version, target);
    let download_url = format!(
        "https://github.com/asl-lang/asl/releases/download/v{}/{}",
        latest_version, tarball_name
    );

    // Fetch release checksums to evaluate build parity
    let mut remote_bin_sha: Option<String> = None;
    let mut remote_archive_sha: Option<String> = None;

    if let Some(checksum_content) = fetch_remote_checksums(&latest_version) {
        for line in checksum_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let hash = parts[0].to_lowercase();
                let name = parts[1];
                if name.ends_with(&tarball_name) || name == tarball_name {
                    remote_archive_sha = Some(hash);
                } else if name.ends_with(&format!("asl-{}", target)) || name.ends_with(&format!("{}/asl", target)) {
                    remote_bin_sha = Some(hash);
                }
            }
        }
    }

    let installed_receipt = read_installed_receipt();
    let is_update_needed = if force {
        println!("\n⚡ Force re-installation requested.");
        true
    } else if latest_version != current_version {
        println!("\n💡 New release version available: v{} -> v{}", current_version, latest_version);
        true
    } else {
        // Same version (e.g. continuous 0.0.1 beta development)
        if let (Some(ref local), Some(ref remote)) = (&local_bin_sha, &remote_bin_sha) {
            if local.eq_ignore_ascii_case(remote) {
                println!("\n✨ ASL v{} is already up to date! (build {})", latest_version, &remote[..8.min(remote.len())]);
                return Ok(());
            } else {
                println!(
                    "\n💡 A newer build of v{} is available! (local: {}, remote: {})",
                    latest_version,
                    &local[..8.min(local.len())],
                    &remote[..8.min(remote.len())]
                );
                true
            }
        } else if let (Some(ref receipt), Some(ref remote_arch)) = (&installed_receipt, &remote_archive_sha) {
            if receipt.eq_ignore_ascii_case(remote_arch) {
                println!("\n✨ ASL v{} is already up to date! (build {})", latest_version, &remote_arch[..8.min(remote_arch.len())]);
                return Ok(());
            } else {
                println!(
                    "\n💡 A newer build of v{} is available! (installed: {}, remote: {})",
                    latest_version,
                    &receipt[..8.min(receipt.len())],
                    &remote_arch[..8.min(remote_arch.len())]
                );
                true
            }
        } else if remote_archive_sha.is_some() || remote_bin_sha.is_some() {
            println!("\n💡 Updating ASL v{} to match the latest official build...", latest_version);
            true
        } else {
            println!("\n✨ ASL v{} is already up to date!", latest_version);
            return Ok(());
        }
    };

    if check_only {
        if is_update_needed {
            println!("   Run 'asl update' to install the latest build.");
        }
        return Ok(());
    }

    println!("\n📦 Downloading pre-compiled release for {}...", target);
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

    // Verify downloaded archive hash if remote archive hash is available
    if let Some(ref expected_sha) = remote_archive_sha {
        let actual_sha = compute_file_sha256(&archive_path)?;
        if !actual_sha.eq_ignore_ascii_case(expected_sha) {
            let _ = fs::remove_dir_all(&temp_dir);
            anyhow::bail!(
                "Integrity check failed for {}: expected {}, got {}",
                tarball_name, expected_sha, actual_sha
            );
        }
        println!("🔒 Checksum verified: {}", &actual_sha[..8.min(actual_sha.len())]);
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

    // Save build receipt for same-version update comparison
    if let Some(ref sha) = remote_archive_sha {
        let _ = write_installed_receipt(sha);
    } else if let Ok(new_bin_sha) = compute_file_sha256(&current_exe) {
        let _ = write_installed_receipt(&new_bin_sha);
    }

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
