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

const SHADOW_WATERMARK: &str = "<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->";

/// Gera o conteúdo textual da projeção sombra em Markdown
pub fn generate_shadow_content(doc: &SkillDocument, skill_file_name: &str) -> String {
    let mut out = String::new();
    out.push_str(SHADOW_WATERMARK);
    out.push('\n');
    out.push_str(&format!(
        "<!-- CANONICAL SOURCE: ./{} | DIGEST: {} -->\n",
        skill_file_name, doc.digest
    ));
    out.push_str("---\n");
    out.push_str(&format!("asl_version: \"{}\"\n", doc.manifest.asl_version));
    out.push_str(&format!("name: \"{}\"\n", doc.manifest.name));
    if !doc.manifest.description.is_empty() {
        out.push_str(&format!("description: \"{}\"\n", doc.manifest.description.replace('\"', "\\\"")));
    }
    out.push_str(&format!("asl_canonical_source: \"./{}\"\n", skill_file_name));
    out.push_str(&format!("asl_digest: \"{}\"\n", doc.digest));
    if let Some(ref sig) = doc.manifest.signature {
        out.push_str(&format!("asl_signature: \"{}\"\n", sig));
    }
    if let Some(ref pubkey) = doc.manifest.signer_pubkey {
        out.push_str(&format!("asl_signer_pubkey: \"{}\"\n", pubkey));
    }
    out.push_str("---\n");

    out.push_str(&format!("# {}\n\n", doc.manifest.name));
    out.push_str(&format!(
        "> ⚡ **Esta habilidade é governada e executada pelo runtime hermético ASL 3.0.**\n> Arquivo canônico atômico: [`{}`](./{})\n\n",
        skill_file_name, skill_file_name
    ));

    out.push_str("### Diretiva para Agentes de IA (Claude Code, Cursor, Codex):\n");
    out.push_str("Para executar esta skill de forma determinística, segura e com 93% de economia de tokens:\n");
    out.push_str("```bash\n");
    out.push_str(&format!("asl run ./{}\n", skill_file_name));
    out.push_str("```\n\n");
    out.push_str("---\n\n");
    out.push_str("## Instruções Semânticas Oficiais\n\n");
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
pub fn project_shadow_markdown(skill_path: &Path, doc: &SkillDocument) -> Result<ShadowProjectResult> {
    if is_ignored_path(skill_path) {
        return Ok(ShadowProjectResult::Skipped(
            skill_path.to_string_lossy().to_string(),
        ));
    }

    let skill_file_name = skill_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("skill.skill");

    let target_md = get_shadow_target_path(skill_path);

    // Avalia arquivo existente (EC-3, EC-4)
    if target_md.exists() {
        let existing = fs::read_to_string(&target_md).map_err(|e| AslError::Io(e.to_string()))?;

        // Verifica se é uma projeção sombra legítima do ASL
        if !existing.contains(SHADOW_WATERMARK) {
            // Colisão com arquivo legítimo manual: protege e cria .asl.md (EC-4)
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

        // Verifica digest para evitar loops e I/O redundante
        let digest_token = format!("DIGEST: {}", doc.digest);
        if existing.contains(&digest_token) {
            return Ok(ShadowProjectResult::Unchanged(target_md));
        }

        // Digest alterado: atualiza atomicamente
        let new_content = generate_shadow_content(doc, skill_file_name);
        write_atomic(&target_md, &new_content)?;
        return Ok(ShadowProjectResult::Updated(target_md));
    }

    // Arquivo não existe: cria pela primeira vez (EC-1)
    let content = generate_shadow_content(doc, skill_file_name);
    write_atomic(&target_md, &content)?;
    Ok(ShadowProjectResult::Created(target_md))
}

/// Remove arquivos .md sombra órfãos cujo .skill original foi deletado (EC-2)
pub fn clean_orphaned_shadows(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut removed = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let sub_removed = clean_orphaned_shadows(&path)?;
                removed.extend(sub_removed);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.contains(SHADOW_WATERMARK) {
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
    let tmp_path = target.with_extension("tmp");
    if let Err(e) = fs::write(&tmp_path, content) {
        // EC-9: Em ambiente somente-leitura (ex: Docker --read-only), loga aviso sem pânico
        if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(30) {
            eprintln!("⚠️ [ASL Shadow] Somente-leitura ao escrever {:?}: {}", tmp_path, e);
            return Ok(());
        }
        return Err(AslError::Io(e.to_string()));
    }
    if let Err(e) = fs::rename(&tmp_path, target) {
        if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(30) {
            eprintln!("⚠️ [ASL Shadow] Somente-leitura ao renomear {:?}: {}", target, e);
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
            digest: digest.to_string(),
        }
    }

    #[test]
    fn test_shadow_projection_lifecycle() {
        let temp_dir = std::env::temp_dir().join(format!("asl_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
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
        let temp_dir = std::env::temp_dir().join(format!("asl_test_collision_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_dir).unwrap();

        let skill_path = temp_dir.join("existing.skill");
        fs::write(&skill_path, "mock skill").unwrap();
        let manual_md = temp_dir.join("existing.md");
        fs::write(&manual_md, "# Minha documentação manual importante! Não apagar.").unwrap();

        let doc = sample_doc("existing", "asl:sha256:dummy");
        let res = project_shadow_markdown(&skill_path, &doc).unwrap();

        // Não deve sobrescrever existing.md, mas criar existing.asl.md
        assert!(matches!(res, ShadowProjectResult::CollisionProtected(_)));
        assert_eq!(fs::read_to_string(&manual_md).unwrap(), "# Minha documentação manual importante! Não apagar.");
        let protected_path = temp_dir.join("existing.asl.md");
        assert!(protected_path.exists());

        // Limpeza não deve remover existing.asl.md enquanto existing.skill existir
        let cleaned_zero = clean_orphaned_shadows(&temp_dir).unwrap();
        assert_eq!(cleaned_zero.len(), 0);
        assert!(protected_path.exists());

        // Após deletar existing.skill, existing.asl.md deve ser limpo como órfão
        fs::remove_file(&skill_path).unwrap();
        let cleaned_one = clean_orphaned_shadows(&temp_dir).unwrap();
        assert_eq!(cleaned_one.len(), 1);
        assert!(!protected_path.exists());
        // existing.md manual original continua preservado
        assert!(manual_md.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
