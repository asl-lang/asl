use asl_core_traits::CapabilityContext;
use asl_parser::{clean_orphaned_shadows, project_shadow_markdown, ShadowProjectResult};
use asl_security::ConfinedSecurityContext;
use asl_spec::{AslError, FsCapabilities, SkillCapabilities, SkillDocument, SkillManifest};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempTestDir {
    pub path: PathBuf,
}

impl TempTestDir {
    fn new(prefix: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("asl_{}_{}_{}", prefix, std::process::id(), nanos));
        fs::create_dir_all(&path).expect("Falha ao criar diretório temporário para teste");
        Self { path }
    }
}

impl Drop for TempTestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn make_dummy_doc(name: &str, digest: &str) -> SkillDocument {
    SkillDocument {
        manifest: SkillManifest {
            asl_version: "3.0".to_string(),
            name: name.to_string(),
            description: "Documento de teste de segurança".to_string(),
            version: Some("1.0.0".to_string()),
            license: Some("MIT".to_string()),
            digest: Some(digest.to_string()),
            signature: None,
            signer_pubkey: None,
            interface: asl_spec::SkillInterface {
                protocol: "mcp-tool-v1".to_string(),
                entrypoint: "run".to_string(),
                input_schema: serde_json::json!({}),
                output_schema: None,
            },
            capabilities: Default::default(),
            limits: Default::default(),
        },
        semantic_section: "# Seção Semântica\nInstruções seguras.".to_string(),
        deterministic_code: "def run(ctx, input):\n    return input".to_string(),
        rules_code: None,
        digest: digest.to_string(),
    }
}

#[test]
fn test_path_traversal_attempts_blocked() {
    let temp = TempTestDir::new("traversal_guard");
    let sandbox = temp.path.join("sandbox");
    let outside = temp.path.join("outside");
    fs::create_dir_all(&sandbox).unwrap();
    fs::create_dir_all(&outside).unwrap();

    let secret_file = outside.join("secret.txt");
    fs::write(&secret_file, "DADO_CONFIDENCIAL").unwrap();

    let caps = SkillCapabilities {
        fs: FsCapabilities {
            confined_read_roots: vec![sandbox.to_string_lossy().to_string()],
            allow_write: vec![],
        },
        net: Default::default(),
        wasi_components: vec![],
    };

    let ctx = ConfinedSecurityContext::from_capabilities(&caps, 1000);

    // 1. Tentativa de traversal relativo usando ../
    let traversal_rel = sandbox.join("../outside/secret.txt");
    let res_rel = ctx.read_file(&traversal_rel.to_string_lossy());
    assert!(
        res_rel.is_err(),
        "Traversal relativo com ../ deve ser bloqueado"
    );
    match res_rel.unwrap_err() {
        AslError::CapabilityViolation(msg) => {
            assert!(msg.contains("Tentativa de fuga de diretório confinado"));
        }
        other => panic!("Esperado CapabilityViolation, obtido: {:?}", other),
    }

    // 2. Tentativa de caminho absoluto externo
    let res_abs = ctx.read_file(&secret_file.to_string_lossy());
    assert!(
        res_abs.is_err(),
        "Acesso absoluto fora da raiz autorizada deve ser bloqueado"
    );
    assert!(matches!(res_abs.unwrap_err(), AslError::CapabilityViolation(_)));

    // 3. Tentativa de /etc/passwd
    let res_passwd = ctx.read_file("/etc/passwd");
    assert!(res_passwd.is_err());
    assert!(matches!(res_passwd.unwrap_err(), AslError::CapabilityViolation(_)));
}

#[test]
fn test_empty_capabilities_denies_all_file_io() {
    let temp = TempTestDir::new("empty_caps");
    let test_file = temp.path.join("sample.txt");
    fs::write(&test_file, "conteúdo").unwrap();

    let empty_caps = SkillCapabilities::default();
    let ctx = ConfinedSecurityContext::from_capabilities(&empty_caps, 1000);

    let res = ctx.read_file(&test_file.to_string_lossy());
    assert!(res.is_err(), "Com raízes vazias, leitura deve ser negada");
    match res.unwrap_err() {
        AslError::CapabilityViolation(msg) => {
            assert!(msg.contains("nenhuma raiz confinada autorizada"));
        }
        other => panic!("Esperado CapabilityViolation, obtido: {:?}", other),
    }
}

#[test]
fn test_shadow_projection_strict_isolation_for_tool_and_asl() {
    let temp = TempTestDir::new("shadow_isolation");

    // 1. Formato .tool NÃO deve gerar .md sob hipótese alguma
    let tool_path = temp.path.join("calculator.tool");
    fs::write(&tool_path, "mock tool content").unwrap();
    let doc_tool = make_dummy_doc("calculator", "asl:sha256:hash_tool");

    let res_tool = project_shadow_markdown(&tool_path, &doc_tool).unwrap();
    assert!(
        matches!(res_tool, ShadowProjectResult::Skipped(_)),
        "Arquivo .tool deve ser ignorado pela projeção sombra"
    );
    assert!(
        !temp.path.join("calculator.md").exists(),
        "Nenhum arquivo .md deve ser gerado para .tool"
    );

    // 2. Formato .asl NÃO deve gerar .md sob hipótese alguma
    let asl_path = temp.path.join("pipeline.asl");
    fs::write(&asl_path, "mock asl content").unwrap();
    let doc_asl = make_dummy_doc("pipeline", "asl:sha256:hash_asl");

    let res_asl = project_shadow_markdown(&asl_path, &doc_asl).unwrap();
    assert!(
        matches!(res_asl, ShadowProjectResult::Skipped(_)),
        "Arquivo .asl deve ser ignorado pela projeção sombra"
    );
    assert!(
        !temp.path.join("pipeline.md").exists(),
        "Nenhum arquivo .md deve ser gerado para .asl"
    );

    // 3. Formato .skill DEVE gerar .md e mantê-lo sincronizado
    let skill_path = temp.path.join("reviewer.skill");
    fs::write(&skill_path, "mock skill content").unwrap();
    let doc_skill = make_dummy_doc("reviewer", "asl:sha256:hash_skill_1");

    let res_skill_created = project_shadow_markdown(&skill_path, &doc_skill).unwrap();
    assert!(
        matches!(res_skill_created, ShadowProjectResult::Created(_)),
        "Primeira execução em .skill deve criar sombra"
    );
    let md_path = temp.path.join("reviewer.md");
    assert!(md_path.exists(), "Arquivo reviewer.md deve existir");

    // Segunda execução com mesmo digest -> Unchanged
    let res_skill_unchanged = project_shadow_markdown(&skill_path, &doc_skill).unwrap();
    assert!(
        matches!(res_skill_unchanged, ShadowProjectResult::Unchanged(_)),
        "Segunda execução idêntica deve retornar Unchanged"
    );

    // Atualização de digest -> Updated
    let doc_skill_updated = make_dummy_doc("reviewer", "asl:sha256:hash_skill_2");
    let res_skill_updated = project_shadow_markdown(&skill_path, &doc_skill_updated).unwrap();
    assert!(
        matches!(res_skill_updated, ShadowProjectResult::Updated(_)),
        "Atualização de digest deve retornar Updated"
    );
    let updated_content = fs::read_to_string(&md_path).unwrap();
    assert!(updated_content.contains("asl:sha256:hash_skill_2"));
}

#[test]
fn test_shadow_collision_protection_preserves_user_markdown() {
    let temp = TempTestDir::new("shadow_collision");

    // Usuário já possui um arquivo README.md ou manual.md criado à mão
    let manual_md = temp.path.join("manual.md");
    let original_user_notes = "# Anotações Pessoais\nTexto manual que NÃO pode ser sobrescrito!";
    fs::write(&manual_md, original_user_notes).unwrap();

    // Cria skill com mesmo nome base: manual.skill
    let skill_path = temp.path.join("manual.skill");
    fs::write(&skill_path, "mock skill").unwrap();
    let doc = make_dummy_doc("manual", "asl:sha256:digest_manual");

    let res = project_shadow_markdown(&skill_path, &doc).unwrap();
    assert!(
        matches!(res, ShadowProjectResult::CollisionProtected(_)),
        "Deve proteger o markdown do usuário e criar .asl.md"
    );

    // O arquivo manual.md original DEVE continuar intacto bit-a-bit
    let current_manual_content = fs::read_to_string(&manual_md).unwrap();
    assert_eq!(current_manual_content, original_user_notes);

    // A projeção sombra protegida foi criada em manual.asl.md
    let protected_path = temp.path.join("manual.asl.md");
    assert!(protected_path.exists());
    let protected_content = fs::read_to_string(&protected_path).unwrap();
    assert!(protected_content.contains("asl:sha256:digest_manual"));
}

#[test]
fn test_clean_orphaned_shadows_preserves_non_shadow_files() {
    let temp = TempTestDir::new("clean_orphans");

    // 1. Arquivo de documentação manual (sem watermark do ASL)
    let readme_path = temp.path.join("README.md");
    fs::write(&readme_path, "# Documentação do Projeto").unwrap();

    // 2. Arquivos da tríade .tool e .asl
    let tool_path = temp.path.join("my_tool.tool");
    fs::write(&tool_path, "tool content").unwrap();

    let asl_path = temp.path.join("core.asl");
    fs::write(&asl_path, "asl content").unwrap();

    // 3. Skill ativa e sua sombra legítima
    let active_skill = temp.path.join("active.skill");
    fs::write(&active_skill, "active skill").unwrap();
    let active_doc = make_dummy_doc("active", "asl:sha256:active_hash");
    project_shadow_markdown(&active_skill, &active_doc).unwrap();
    let active_md = temp.path.join("active.md");
    assert!(active_md.exists());

    // 4. Sombra órfã (o arquivo .skill correspondente foi deletado pelo usuário)
    let orphan_skill = temp.path.join("deleted.skill");
    let orphan_doc = make_dummy_doc("deleted", "asl:sha256:deleted_hash");
    project_shadow_markdown(&orphan_skill, &orphan_doc).unwrap();
    let orphan_md = temp.path.join("deleted.md");
    assert!(orphan_md.exists());
    // Remove o .skill de origem simulando exclusão
    let _ = fs::remove_file(&orphan_skill);

    // Executa a limpeza de órfãos
    let cleaned_orphans = clean_orphaned_shadows(&temp.path).unwrap();
    assert_eq!(cleaned_orphans.len(), 1, "Exatamente uma sombra órfã deve ser removida");

    // Verificações
    assert!(!orphan_md.exists(), "deleted.md deve ter sido excluído");
    assert!(readme_path.exists(), "README.md manual NÃO pode ser excluído");
    assert!(active_md.exists(), "active.md da skill ativa deve ser preservado");
    assert!(tool_path.exists(), "my_tool.tool deve permanecer intocado");
    assert!(asl_path.exists(), "core.asl deve permanecer intocado");
}
