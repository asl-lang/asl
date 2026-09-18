use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::{
    clean_orphaned_shadows, is_ignored_path, project_shadow_markdown, CommonMarkYamlParser,
    ShadowProjectResult,
};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SyncStats {
    pub created: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub collisions: usize,
    pub orphaned_removed: usize,
}

/// Synchronizes Markdown shadow projections for a .skill file or full directory
pub fn handle_sync_shadows(target_path: &Path, parser: &CommonMarkYamlParser) -> Result<SyncStats> {
    sync_internal(target_path, parser, true)
}

pub fn handle_sync_shadows_quiet(target_path: &Path, parser: &CommonMarkYamlParser) -> Result<SyncStats> {
    sync_internal(target_path, parser, false)
}

fn scaffold_empty_asl_file(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    let stem = path.file_stem()?.to_str().unwrap_or("draft");
    match ext.as_str() {
        "skill" => Some(format!(
            "#!/usr/bin/env -S asl run\n---\nasl_version: \"3.0\"\nname: \"{}\"\ndescription: \"Draft skill '{}' under construction\"\n---\n\n# {}\n\nDraft instructions here.\n",
            stem, stem, stem
        )),
        "tool" => Some(format!(
            "#!/usr/bin/env -S asl run\n---\nasl_version: \"3.0\"\nname: \"{}\"\ndescription: \"Draft tool '{}' under construction\"\ninterface:\n  protocol: \"mcp-tool-v1\"\n  entrypoint: \"run\"\n---\n\n# {}\n\nTool instructions here.\n\n```asl:deterministic\ndef run(ctx, input):\n    return {{\"status\": \"ok\"}}\n```\n",
            stem, stem, stem
        )),
        "asl" => Some(format!(
            "#!/usr/bin/env -S asl run\n---\nasl_version: \"3.0\"\nname: \"{}\"\ndescription: \"Draft module '{}' under construction\"\n---\n\n# {}\n\nModule instructions here.\n",
            stem, stem, stem
        )),
        _ => None,
    }
}

#[cfg(unix)]
fn set_executable_permission(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(path, perms);
    }
}

#[cfg(not(unix))]
fn set_executable_permission(_path: &Path) {}

fn process_asl_file(
    path: &Path,
    parser: &CommonMarkYamlParser,
    stats: &mut SyncStats,
) -> Result<()> {
    if is_ignored_path(path) || !asl_spec::is_asl_file(path) {
        return Ok(());
    }

    let mut content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {:?}", path))?;

    if content.trim().is_empty() {
        if let Some(scaffold) = scaffold_empty_asl_file(path) {
            let _ = fs::write(path, &scaffold);
            set_executable_permission(path);
            content = scaffold;
        }
    }

    if asl_spec::is_shadow_eligible(path) {
        let doc = parser
            .parse(&content)
            .with_context(|| format!("Failed to parse {:?}", path))?;

        match project_shadow_markdown(path, &doc)? {
            ShadowProjectResult::Created(p) => {
                println!("⚡ Shadow created: {:?}", p);
                stats.created += 1;
            }
            ShadowProjectResult::Updated(p) => {
                println!("⚡ Shadow updated: {:?}", p);
                stats.updated += 1;
            }
            ShadowProjectResult::CollisionProtected(p) => {
                println!("⚠️ Collision protected: {:?}", p);
                stats.collisions += 1;
            }
            ShadowProjectResult::Unchanged(_) => {
                stats.unchanged += 1;
            }
            ShadowProjectResult::Skipped(_) => {}
        }
    }
    Ok(())
}

fn sync_internal(target_path: &Path, parser: &CommonMarkYamlParser, verbose: bool) -> Result<SyncStats> {
    let mut stats = SyncStats::default();

    if target_path.is_file() {
        process_asl_file(target_path, parser, &mut stats)?;
    } else if target_path.is_dir() {
        sync_dir_recursive(target_path, parser, &mut stats)?;
    }

    if verbose {
        println!("\n📊 Shadow Synchronization Summary:");
        println!("  Created:              {}", stats.created);
        println!("  Updated:              {}", stats.updated);
        println!("  Unchanged:            {}", stats.unchanged);
        println!("  Collisions Protected: {}", stats.collisions);
        println!("  Orphans Removed:      {}", stats.orphaned_removed);
    }

    Ok(stats)
}

fn sync_dir_recursive(
    dir: &Path,
    parser: &CommonMarkYamlParser,
    stats: &mut SyncStats,
) -> Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_symlink() {
                continue;
            }
            let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
            if path.is_dir() {
                if asl_parser::is_skippable_dir(file_name) {
                    continue;
                }
                sync_dir_recursive(&path, parser, stats)?;
            } else if asl_spec::is_asl_file(&path) {
                let _ = process_asl_file(&path, parser, stats);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md")
                && asl_parser::is_shadow_markdown_file(&path)
            {
                let expected_skill = asl_parser::get_expected_skill_path(&path);
                if !expected_skill.exists() {
                    let _ = fs::remove_file(&path);
                    stats.orphaned_removed += 1;
                    println!("🧹 Orphan removed: {:?}", path);
                }
            }
        }
    }
    Ok(())
}

/// Continuously monitors directory and projects shadows in real-time (EC-7 / Zero-Touch)
pub fn handle_watch_shadows(
    target_path: &Path,
    parser: &CommonMarkYamlParser,
    interval_ms: u64,
) -> Result<()> {
    println!(
        "👀 ASL Shadow Watcher active at {:?} (interval: {}ms).",
        target_path, interval_ms
    );
    println!("Press Ctrl+C to terminate...\n");

    let sleep_dur = std::time::Duration::from_millis(interval_ms);
    loop {
        let _ = handle_sync_quiet(target_path, parser);
        std::thread::sleep(sleep_dur);
    }
}

fn handle_sync_quiet(target_path: &Path, parser: &CommonMarkYamlParser) -> Result<()> {
    if target_path.is_dir() {
        let mut stats = SyncStats::default();
        sync_dir_recursive(target_path, parser, &mut stats)?;
        let _ = clean_orphaned_shadows(target_path);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_shadows_lifecycle() {
        let temp_dir = std::env::temp_dir().join(format!(
            "asl_cli_sync_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let skill_content = r#"---
asl_version: "3.0"
name: "cli-test-skill"
description: "Testando sync-shadows"
interface:
  entrypoint: "run"
---
# Instruções
Executar teste de sincronização.
```asl
def run(ctx, input):
    return {}
```
"#;
        let skill_path = temp_dir.join("cli_test.skill");
        fs::write(&skill_path, skill_content).unwrap();

        let parser = CommonMarkYamlParser::new();

        // 1. Primeira sincronização: deve criar a sombra
        let stats1 = handle_sync_shadows(&temp_dir, &parser).unwrap();
        assert_eq!(stats1.created, 1);
        assert_eq!(stats1.unchanged, 0);
        let shadow_md = temp_dir.join("cli_test.md");
        assert!(shadow_md.exists());

        // 2. Segunda sincronização: idempotência (no-op)
        let stats2 = handle_sync_shadows(&temp_dir, &parser).unwrap();
        assert_eq!(stats2.created, 0);
        assert_eq!(stats2.unchanged, 1);

        // 3. Exclusão do .skill: deve remover órfão
        fs::remove_file(&skill_path).unwrap();
        let stats3 = handle_sync_shadows(&temp_dir, &parser).unwrap();
        assert_eq!(stats3.orphaned_removed, 1);
        assert!(!shadow_md.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_empty_files_triad_scaffolding() {
        let temp_dir = std::env::temp_dir().join(format!(
            "asl_cli_triad_scaffold_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let skill_path = temp_dir.join("test_auto.skill");
        let tool_path = temp_dir.join("test_auto.tool");
        let asl_path = temp_dir.join("test_auto.asl");

        fs::write(&skill_path, "").unwrap();
        fs::write(&tool_path, "").unwrap();
        fs::write(&asl_path, "").unwrap();

        let parser = CommonMarkYamlParser::new();
        let stats = handle_sync_shadows(&temp_dir, &parser).unwrap();
        assert_eq!(stats.created, 1, "Only .skill generates shadow .md");

        // .skill checks
        let skill_content = fs::read_to_string(&skill_path).unwrap();
        assert!(skill_content.starts_with("#!/usr/bin/env -S asl run"));
        assert!(skill_content.contains("name: \"test_auto\""));
        let shadow_md = temp_dir.join("test_auto.md");
        assert!(shadow_md.exists());
        let md_content = fs::read_to_string(&shadow_md).unwrap();
        assert!(!md_content.contains("Claude Code"));
        assert!(!md_content.contains("token savings"));
        assert!(!md_content.contains("Directive for AI Agents"));

        // .tool checks
        let tool_content = fs::read_to_string(&tool_path).unwrap();
        assert!(tool_content.starts_with("#!/usr/bin/env -S asl run"));
        assert!(tool_content.contains("protocol: \"mcp-tool-v1\""));

        // .asl checks
        let asl_content = fs::read_to_string(&asl_path).unwrap();
        assert!(asl_content.starts_with("#!/usr/bin/env -S asl run"));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for p in [&skill_path, &tool_path, &asl_path] {
                let meta = fs::metadata(p).unwrap();
                assert_eq!(meta.permissions().mode() & 0o111, 0o111, "Must be executable");
            }
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
