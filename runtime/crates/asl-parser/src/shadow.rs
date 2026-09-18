use asl_spec::{AslError, Result, SkillDocument};
use std::fs;
use std::path::{Path, PathBuf};

/// Resultado da operação de projeção sombra
#[derive(Debug, Clone, PartialEq)]
pub enum ShadowProjectResult {
    /// Arquivo .md sombra criado pela primeira vez
    Created(PathBuf),
    /// Arquivo .md sombra atualizado devido a mudança de digest
    Updated(PathBuf),
    /// Arquivo .md sombra idêntico (digest coincidente, no-op)
    Unchanged(PathBuf),
    /// Colisão prevenida com .md pré-existente não-gerado pelo ASL (criado .asl.md)
    CollisionProtected(PathBuf),
    /// Ignorado (arquivo temporário ou swap)
    Skipped(String),
}

const SHADOW_WATERMARK: &str =
    "<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->";

/// Gera o conteúdo textual da projeção sombra em Markdown
pub fn generate_shadow_content(doc: &SkillDocument, skill_file_name: &str) -> String {
    let clean_stem = skill_file_name
        .strip_suffix(".skill")
        .unwrap_or(skill_file_name);
    let effective_name = if doc.manifest.name == "draft-skill" || doc.manifest.name.is_empty() {
        clean_stem
    } else {
        &doc.manifest.name
    };

    let mut out = String::new();
    out.push_str(SHADOW_WATERMARK);
    out.push('\n');
    out.push_str(&format!(
        "<!-- CANONICAL SOURCE: ./{} | DIGEST: {} -->\n",
        skill_file_name, doc.digest
    ));
    out.push_str("---\n");
    out.push_str(&format!("asl_version: \"{}\"\n", doc.manifest.asl_version));
    let clean_name = effective_name.replace('\"', "\\\"").replace('\n', " ");
    out.push_str(&format!("name: \"{}\"\n", clean_name));
    if !doc.manifest.description.is_empty() {
        let clean_desc = doc
            .manifest
            .description
            .replace('\"', "\\\"")
            .replace('\n', " ");
        out.push_str(&format!("description: \"{}\"\n", clean_desc));
    }
    out.push_str(&format!(
        "asl_canonical_source: \"./{}\"\n",
        skill_file_name
    ));
    out.push_str(&format!("asl_digest: \"{}\"\n", doc.digest));
    if let Some(ref sig) = doc.manifest.signature {
        out.push_str(&format!("asl_signature: \"{}\"\n", sig));
    }
    if let Some(ref pubkey) = doc.manifest.signer_pubkey {
        out.push_str(&format!("asl_signer_pubkey: \"{}\"\n", pubkey));
    }
    out.push_str("---\n");

    out.push_str(&format!("# {}\n\n", effective_name));
    out.push_str(&format!(
        "> ⚡ **This skill is governed and executed by the ASL 3.0 hermetic runtime.**\n> Canonical atomic file: [`{}`](./{})\n\n",
        skill_file_name, skill_file_name
    ));

    out.push_str("### Directive for AI Agents (Claude Code, Cursor, Codex):\n");
    out.push_str("To execute this skill deterministically, securely, and with up to 93% token savings:\n");
    out.push_str("```bash\n");
    out.push_str(&format!("asl run ./{}\n", skill_file_name));
    out.push_str("```\n\n");
    out.push_str("---\n\n");
    out.push_str("## Official Semantic Instructions\n\n");
    out.push_str(&doc.semantic_section);
    out.push('\n');

    out
}

/// Determina o caminho canônico do .md correspondente ao .skill
pub fn get_shadow_target_path(skill_path: &Path) -> PathBuf {
    let file_name = skill_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("skill.skill");

    if file_name.eq_ignore_ascii_case("SKILL.skill") {
        skill_path.with_file_name("SKILL.md")
    } else {
        skill_path.with_extension("md")
    }
}

/// Verifica se o caminho deve ser ignorado (arquivos temporários, swap de editores - EC-11)
pub fn is_ignored_path(path: &Path) -> bool {
    let name = match path.file_name().and_then(|f| f.to_str()) {
        Some(n) => n,
        None => return true,
    };

    name.starts_with('.')
        || name.ends_with('~')
        || name.ends_with(".tmp")
        || name.ends_with(".swp")
        || name.ends_with(".bak")
}

/// Projeta e sincroniza atomicamente o arquivo .md sombra correspondente ao .skill
/// Apenas arquivos .skill geram sombra Markdown; outros formatos de IA operam sem projeção sombra.
pub fn project_shadow_markdown(
    skill_path: &Path,
    doc: &SkillDocument,
) -> Result<ShadowProjectResult> {
    if is_ignored_path(skill_path) || !asl_spec::is_shadow_eligible(skill_path) {
        return Ok(ShadowProjectResult::Skipped(
            skill_path.to_string_lossy().to_string(),
        ));
    }

    let skill_file_name = skill_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("skill.skill");

    let target_md = get_shadow_target_path(skill_path);

    // Evaluate existing file (EC-3, EC-4)
    if target_md.exists() {
        let existing = fs::read_to_string(&target_md).map_err(|e| AslError::Io(e.to_string()))?;

        // Check if it is a legitimate ASL shadow projection
        if !existing.trim_start().starts_with(SHADOW_WATERMARK) {
            // Collision with manual file: protect and create .asl.md (EC-4)
            let protected_md = skill_path.with_extension("asl.md");
            let digest_token = format!("DIGEST: {}", doc.digest);
            if protected_md.exists() {
                if let Ok(prot_content) = fs::read_to_string(&protected_md) {
                    if prot_content.contains(&digest_token) {
                        return Ok(ShadowProjectResult::CollisionProtected(protected_md));
                    }
                }
            }
            let content = generate_shadow_content(doc, skill_file_name);
            write_atomic(&protected_md, &content)?;
            return Ok(ShadowProjectResult::CollisionProtected(protected_md));
        }

        // Check digest to prevent redundant I/O loops
        let digest_token = format!("DIGEST: {}", doc.digest);
        if existing.contains(&digest_token) {
            return Ok(ShadowProjectResult::Unchanged(target_md));
        }

        // Changed digest: update atomically
        let new_content = generate_shadow_content(doc, skill_file_name);
        write_atomic(&target_md, &new_content)?;
        return Ok(ShadowProjectResult::Updated(target_md));
    }

    // File does not exist: create for the first time (EC-1)
    let content = generate_shadow_content(doc, skill_file_name);
    write_atomic(&target_md, &content)?;
    Ok(ShadowProjectResult::Created(target_md))
}

/// Removes orphaned shadow .md files whose original .skill was deleted (EC-2)
pub fn clean_orphaned_shadows(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut removed = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_symlink() {
                continue;
            }
            let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
            if path.is_dir() {
                if file_name.starts_with('.')
                    || file_name == "node_modules"
                    || file_name == "target"
                    || file_name == "dist"
                    || file_name == "build"
                    || file_name == "out"
                    || file_name == "vendor"
                    || file_name == "venv"
                    || file_name == "__pycache__"
                    || file_name == "Library"
                    || file_name == "Applications"
                {
                    continue;
                }
                if let Ok(sub_removed) = clean_orphaned_shadows(&path) {
                    removed.extend(sub_removed);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.trim_start().starts_with(SHADOW_WATERMARK) {
                        // Deriva o .skill original esperado tratando .asl.md e SKILL.md
                        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                        let expected_skill = if file_name.ends_with(".asl.md") {
                            let base = file_name.strip_suffix(".asl.md").unwrap_or("");
                            path.with_file_name(format!("{}.skill", base))
                        } else if file_name.eq_ignore_ascii_case("SKILL.md") {
                            path.with_file_name("SKILL.skill")
                        } else {
                            path.with_extension("skill")
                        };

                        if !expected_skill.exists() {
                            let _ = fs::remove_file(&path);
                            removed.push(path);
                        }
                    }
                }
            }
        }
    }

    Ok(removed)
}

/// Escreve de forma atômica utilizando arquivo temporário e rename com suporte a EC-9
fn write_atomic(target: &Path, content: &str) -> Result<()> {
    if let Some(parent) = target.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);
        }
    }
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    let tmp_path = target.with_extension(format!("tmp.{}.{}", pid, nanos));

    if let Err(e) = fs::write(&tmp_path, content) {
        // EC-9: In read-only environment (e.g. Docker --read-only), log warning without panic
        if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(30) {
            eprintln!(
                "⚠️ [ASL Shadow] Read-only filesystem when writing {:?}: {}",
                tmp_path, e
            );
            return Ok(());
        }
        return Err(AslError::Io(e.to_string()));
    }
    if let Err(e) = fs::rename(&tmp_path, target) {
        let _ = fs::remove_file(&tmp_path);
        if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(30) {
            eprintln!(
                "⚠️ [ASL Shadow] Somente-leitura ao renomear {:?}: {}",
                target, e
            );
            return Ok(());
        }
        return Err(AslError::Io(e.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use asl_spec::SkillManifest;

    fn sample_doc(name: &str, digest: &str) -> SkillDocument {
        SkillDocument {
            manifest: SkillManifest {
                asl_version: "3.0".to_string(),
                name: name.to_string(),
                description: "Uma skill de teste".to_string(),
                version: None,
                license: None,
                digest: Some(digest.to_string()),
                signature: None,
                signer_pubkey: None,
                interface: asl_spec::SkillInterface {
                    protocol: "mcp-v1".to_string(),
                    entrypoint: "run".to_string(),
                    input_schema: serde_json::json!({}),
                    output_schema: None,
                },
                capabilities: Default::default(),
                limits: Default::default(),
            },
            semantic_section: "# Regras Semânticas\nSiga sempre as convenções.".to_string(),
            deterministic_code: "def run(ctx, input): return input".to_string(),
            rules_code: None,
            digest: digest.to_string(),
        }
    }

    #[test]
    fn test_shadow_projection_lifecycle() {
        let temp_dir = std::env::temp_dir().join(format!(
            "asl_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let skill_path = temp_dir.join("test_skill.skill");
        fs::write(&skill_path, "mock skill content").unwrap();

        let doc1 = sample_doc("test_skill", "asl:sha256:digest_1");

        // 1. Criação do zero (EC-1)
        let res1 = project_shadow_markdown(&skill_path, &doc1).unwrap();
        assert!(matches!(res1, ShadowProjectResult::Created(_)));
        let md_path = temp_dir.join("test_skill.md");
        assert!(md_path.exists());
        let content1 = fs::read_to_string(&md_path).unwrap();
        assert!(content1.contains(SHADOW_WATERMARK));
        assert!(content1.contains("DIGEST: asl:sha256:digest_1"));

        // 2. Idempotência / Unchanged (Evita loops de I/O)
        let res2 = project_shadow_markdown(&skill_path, &doc1).unwrap();
        assert!(matches!(res2, ShadowProjectResult::Unchanged(_)));

        // 3. Atualização após alteração de digest (EC-3)
        let doc2 = sample_doc("test_skill", "asl:sha256:digest_2_updated");
        let res3 = project_shadow_markdown(&skill_path, &doc2).unwrap();
        assert!(matches!(res3, ShadowProjectResult::Updated(_)));
        let content2 = fs::read_to_string(&md_path).unwrap();
        assert!(content2.contains("DIGEST: asl:sha256:digest_2_updated"));

        // 4. Limpeza de órfão após exclusão do .skill (EC-2)
        fs::remove_file(&skill_path).unwrap();
        let cleaned = clean_orphaned_shadows(&temp_dir).unwrap();
        assert_eq!(cleaned.len(), 1);
        assert!(!md_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_collision_protection_preexisting_manual_md() {
        let temp_dir = std::env::temp_dir().join(format!(
            "asl_test_collision_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let skill_path = temp_dir.join("existing.skill");
        fs::write(&skill_path, "mock skill").unwrap();
        let manual_md = temp_dir.join("existing.md");
        fs::write(
            &manual_md,
            "# Important manual documentation! Do not delete.",
        )
        .unwrap();

        let doc = sample_doc("existing", "asl:sha256:dummy");
        let res = project_shadow_markdown(&skill_path, &doc).unwrap();

        // Must not overwrite existing.md, but create existing.asl.md
        assert!(matches!(res, ShadowProjectResult::CollisionProtected(_)));
        assert_eq!(
            fs::read_to_string(&manual_md).unwrap(),
            "# Important manual documentation! Do not delete."
        );
        let protected_path = temp_dir.join("existing.asl.md");
        assert!(protected_path.exists());

        // Cleanup should not remove existing.asl.md while existing.skill exists
        let cleaned_zero = clean_orphaned_shadows(&temp_dir).unwrap();
        assert_eq!(cleaned_zero.len(), 0);
        assert!(protected_path.exists());

        // After deleting existing.skill, existing.asl.md should be cleaned as an orphan
        fs::remove_file(&skill_path).unwrap();
        let cleaned_one = clean_orphaned_shadows(&temp_dir).unwrap();
        assert_eq!(cleaned_one.len(), 1);
        assert!(!protected_path.exists());
        // Original manual existing.md remains preserved
        assert!(manual_md.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_non_skill_extensions_skip_shadow_projection() {
        let temp_dir = std::env::temp_dir().join(format!(
            "asl_test_no_shadow_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let doc = sample_doc("test", "asl:sha256:dummy");
        let non_skill_exts = ["tool", "asl"];

        for ext in non_skill_exts {
            let path = temp_dir.join(format!("artifact.{}", ext));
            fs::write(&path, "content").unwrap();
            let res = project_shadow_markdown(&path, &doc).unwrap();
            assert!(
                matches!(res, ShadowProjectResult::Skipped(_)),
                "Extension .{} should be ignored in shadow projection",
                ext
            );
            let md = temp_dir.join("artifact.md");
            assert!(!md.exists(), "Should not generate .md for .{}", ext);
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
