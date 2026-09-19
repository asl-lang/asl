# ADR-0024: Endurecimento de Segurança do Runtime, Confinamento OCap Estrito e Alinhamento Semântico

- **Status**: Aceito
- **Data**: 2026-09-19
- **Autores**: Antigravity (Advanced Agentic Coding) & Jean
- **Decisores**: Conselho de Arquitetura ASL
- **Crates Afetadas**: `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-vm-wasm`, `asl-parser`, `asl-protocol-mcp`, `asl-cli`, `asl-ffi`

---

## 1. Contexto e Problema

Uma auditoria exaustiva de segurança e conformidade arquitetural no runtime do **Agent Skill Language (ASL)** revelou divergências críticas entre as garantias teóricas publicadas (`ASL_SCIENTIFIC_PAPER.md`, `ARCHITECTURE.md`, ADRs anteriores) e a implementação em Rust:

1. **Confusão de Privilégios e Auto-Concessão de Capabilities**: As permissões declaradas no manifesto de uma skill (`capabilities: fs, net, env`) tornam-se automaticamente permissões concedidas em `asl run` e `asl-ffi`, sem um mecanismo do hospedeiro (*host*) para filtrar ou restringir acessos de rede e variáveis de ambiente. Declarar `net: true` ou `env: true` concede acesso irrestrito a qualquer host/IP e a todas as variáveis de ambiente do processo.
2. **Pseudo-OCap e Autoridade Ambiente**: A injeção de `ctx` no Starlark é uma mera fachada sobre funções nativas registradas globalmente em `GlobalsBuilder` (`asl_native_fs_read`, `asl_native_http_request`, etc.). Qualquer código de skill pode ignorar `ctx` e invocar diretamente as funções nativas globais, violando o princípio fundamental de Object-Capabilities (Axioma 3).
3. **Ausência de Fuel Metering em Opcodes e Traps**: O Starlark avalia módulos sem medição de passos de bytecode. O medidor de `fuel` só debita chamadas de I/O, mas a função `consume_fuel` jamais compara com o orçamento nem aborta a execução, impossibilitando a contenção de loops infinitos ou computação pesada (Axioma 5).
4. **Limites Inoperantes (`max_heap_kib` e `wall_clock_timeout_ms`)**: `max_heap_kib` não é consultado nem aplicado em nenhum lugar do runtime. `wall_clock_timeout_ms` possui um valor hardcoded de 15s usado apenas no cliente HTTP, sem timeout na avaliação da VM.
5. **Efeitos Colaterais Reais no `asl check`**: O comando de validação/linter `asl check` executa o entrypoint da skill utilizando `ConfinedSecurityContext`, disparando chamadas reais de rede e escrita em disco durante checagens de integridade.
6. **Divergência entre Entrypoints**: `asl run` (CLI), `asl serve` (MCP) e `asl_skill_execute` (FFI) aplicam regras de segurança mutuamente incompatíveis. O `asl serve` descarta todas as capacidades da skill e compartilha uma única instância global de contexto, acumulando o débito de fuel entre clientes diferentes.
7. **Ausência de Validação de Schemas JSON**: `input_schema` e `output_schema` não são validados no runtime antes ou depois da execução da VM.
8. **Incompatibilidade Semântica e Ausência de WIT no WASM**: O motor `asl-vm-wasm` ignora o `CapabilityContext`, só aceita números escalares primitivos, empacota retornos em envelopes arbitrários e usa `wasmi` Core MVP em vez do alegado WASI Preview 2 Component Model / WIT.
9. **Vulnerabilidades Críticas Adicionais**:
   - **Injeção de Código**: O nome do `entrypoint` é interpolado sem sanitização em script Starlark (`asl_result = {entrypoint}(asl_ctx, asl_raw_input)`).
   - **SSRF via Redirects**: O cliente HTTP `ureq` segue redirecionamentos por padrão sem revalidar a lista de domínios autorizados, permitindo acesso a IPs privados e metadados de nuvem (`169.254.169.254`).
   - **Assinaturas Criptográficas Ignoradas**: `asl run` não valida a assinatura Ed25519, e `asl check` valida a assinatura contra a chave pública declarada no próprio arquivo não confiável.
   - **Ficções Documentais**: `bounded_while`, "Dual-Consumer AST", "Taint Tracking" e "100% KV-Cache" foram descritos no paper formal mas não possuem respaldo na implementação.

---

## 2. Proposta Detalhada da Decisão

### 2.1 Separação Formal entre Requested e Granted Capabilities

Introduzir na camada de domínio (`asl-spec`) o modelo formal de política do hospedeiro:

```rust
// crates/asl-spec/src/capabilities.rs

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSecurityPolicy {
    pub allowed_fs_read_roots: Vec<PathBuf>,
    pub allowed_fs_write_roots: Vec<PathBuf>,
    pub allowed_domains: Vec<String>,
    pub allowed_env_keys: Vec<String>,
    pub max_fuel_opcodes: u64,
    pub max_timeout_ms: u64,
    pub max_heap_kib: u64,
    pub allow_all_network: bool,
    pub allow_all_env: bool,
}

impl HostSecurityPolicy {
    /// Computa a interseção estrita entre o solicitado pela skill e o concedido pelo hospedeiro.
    pub fn intersect(&self, requested: &SkillCapabilities) -> Result<SkillCapabilities, AslError> {
        // Interseção restrita de FS Read, FS Write, Network Domains e Env Keys.
        // Se a skill requisitar recursos não autorizados na política do host,
        // retorna Err(AslError::CapabilityViolation).
    }
}
```

### 2.2 Verdadeiro Modelo Object-Capability (OCap) no Starlark

1. **Eliminação de Globais**: Nenhuma função nativa de I/O será registrada no `GlobalsBuilder`. O ambiente global conterá apenas a biblioteca padrão pura e hermética do Starlark (`json`, `struct`, helpers de string).
2. **Encapsulamento em Tipos Starlark**: Criar tipos customizados derivados de `starlark::values::StarlarkValue` (`AslContext`, `FsCap`, `HttpCap`, `CryptoCap`, `EnvCap`) que contêm a referência segura para a trait `CapabilityContext`.
3. **Impossibilidade de Bypass**: O acesso a recursos externos só é possível através da invocação de métodos na instância `ctx` explicitamente passada como primeiro argumento da função entrypoint.

### 2.3 Fuel Metering Real e Interrupção por Exaustão

1. **Controle de Passos**: Configurar hooks de passos no avaliador Starlark para decrementar combustível por instrução/expressão avaliada.
2. **Hard Traps**: Modificar `CapabilityContext::consume_fuel` para que estoure um erro irrecuperável `Err(AslError::FuelExhausted)` caso o acumulado ultrapasse o orçamento:
   ```rust
   pub fn consume_fuel(&self, amount: u64) -> Result<(), AslError> {
       let current = self.fuel_consumed.fetch_add(amount, Ordering::SeqCst);
       if current + amount > self.fuel_budget {
           return Err(AslError::FuelExhausted {
               budget: self.fuel_budget,
               consumed: current + amount,
           });
       }
       Ok(())
   }
   ```

### 2.4 Watchdog de Timeout de Parede e Limites de Heap

1. Executar a avaliação do motor com monitor de cancelamento (`eval.request_cancel()`) acionado por temporizador assíncrono vinculado a `limits.wall_clock_timeout_ms`.
2. Monitorar o crescimento do heap na alocação de memória Starlark e WASM para barrar payloads acima de `limits.max_heap_kib`.

### 2.5 Isolamento Estático de `asl check`

1. `asl check` executará apenas parsing sintático, validação de JSON Schema, verificação criptográfica e compilação da AST Starlark. **Nenhum código será executado por padrão**.
2. A opção `--dry-run` utilizará estritamente `MockSecurityContext` (virtual FS e rede em memória), sendo terminantemente proibido o uso de `ConfinedSecurityContext` no linter.

### 2.6 Validação de JSON Schema em Runtime

Integrar o crate `jsonschema` para garantir o cumprimento estrito dos contratos antes e após a execução:
1. Validar `input_args` contra `manifest.interface.input_schema` antes da invocação.
2. Validar o valor retornado contra `manifest.interface.output_schema` antes de entregar o `ExecutionResult`.

### 2.7 Sanitização de Entrypoint e Defesa contra Injeção

Validar o identificador do entrypoint antes de qualquer interpolação no script prelúdio:
```rust
pub fn validate_entrypoint(ep: &str) -> Result<(), AslError> {
    if !ep.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || ep.is_empty() || ep.chars().next().unwrap().is_ascii_digit() {
        return Err(AslError::InvalidEntrypoint(format!("Invalid entrypoint: '{}'", ep)));
    }
    Ok(())
}
```

### 2.8 Defesa contra SSRF e Confinamento de Rede

1. Configurar `ureq` com `.max_redirects(0)` ou interceptar redirecionamentos para revalidar `is_domain_allowed` antes de seguir qualquer `Location`.
2. Resolver DNS e bloquear IPs de loopback (`127.0.0.0/8`, `::1`), RFC 1918 e metadados de nuvem (`169.254.169.254`).

### 2.9 Unificação da Aplicação de Políticas (CLI, MCP, FFI)

1. `asl run`: Recebe flags de host (`--allowed-root`, `--allow-domain`, `--policy-file`) e aplica a interseção estrita.
2. `asl serve`: Instancia um `CapabilityContext` efêmero e isolado **por requisição de ferramenta MCP**, derivado da política do servidor e das capabilities da skill chamada, eliminando a poluição de fuel compartilhado.
3. `asl-ffi`: Adiciona à API C a estrutura `asl_policy_t` para que a aplicação hospedeira defina formalmente as permissões concedidas.

---

## 3. Alternativas Consideradas

- **Alternativa A (Manter Globais no Starlark e Apenas Filtrar Nomes)**: Descartada pois manter autoridade em `eval.extra` permite vazamento lateral de referências e mantém a autoridade ambiente ativa no interpretador.
- **Alternativa B (Confiar nas Capacidades do Manifesto sem Host Policy)**: Descartada pois viola os princípios elementares de segurança para execução de código de terceiros (o agente ou skill dita suas próprias permissões).
- **Alternativa C (Implementar WASI 0.2 Component Model Imediatamente)**: Adiada para a Fase 3. Como o ecossistema `wasmtime-wasi` Preview 2 é denso, a Fase 1 priorizará a padronização do `EnginePort` com buffer de memória linear serializado para dados JSON, garantindo paridade semântica imediata.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - Eliminação completa de vetores de injeção de código, SSRF e escape de diretórios.
  - Alinhamento real com o paradigma Object-Capability (Axioma 3).
  - Garantia efetiva de término por esgotamento de combustível e timeout de parede (Axioma 5).
  - Linter seguro (`asl check`) sem efeitos colaterais em produção.
  - Contratos de dados assegurados por validação de JSON Schema em runtime.
- **Negativas / Riscos**:
  - Skills legadas que dependiam de autoridade ambiente ou de permissões auto-concedidas precisarão de políticas de host explícitas para rodar.
  - Pequeno overhead de validação de JSON Schema antes e após cada execução.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Preservada integralmente sem scripts companheiros.
- [x] **Axioma 2 (Zero dependências externas)**: Implementação em Rust puro sem dependência de runtimes do sistema operacional.
- [x] **Axioma 3 (Object-Capability estrito)**: Conquistado de fato com a erradicação de autoridade ambiente e de natives globais.
- [x] **Axioma 4 (Isolamento hexagonal)**: Mantido. Os adaptadores continuam implementando portas sem dependências cruzadas.
- [x] **Axioma 5 (Término determinístico por Fuel)**: Garantido através de contagem de opcodes e traps reais de exaustão.
- [x] **Axioma 6 (Prefixo estático imutável)**: Mantido; métricas de análise calibradas para invariância léxica.
- [x] **Axioma 7 (Limite cognitivo de < 400 linhas)**: Todas as refatorações serão divididas em submódulos ortogonais respeitando o teto de linhas.
