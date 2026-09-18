# Plano de Implementação: ADR-0021 - Capacidades OCap Autônomas (ctx.env, ctx.http e Crypto Base64)

## 1. Visão Geral e Objetivos

Implementar capacidades de I/O autocontidas e seguras (OCap) no ecossistema ASL para permitir que skills executem operações de rede e autenticação de forma 100% autônoma, sem exigir a presença de um servidor ou daemon MCP.

---

## 2. Fases de Execução

### Fase 1: Especificação e Schemas (`asl-spec`)
1. Adicionar `EnvCapabilities` com `allow_keys: Vec<String>`.
2. Integrar `env: EnvCapabilities` no `SkillCapabilities`.
3. Adicionar erro `OcapViolation(String)` em `AslError`.

### Fase 2: Contratos de Arquitetura (`asl-core-traits`)
1. Expandir a trait `CapabilityContext`:
   - `env_var(&self, key: &str) -> Result<Option<String>>;`
   - `http_request(&self, method: &str, url: &str, headers: &[(String, String)], body: Option<&str>) -> Result<HttpResponsePayload>;`
   - `base64_encode(&self, data: &str) -> String;`
   - `base64_decode(&self, encoded: &str) -> Result<String>;`
2. Criar a struct `HttpResponsePayload`:
   - `status: u16`, `headers: Vec<(String, String)>`, `body: String`.

### Fase 3: Motor de Segurança e Confinamento (`asl-security`)
1. Implementar validação de domínio em `http_request` contra `capabilities.net.allow_domains`.
2. Implementar checagem de whitelist em `env_var` contra `capabilities.env.allow_keys`.
3. Implementar cliente HTTP síncrono e leve (`ureq` com TLS nativo/rustls).
4. Implementar consumo de fuel proporcional a bytes de payload.
5. Implementar codificação e decodificação Base64 no módulo criptográfico.

### Fase 4: Binding no Motor Starlark (`asl-vm-starlark`)
1. Registrar bridges nativas em `asl_natives`:
   - `asl_native_env_get`
   - `asl_native_http_get` / `asl_native_http_post`
   - `asl_native_crypto_base64_encode` / `asl_native_crypto_base64_decode`
2. Injetar `env`, `http` e `crypto.base64_*` no objeto `asl_ctx`.
3. Prover métodos utilitários no objeto de resposta HTTP (`resp.status`, `resp.text`, `resp.json()`, `resp.headers`).

### Fase 5: Exemplo Canônico Completo (`examples/pr-status-teams.skill`)
1. Criar o arquivo canônico `examples/pr-status-teams.skill` unificando toda a lógica do Jira dev panel GraphQL, verificação de PRs no GitHub e formatação determinística em um único arquivo `.skill`.
2. Demonstrar execução com zero MCP e zero scripts auxiliares de shell.

### Fase 6: Auditoria de Guardrails e Testes
1. Executar `./scripts/guardrail_check.sh` garantindo:
   - Limite estrito de < 450 linhas por arquivo `.rs` (Axioma 7).
   - Clippy com zero advertências.
   - 100% dos testes unitários e de integração passando.
   - 100% de conformidade com idioma inglês no código Rust.

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.
