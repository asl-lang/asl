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

/// Sincroniza projeções sombra de Markdown para um arquivo .skill ou diretório completo
pub fn handle_sync_shadows(target_path: &Path, parser: &CommonMarkYamlParser) -> Result<SyncStats> {
    let mut stats = SyncStats::default();

    if target_path.is_file() {
        if target_path.extension().and_then(|e| e.to_str()) == Some("skill")
            && !is_ignored_path(target_path)
        {
            let content = fs::read_to_string(target_path)
                .with_context(|| format!("Falha ao ler {:?}", target_path))?;
            let doc = parser
                .parse(&content)
                .with_context(|| format!("Falha ao analisar {:?}", target_path))?;
            match project_shadow_markdown(target_path, &doc)? {
                ShadowProjectResult::Created(p) => {
                    println!("⚡ Criada sombra: {:?}", p);
                    stats.created += 1;
                }
                ShadowProjectResult::Updated(p) => {
                    println!("⚡ Atualizada sombra: {:?}", p);
                    stats.updated += 1;
                }
                ShadowProjectResult::CollisionProtected(p) => {
                    println!("⚠️ Conflito protegido: {:?}", p);
                    stats.collisions += 1;
                }
                ShadowProjectResult::Unchanged(_) => {
                    stats.unchanged += 1;
                }
                ShadowProjectResult::Skipped(_) => {}
            }
        }
    } else if target_path.is_dir() {
        sync_dir_recursive(target_path, parser, &mut stats)?;
        let removed = clean_orphaned_shadows(target_path).unwrap_or_default();
        stats.orphaned_removed = removed.len();
        for r in removed {
            println!("🧹 Órfão removido: {:?}", r);
        }
    }

    println!("\n📊 Resumo da Sincronização Sombra:");
    println!("  Criadas:             {}", stats.created);
    println!("  Atualizadas:         {}", stats.updated);
    println!("  Inalteradas:         {}", stats.unchanged);
    println!("  Colisões Protegidas: {}", stats.collisions);
    println!("  Órfãos Removidos:    {}", stats.orphaned_removed);

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
            if path.is_dir() {
                sync_dir_recursive(&path, parser, stats)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("skill")
                && !is_ignored_path(&path)
            {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(doc) = parser.parse(&content) {
                        match project_shadow_markdown(&path, &doc)? {
                            ShadowProjectResult::Created(p) => {
                                println!("⚡ Criada sombra: {:?}", p);
                                stats.created += 1;
                            }
                            ShadowProjectResult::Updated(p) => {
                                println!("⚡ Atualizada sombra: {:?}", p);
                                stats.updated += 1;
                            }
                            ShadowProjectResult::CollisionProtected(p) => {
                                println!("⚠️ Conflito protegido: {:?}", p);
                                stats.collisions += 1;
                            }
                            ShadowProjectResult::Unchanged(_) => {
                                stats.unchanged += 1;
                            }
                            ShadowProjectResult::Skipped(_) => {}
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Monitora continuamente diretório e projeta sombras em tempo real (EC-7 / Zero-Touch)
pub fn handle_watch_shadows(
    target_path: &Path,
    parser: &CommonMarkYamlParser,
    interval_ms: u64,
) -> Result<()> {
    println!(
        "👀 ASL Shadow Watcher ativo em {:?} (intervalo: {}ms).",
        target_path, interval_ms
    );
    println!("Pressione Ctrl+C para encerrar...\n");

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
}
