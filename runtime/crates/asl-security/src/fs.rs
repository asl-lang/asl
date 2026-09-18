use asl_spec::AslError;
use std::path::{Path, PathBuf};

/// Resolves a directory root string into an absolute canonical PathBuf.
/// Expands leading `~` with the user's home directory.
pub fn resolve_and_canonicalize_root(root_str: &str) -> PathBuf {
    let expanded = if let Some(rest) = root_str.strip_prefix("~/") {
        if let Some(home) = dirs_home() {
            home.join(rest)
        } else {
            PathBuf::from(root_str)
        }
    } else if root_str == "~" {
        dirs_home().unwrap_or_else(|| PathBuf::from(root_str))
    } else {
        PathBuf::from(root_str)
    };

    let abs = if expanded.is_relative() {
        std::env::current_dir()
            .map(|c| c.join(&expanded))
            .unwrap_or_else(|_| expanded.clone())
    } else {
        expanded
    };

    std::fs::canonicalize(&abs).unwrap_or(abs)
}

/// Checks that target_path strictly resides within at least one authorized root.
pub fn check_path_confinement(path_str: &str, allowed_roots: &[PathBuf]) -> Result<PathBuf, AslError> {
    if allowed_roots.is_empty() {
        return Err(AslError::CapabilityViolation(format!(
            "Access denied: no confined root authorized for '{}'",
            path_str
        )));
    }

    let expanded = if let Some(rest) = path_str.strip_prefix("~/") {
        if let Some(home) = dirs_home() {
            home.join(rest)
        } else {
            PathBuf::from(path_str)
        }
    } else if path_str == "~" {
        dirs_home().unwrap_or_else(|| PathBuf::from(path_str))
    } else {
        PathBuf::from(path_str)
    };

    let target_path = expanded.as_path();

    let canonical_target = if target_path.exists() {
        std::fs::canonicalize(target_path).map_err(|e| AslError::Io(e.to_string()))?
    } else if let Some(parent) = target_path.parent() {
        let canonical_parent = if parent.as_os_str().is_empty() {
            std::fs::canonicalize(".").map_err(|e| AslError::Io(e.to_string()))?
        } else if parent.exists() {
            std::fs::canonicalize(parent).map_err(|e| AslError::Io(e.to_string()))?
        } else {
            parent.to_path_buf()
        };
        if let Some(file_name) = target_path.file_name() {
            canonical_parent.join(file_name)
        } else {
            canonical_parent
        }
    } else {
        target_path.to_path_buf()
    };

    let is_allowed = allowed_roots
        .iter()
        .any(|root| canonical_target.starts_with(root));

    if !is_allowed {
        return Err(AslError::CapabilityViolation(format!(
            "Confined directory breakout attempt detected for '{}'",
            path_str
        )));
    }

    Ok(canonical_target)
}

pub fn safe_read_file(canonical_target: &Path) -> Result<Option<String>, AslError> {
    if !canonical_target.exists() {
        return Ok(None);
    }
    match std::fs::read_to_string(canonical_target) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(AslError::Io(e.to_string())),
    }
}

pub fn safe_write_file(canonical_target: &Path, content: &str) -> Result<(), AslError> {
    if let Some(parent) = canonical_target.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| AslError::Io(e.to_string()))?;
        }
    }
    std::fs::write(canonical_target, content).map_err(|e| AslError::Io(e.to_string()))
}

pub fn safe_list_dir(canonical_target: &Path) -> Result<Vec<String>, AslError> {
    if !canonical_target.exists() {
        return Err(AslError::Io(format!("Directory not found: {:?}", canonical_target)));
    }
    let entries = std::fs::read_dir(canonical_target).map_err(|e| AslError::Io(e.to_string()))?;
    let mut names = Vec::new();
    for entry in entries.flatten() {
        if let Ok(name) = entry.file_name().into_string() {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}
