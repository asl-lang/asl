# Plano de Implementação: Adaptador de Motor de Execução WebAssembly / WASI (`asl-vm-wasm`)

- **ADR Vinculado**: `docs/adrs/0005-wasm-wasi-engine-adapter.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (3/3 Fases - 100%)
- **Meta**: Permitir a execução de blocos de código WebAssembly (WAT ou binário WASM) através da porta hexagonal `EnginePort`, com isolamento de memória e medição determinística de combustível (fuel metering).

---

## Fase 1: [x] Concluída - Configuração da Crate `asl-vm-wasm`

### 1.1 Objetivo da Fase
Criar `runtime/crates/asl-vm-wasm` e registrá-la no workspace Cargo com a dependência `wasmi = "2.0.0"`.

### 1.2 Código a Implementar
No arquivo `runtime/crates/asl-vm-wasm/Cargo.toml`:
```toml
[package]
name = "asl-vm-wasm"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
asl-spec.workspace = true
asl-core-traits.workspace = true
serde_json.workspace = true
wasmi = "2.0.0"
hex = "0.4"
```

### 1.3 Verificação Local
```bash
cargo check -p asl-vm-wasm
```

---

## Fase 2: [x] Concluída - Implementação do `WasmEngine` (`asl-vm-wasm/src/lib.rs`)

### 2.1 Objetivo da Fase
Implementar `WasmEngine` em conformidade com `EnginePort`:
- Configuração de `wasmi::Engine` com `consume_fuel(true)`.
- Suporte a código em formato WAT (`wat::parse_str`) ou binário WASM puro / hex.
- Execução determinística e controle de fuel consumido.

### 2.2 Código a Implementar
```rust
pub struct WasmEngine {
    engine: wasmi::Engine,
}

impl EnginePort for WasmEngine {
    fn name(&self) -> &'static str {
        "wasm-component"
    }

    fn execute(...) -> Result<ExecutionResult>;
}
```

### 2.3 Testes Unitários da Fase
- Teste de função WAT com retorno inteiro (`(module (func (export "add") (param i32 i32) (result i32) ...))`).
- Teste de estouro de combustível / fuel metering.

### 2.4 Verificação Local
```bash
cargo test -p asl-vm-wasm
cargo clippy -p asl-vm-wasm -- -D warnings
```

---

## Fase 3: [x] Concluída - Validação dos Guardrails do Workspace

### 3.1 Objetivo da Fase
Executar todos os guardrails do projeto garantindo integridade e conformidade com os 7 Axiomas.

### 3.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 3.3 Finalização (Commit & Push)
```bash
git add runtime/crates/asl-vm-wasm docs/adrs/ docs/plans/ runtime/Cargo.toml
git commit -m "feat(vm-wasm): implementar adaptador de motor WebAssembly hermetico (Marco 2)"
git push origin main
```
