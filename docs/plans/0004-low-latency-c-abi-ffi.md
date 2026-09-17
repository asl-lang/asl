# Plano de Implementação: Interface C-ABI de Baixa Latência In-Process (`libasl` / `asl-ffi`)

- **ADR Vinculado**: `docs/adrs/0004-low-latency-c-abi-ffi.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (3/3 Fases - 100%)
- **Meta**: Permitir invocação in-process do ASL 3.0 por linguagens de alto nível (Python, Node.js, Go, C/C++) com latência inferior a 35 µs e barreira total de pânico.

---

## Fase 1: [x] Concluída - Cabeçalho C (`libasl.h`) e Configuração da Crate `asl-ffi`

### 1.1 Objetivo da Fase
Criar o cabeçalho C público em `runtime/crates/asl-ffi/include/libasl.h` e configurar a nova crate `asl-ffi` no workspace Cargo com tipos de biblioteca `cdylib`, `staticlib` e `rlib`.

### 1.2 Código a Implementar
No arquivo `runtime/crates/asl-ffi/Cargo.toml`:
```toml
[package]
name = "asl-ffi"
version.workspace = true
edition.workspace = true
license.workspace = true

[lib]
name = "asl"
crate-type = ["cdylib", "staticlib", "rlib"]

[dependencies]
asl-spec.workspace = true
asl-core-traits.workspace = true
asl-parser.workspace = true
asl-security.workspace = true
asl-vm-starlark.workspace = true
serde_json.workspace = true
```

### 1.3 Verificação Local
```bash
cargo check -p asl-ffi
```

---

## Fase 2: [x] Concluída - Implementação da C-ABI e Barreira de Pânico (`asl-ffi/src/lib.rs`)

### 2.1 Objetivo da Fase
Implementar os pontos de entrada exportados com proteção estrita via `catch_unwind`:
- `asl_runtime_init`, `asl_runtime_free`
- `asl_skill_load`, `asl_skill_free`
- `asl_skill_execute`, `asl_exec_result_free`

### 2.2 Código a Implementar
No arquivo `runtime/crates/asl-ffi/src/lib.rs`:
```rust
use asl_core_traits::{EnginePort, ParserPort};
use asl_parser::CommonMarkYamlParser;
use asl_security::ConfinedSecurityContext;
use asl_spec::SkillDocument;
use asl_vm_starlark::StarlarkEngine;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;
```

### 2.3 Testes Unitários da Fase
Testes nativos em Rust simulando a chamada FFI completa via ponteiros brutos (`unsafe`).

### 2.4 Verificação Local
```bash
cargo test -p asl-ffi
cargo clippy -p asl-ffi -- -D warnings
```

---

## Fase 3: [x] Concluída - Validação de Guardrails e Integração Workspace

### 3.1 Objetivo da Fase
Executar todos os guardrails do projeto garantindo limite cognitivo (< 450 linhas), 0 warnings e integridade das skills.

### 3.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 3.3 Finalização (Commit & Push)
```bash
git add runtime/crates/asl-ffi docs/adrs/ docs/plans/ runtime/Cargo.toml
git commit -m "feat(ffi): implementar biblioteca C-ABI de baixa latencia libasl (Marco 1)"
git push origin main
```
