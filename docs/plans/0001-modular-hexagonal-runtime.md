# PLAN-0001: Implementação do Runtime Modular Hexagonal em Rust

- **ADR Vinculado**: [`docs/adrs/0001-hexagonal-ports-adapters.md`](../adrs/0001-hexagonal-ports-adapters.md)
- **Status**: Concluído (Referência Canônica)
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Construir o runtime do ASL decomposto em micro-crates ortogonais e testadas com zero dependências externas.

---

## Fase 1: Fundação Pura (`asl-spec`) e Portas Abstratas (`asl-core-traits`)

### 1.1 Objetivo
Criar os contratos fundamentais de tipos sem dependência de I/O e definir os traits das portas hexagonais.

### 1.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-spec/src/lib.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDocument {
    pub manifest: SkillManifest,
    pub semantic_section: String,
    pub deterministic_code: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLimits {
    pub max_fuel_opcodes: u64,
    pub max_heap_kib: u64,
    pub wall_clock_timeout_ms: u64,
}
```

Em `runtime/crates/asl-core-traits/src/lib.rs`:
```rust
pub trait EnginePort: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(
        &self,
        code: &str,
        entrypoint: &str,
        input_args: &Value,
        context: &dyn CapabilityContext,
        limits: &Limits,
    ) -> Result<ExecutionResult>;
}

pub trait CapabilityContext: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Option<String>>;
    fn check_fuel(&self) -> Result<u64>;
}
```

### 1.3 Testes Unitários da Fase
```rust
#[test]
fn test_skill_limits_default() {
    let limits = SkillLimits::default();
    assert_eq!(limits.max_fuel_opcodes, 1_000_000);
}
```

### 1.4 Verificação Local
```bash
cargo test -p asl-spec -p asl-core-traits
cargo clippy -p asl-spec -p asl-core-traits -- -D warnings
```

### 1.5 Finalização da Fase (Commit & Push na Main)
```bash
git add runtime/crates/asl-spec/ runtime/crates/asl-core-traits/
git commit -m "feat(core): criar tipos fundamentais asl-spec e portas asl-core-traits"
git push origin main
```

---

## Fase 2: Adaptadores de Sintaxe (`asl-parser`) e Confinamento (`asl-security`)

### 2.1 Objetivo
Implementar o parser híbrido CommonMark/YAML com cálculo de digest canônico e o contexto ocap com mock em memória.

### 2.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-parser/src/lib.rs`:
```rust
impl ParserPort for CommonMarkYamlParser {
    fn parse(&self, raw_content: &str) -> Result<SkillDocument> {
        let digest = compute_canonical_digest(raw_content);
        let (frontmatter, markdown) = extract_frontmatter_and_markdown(raw_content)?;
        let manifest: SkillManifest = serde_yaml::from_str(&frontmatter)?;
        let (semantic, code) = parse_markdown_blocks(&markdown)?;
        Ok(SkillDocument { manifest, semantic_section: semantic, deterministic_code: code, digest })
    }
}
```

Em `runtime/crates/asl-security/src/lib.rs`:
```rust
pub struct MockSecurityContext {
    virtual_fs: HashMap<String, String>,
    fuel_counter: AtomicU64,
}
```

### 2.3 Verificação Local
```bash
cargo test -p asl-parser -p asl-security
cargo clippy -p asl-parser -p asl-security -- -D warnings
```

### 2.4 Finalização da Fase (Commit & Push na Main)
```bash
git add runtime/crates/asl-parser/ runtime/crates/asl-security/
git commit -m "feat(adapters): implementar parser hibrido e mock security context"
git push origin main
```

---

## Fase 3: Motor Hermético Starlark (`asl-vm-starlark`)

### 3.1 Objetivo
Integrar o avaliador Starlark hermético com suporte a JSON, structs e Fuel Metering.

### 3.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-vm-starlark/src/lib.rs`:
```rust
impl EnginePort for StarlarkEngine {
    fn name(&self) -> &'static str { "starlark-hermetic" }
    fn execute(&self, code: &str, entrypoint: &str, input_args: &Value, ctx: &dyn CapabilityContext, limits: &Limits) -> Result<ExecutionResult> {
        let globals = Globals::extended_by(&[LibraryExtension::Json, LibraryExtension::StructType]);
        // Evaluator hermético com fuel e heap isolada
        ...
    }
}
```

### 3.3 Verificação Local
```bash
cargo test -p asl-vm-starlark
cargo clippy -p asl-vm-starlark -- -D warnings
```

### 3.4 Finalização da Fase (Commit & Push na Main)
```bash
git add runtime/crates/asl-vm-starlark/
git commit -m "feat(vm-starlark): implementar avaliador deterministico hermetico"
git push origin main
```

---

## Fase 4: Adaptador de Transporte MCP (`asl-protocol-mcp`)

### 4.1 Objetivo
Implementar o servidor JSON-RPC 2.0 para o protocolo Model Context Protocol sobre `stdio`.

### 4.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-protocol-mcp/src/lib.rs`:
```rust
pub struct McpServer<'a> {
    skills: Vec<SkillDocument>,
    engine: &'a dyn EnginePort,
    context: &'a dyn CapabilityContext,
}
impl<'a> McpServer<'a> {
    pub fn handle_message(&self, msg_str: &str) -> Option<JsonRpcResponse> { ... }
    pub fn run_stdio_loop<R: BufRead, W: Write>(&self, reader: R, writer: W) -> std::io::Result<()> { ... }
}
```

### 4.3 Verificação Local
```bash
cargo test -p asl-protocol-mcp
cargo clippy -p asl-protocol-mcp -- -D warnings
```

### 4.4 Finalização da Fase (Commit & Push na Main)
```bash
git add runtime/crates/asl-protocol-mcp/
git commit -m "feat(mcp): implementar servidor json-rpc stdio para o Model Context Protocol"
git push origin main
```

---

## Fase 5: Injeção de Dependências no CLI (`asl-cli`) e Testes E2E

### 5.1 Objetivo
Unificar os adaptadores via CLI com subcomandos `run`, `check`, `serve` e `compile-grammar` e testes de arquitetura.

### 5.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 5.3 Finalização da Fase (Commit & Push na Main)
```bash
git add runtime/crates/asl-cli/ scripts/ docs/
git commit -m "feat(cli): integrar adaptadores no binario asl e validar guardrails"
git push origin main
```
