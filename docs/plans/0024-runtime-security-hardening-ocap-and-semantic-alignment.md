# Plano de Implementação: Endurecimento de Segurança do Runtime, Confinamento OCap Estrito e Alinhamento Semântico

- **ADR Vinculado**: `docs/adrs/0024-runtime-security-hardening-ocap-and-semantic-alignment.md`
- **Data**: 2026-09-19
- **Responsável**: Antigravity & Jean
- **Meta**: Sanar todas as 12 vulnerabilidades auditadas e as 4 falhas arquiteturais críticas descobertas no runtime ASL, assegurando conformidade estrita com os 7 Axiomas.

---

## Fase 1: Política do Host, Sanitização de Entrypoint e Linter Seguro

### 1.1 Objetivo da Fase
1. Criar `HostSecurityPolicy` em `asl-spec` com cálculo de interseção estrita contra as capacidades solicitadas pela skill.
2. Adicionar validação estrita de identificador do entrypoint contra injeção de código Starlark.
3. Desacoplar `asl check` da execução real, garantindo que o linter não dispare I/O ou rede e usando `MockSecurityContext` no `--dry-run`.

### 1.2 Código a Implementar (Exemplo Exaustivo)

Em `runtime/crates/asl-spec/src/capabilities.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSecurityPolicy {
    #[serde(default)]
    pub allowed_fs_read_roots: Vec<String>,
    #[serde(default)]
    pub allowed_fs_write_roots: Vec<String>,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub allowed_env_keys: Vec<String>,
    #[serde(default = "default_policy_fuel")]
    pub max_fuel_opcodes: u64,
}

fn default_policy_fuel() -> u64 {
    1_000_000
}

impl HostSecurityPolicy {
    pub fn intersect(&self, requested: &SkillCapabilities) -> Result<SkillCapabilities, AslError> {
        let mut effective = SkillCapabilities::default();

        // 1. Interseção de leitura
        for root in &requested.fs.confined_read_roots {
            if self.allowed_fs_read_roots.iter().any(|a| a == "*" || root.starts_with(a)) {
                effective.fs.confined_read_roots.push(root.clone());
            } else {
                return Err(AslError::CapabilityViolation(format!(
                    "Host policy denied FS read root: '{}'", root
                )));
            }
        }

        // 2. Interseção de escrita
        for root in &requested.fs.allow_write {
            if self.allowed_fs_write_roots.iter().any(|a| a == "*" || root.starts_with(a)) {
                effective.fs.allow_write.push(root.clone());
            } else {
                return Err(AslError::CapabilityViolation(format!(
                    "Host policy denied FS write root: '{}'", root
                )));
            }
        }

        // 3. Interseção de domínios
        for domain in &requested.net.allow_domains {
            if self.allowed_domains.iter().any(|d| d == "*" || d == domain || domain.ends_with(&format!(".{}", d))) {
                effective.net.allow_domains.push(domain.clone());
            } else {
                return Err(AslError::CapabilityViolation(format!(
                    "Host policy denied network domain: '{}'", domain
                )));
            }
        }

        // 4. Interseção de variáveis de ambiente
        for key in &requested.env.allow_keys {
            if self.allowed_env_keys.iter().any(|k| k == "*" || k == key) {
                effective.env.allow_keys.push(key.clone());
            } else {
                return Err(AslError::CapabilityViolation(format!(
                    "Host policy denied environment key: '{}'", key
                )));
            }
        }

        Ok(effective)
    }
}
```

Em `runtime/crates/asl-spec/src/lib.rs`:
```rust
pub fn validate_entrypoint_identifier(ep: &str) -> Result<(), AslError> {
    if ep.is_empty() {
        return Err(AslError::InvalidEntrypoint("Entrypoint cannot be empty".to_string()));
    }
    let first = ep.chars().next().unwrap();
    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(AslError::InvalidEntrypoint(format!(
            "Entrypoint must start with ASCII letter or underscore: '{}'", ep
        )));
    }
    if !ep.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(AslError::InvalidEntrypoint(format!(
            "Entrypoint contains invalid characters: '{}'", ep
        )));
    }
    Ok(())
}
```

Em `runtime/crates/asl-cli/src/prefix_cmds.rs` (Refatoração de `handle_check`):
```rust
pub fn handle_check(
    skill_file: &Path,
    parser: &CommonMarkYamlParser,
    engine: &StarlarkEngine,
    dry_run: bool,
    no_shadow: bool,
) -> Result<()> {
    // 1. Parsing estático e validação de manifesto
    let content = fs::read_to_string(skill_file)?;
    let doc = parser.parse(&content)?;

    // 2. Validação estrita do identificador do entrypoint
    asl_spec::validate_entrypoint_identifier(&doc.manifest.interface.entrypoint)?;

    // 3. Validação do digest e da assinatura se presente
    // ...

    // 4. Execução SOMENTE se dry_run for true, e OBRIGATORIAMENTE usando MockSecurityContext
    if dry_run {
        let mock_ctx = asl_security::MockSecurityContext::new(doc.manifest.limits.max_fuel_opcodes);
        let res = engine.execute(
            &doc.deterministic_code,
            &doc.manifest.interface.entrypoint,
            &serde_json::json!({}),
            &mock_ctx,
            &doc.manifest.limits,
        )?;
        println!("Dry Run: ✅ Executed safely in-memory -> {:?}", res.output);
    } else {
        println!("Compilation: ✅ Syntactically valid (entrypoint '{}' compiled)", doc.manifest.interface.entrypoint);
    }

    Ok(())
}
```

### 1.3 Testes Unitários da Fase
```rust
#[test]
fn test_host_policy_intersection_denial() {
    let mut policy = HostSecurityPolicy::default();
    policy.allowed_domains = vec!["api.github.com".to_string()];

    let mut requested = SkillCapabilities::default();
    requested.net.allow_domains = vec!["evil.com".to_string()];

    let res = policy.intersect(&requested);
    assert!(matches!(res, Err(AslError::CapabilityViolation(_))));
}

#[test]
fn test_entrypoint_injection_rejected() {
    assert!(validate_entrypoint_identifier("run; pwn()").is_err());
    assert!(validate_entrypoint_identifier("123run").is_err());
    assert!(validate_entrypoint_identifier("valid_entrypoint_1").is_ok());
}
```

### 1.4 Verificação Local
```bash
cargo test -p asl-spec -p asl-cli
cargo clippy -p asl-spec -p asl-cli -- -D warnings
```

### 1.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-spec runtime/crates/asl-cli docs/plans/
git commit -m "feat(security): implementar HostSecurityPolicy, validar entrypoint e isolar asl check"
git push origin main
```

---

## Fase 2: OCap Puro no Starlark e Fuel Metering com Hard Traps

### 2.1 Objetivo da Fase
1. Remover funções nativas de I/O de `GlobalsBuilder`, eliminando a autoridade ambiente global.
2. Criar tipos OCap dedicados que vinculam métodos à instância `ctx` passada por parâmetro.
3. Implementar contagem de instruções no Starlark e hard trap com `Err(AslError::FuelExhausted)` no `consume_fuel`.
4. Integrar watchdog com cancelamento assíncrono para cumprir `limits.wall_clock_timeout_ms`.

### 2.2 Código a Implementar (Exemplo Exaustivo)

Em `runtime/crates/asl-security/src/lib.rs`:
```rust
pub fn consume_fuel(&self, amount: u64) -> Result<(), AslError> {
    let prev = self.fuel_consumed.fetch_add(amount, Ordering::SeqCst);
    let total = prev.saturating_add(amount);
    if total > self.fuel_budget {
        return Err(AslError::FuelExhausted {
            budget: self.fuel_budget,
            consumed: total,
        });
    }
    Ok(())
}
```

Em `runtime/crates/asl-vm-starlark/src/lib.rs`:
- Remover `asl_natives` do `GlobalsBuilder`.
- Configurar hook de interrupção ou verificação de fuel/timeout a cada chamada de função.

### 2.3 Testes Unitários da Fase
```rust
#[test]
fn test_ambient_authority_eradication() {
    // Código Starlark chamando asl_native_fs_read diretamente deve falhar
    let script = "def run(ctx, input):\n    return asl_native_fs_read('test.txt')";
    let res = engine.execute(script, "run", &json!({}), &ctx, &limits);
    assert!(res.is_err());
}

#[test]
fn test_fuel_exhaustion_halts_execution() {
    let mut limits = SkillLimits::default();
    limits.max_fuel_opcodes = 5;
    let ctx = MockSecurityContext::new(5);
    // Operação que gasta 10 de fuel deve abortar imediatamente
    let res = ctx.http_request("GET", "https://api.test", &[], None);
    assert!(matches!(res, Err(AslError::FuelExhausted { .. })));
}
```

### 2.4 Verificação Local
```bash
cargo test -p asl-security -p asl-vm-starlark
cargo clippy -p asl-security -p asl-vm-starlark -- -D warnings
```

### 2.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-security runtime/crates/asl-vm-starlark docs/plans/
git commit -m "feat(vm-starlark): erradicar autoridade ambiente OCap e impor hard traps de fuel"
git push origin main
```

---

## Fase 3: Confinamento de Rede (SSRF/Redirects) e Validação de JSON Schema

### 3.1 Objetivo da Fase
1. Configurar `ureq` para desabilitar redirects automáticos e validar resolução de DNS contra IPs privados/loopback/metadados.
2. Adicionar o crate `jsonschema` e validar `input_args` e `output` antes e após a execução da VM.
3. Isolar o contexto de segurança em `asl serve` (MCP), criando instâncias efêmeras por chamada de ferramenta.

### 3.2 Código a Implementar (Exemplo Exaustivo)

Em `runtime/crates/asl-security/src/net.rs`:
```rust
pub fn execute_http_request(
    method: &str,
    url_str: &str,
    headers: &[(String, String)],
    body: Option<&str>,
    allowed_domains: &[String],
    timeout_ms: u64,
) -> Result<HttpResponsePayload> {
    let domain = extract_domain(url_str)?;
    if !is_domain_allowed(&domain, allowed_domains) {
        return Err(AslError::CapabilityViolation(format!(
            "Network access denied: domain '{}' is not authorized", domain
        )));
    }

    // Validação de IP anti-SSRF
    let host = domain.split(':').next().unwrap();
    if host == "localhost" || host == "127.0.0.1" || host == "::1" || host == "169.254.169.254" {
        return Err(AslError::CapabilityViolation(format!(
            "Access to private/metadata IP is prohibited: '{}'", host
        )));
    }

    let config = ureq::config::Config::builder()
        .timeout_global(Some(Duration::from_millis(timeout_ms)))
        .max_redirects(0) // Prevenção estrita contra bypass de domínio via redirect
        .build();
    let agent: ureq::Agent = config.into();
    // ...
}
```

Em `runtime/crates/asl-core-traits/src/lib.rs`:
```rust
pub fn validate_json_schema(schema: &Value, instance: &Value) -> Result<(), AslError> {
    if schema.is_null() || schema.as_object().map(|o| o.is_empty()).unwrap_or(false) {
        return Ok(());
    }
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| AslError::InvalidSchema(e.to_string()))?;
    if let Err(mut errors) = validator.validate(instance) {
        let first_err = errors.next().map(|e| e.to_string()).unwrap_or_default();
        return Err(AslError::SchemaValidation(first_err));
    }
    Ok(())
}
```

### 3.3 Testes Unitários da Fase
```rust
#[test]
fn test_ssrf_redirect_and_metadata_ip_blocked() {
    let caps = vec!["api.github.com".to_string()];
    let res = execute_http_request("GET", "http://169.254.169.254/latest/meta-data", &[], None, &caps, 1000);
    assert!(matches!(res, Err(AslError::CapabilityViolation(_))));
}

#[test]
fn test_input_schema_validation_rejection() {
    let schema = serde_json::json!({
        "type": "object",
        "required": ["username"],
        "properties": { "username": { "type": "string" } }
    });
    let invalid_input = serde_json::json!({ "username": 12345 });
    assert!(validate_json_schema(&schema, &invalid_input).is_err());
}
```

### 3.4 Verificação Local
```bash
cargo test -p asl-security -p asl-core-traits
cargo clippy -p asl-security -p asl-core-traits -- -D warnings
```

### 3.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-security runtime/crates/asl-core-traits docs/plans/
git commit -m "feat(net-schema): adicionar anti-SSRF, max_redirects(0) e validacao JSON Schema"
git push origin main
```

---

## Fase 4: Sincronização da Documentação Científica e Limpeza de Artefatos

### 4.1 Objetivo da Fase
1. Atualizar `ASL_SCIENTIFIC_PAPER.md` e `ARCHITECTURE.md` para remover menções a `bounded_while`, "Dual-Consumer AST" e "Taint Tracking", alinhando com a realidade do código.
2. Calibrar o cálculo de KV-Cache em `prefix_analyzer.rs` para atuar como estimativa estática de invariância.
3. Executar o script oficial de guardrails `./scripts/guardrail_check.sh`.

### 4.2 Verificação e Guardrail Completo
```bash
./scripts/guardrail_check.sh
```

### 4.3 Finalização da Fase (Commit & Push)
```bash
git add ASL_SCIENTIFIC_PAPER.md ARCHITECTURE.md runtime/crates/asl-parser docs/plans/
git commit -m "docs(scientific): alinhar especificações formais com arquitetura real do runtime"
git push origin main
```
