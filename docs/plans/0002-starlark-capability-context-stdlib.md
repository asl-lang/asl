# PLAN-0002: Implementação da Biblioteca Padrão de Capabilities no Starlark (ctx.fs, ctx.crypto, ctx.fuel)

- **ADR Vinculado**: [`docs/adrs/0002-starlark-capability-context-stdlib.md`](../adrs/0002-starlark-capability-context-stdlib.md)
- **Status**: Concluído
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Expor primitivas atenuadas de filesystem, criptografia e medição de fuel dentro do objeto `ctx` para scripts `.skill` em Starlark.

---

## Fase 1: Extensão de `asl-core-traits` e `asl-security` [x] Concluída

### 1.1 Objetivo
Estender a interface `CapabilityContext` com `sha256` e `fuel_consumed`, e implementar essas capacidades em `MockSecurityContext` e `ConfinedSecurityContext`.

### 1.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-core-traits/src/lib.rs`:
```rust
pub trait CapabilityContext: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Option<String>>;
    fn sha256(&self, data: &str) -> String;
    fn check_fuel(&self) -> Result<u64>;
    fn fuel_consumed(&self) -> u64;
}
```

Em `runtime/crates/asl-security/src/lib.rs`:
```rust
impl CapabilityContext for ConfinedSecurityContext {
    ...
    fn sha256(&self, data: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn fuel_consumed(&self) -> u64 {
        self.fuel_counter.load(Ordering::Relaxed)
    }
}
```

### 1.3 Verificação Local
```bash
cargo check -p asl-core-traits -p asl-security
cargo test -p asl-security
cargo clippy -p asl-security -- -D warnings
```

### 1.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-core-traits/ runtime/crates/asl-security/ docs/plans/ docs/adrs/
git commit -m "feat(security): estender CapabilityContext com sha256 e metricas de fuel"
git push origin main 2>/dev/null || true
```

---

## Fase 2: Injeção de `ctx.fs`, `ctx.crypto` e `ctx.fuel` em `asl-vm-starlark` [x] Concluída

### 2.1 Objetivo
Construir o ambiente de avaliação Starlark injetando um `ctx` enriquecido com métodos executáveis que delegam para `CapabilityContext`.

### 2.2 Código a Implementar (Exemplo Exaustivo)
Em `runtime/crates/asl-vm-starlark/src/lib.rs`:
```rust
// Wrapper Starlark para injeção de primitivas nativas
let fs_read_impl = ...; // Registra função nativa Starlark 'asl_native_fs_read'
let crypto_sha256_impl = ...; // Registra função nativa 'asl_native_sha256'
let fuel_consumed_impl = ...; // Registra função nativa 'asl_native_fuel'

// Construção do objeto ctx no Starlark:
// ctx = struct(
//     fs = struct(read = _asl_fs_read),
//     crypto = struct(sha256 = _asl_sha256),
//     fuel = struct(consumed = _asl_fuel_consumed, remaining = _asl_fuel_remaining),
// )
```

### 2.3 Verificação Local
```bash
cargo test -p asl-vm-starlark
cargo clippy -p asl-vm-starlark -- -D warnings
```

### 2.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-vm-starlark/
git commit -m "feat(vm-starlark): injetar stdlib de capabilities ctx.fs, ctx.crypto e ctx.fuel"
git push origin main 2>/dev/null || true
```

---

## Fase 3: Integração End-to-End, Exemplo Real e Guardrail Final [x] Concluída

### 3.1 Objetivo
Adicionar teste de integração no `asl-cli` exercitando `ctx.fs.read()` e `ctx.crypto.sha256()`, validar o repositório completo com `./scripts/guardrail_check.sh` e atualizar a documentação.

### 3.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 3.3 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-cli/ docs/
git commit -m "feat(cli): integrar e validar stdlib de capabilities em testes end-to-end"
git push origin main 2>/dev/null || true
```
