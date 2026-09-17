# 🏛️ Registros de Decisão de Arquitetura (ADRs)

Este diretório contém todas as decisões técnicas e arquiteturais do projeto **Agent Skill Language (ASL)**.

## 📜 Regra do Diretório

> **O que deve conter em `docs/adrs/`**:
> Cada arquivo neste diretório deve apresentar a **proposta detalhada** de uma decisão arquitetural, técnica ou de protocolo, explicitando o contexto do problema, os gaps identificados, o desenho formal da solução, a análise comparativa de alternativas, os trade-offs e a conformidade com os **7 Axiomas do ASL**.

## 📖 Como Criar um Novo ADR

Utilize a skill `asl-adr` (`skills/asl-adr/SKILL.md`):
1. Copie o template canônico de ADR.
2. Nomeie o arquivo como `NNNN-<titulo-em-kebab-case>.md` (ex: `0002-wasm-engine-adapter.md`).
3. Detalhe exaustivamente a proposta de arquitetura.
4. Após aprovação, elabore o plano correspondente em `docs/plans/` via skill `asl-plan`.

## 📑 Índice de Decisões

| ID | Título | Status | Data | Crates Afetadas |
| :--- | :--- | :--- | :--- | :--- |
| **[ADR-0001](./0001-hexagonal-ports-adapters.md)** | Arquitetura Hexagonal (Ports & Adapters) em Micro-Crates | **Aceito** | 2026-09-17 | Workspace completo (`asl-spec`, `asl-core-traits`, adaptadores) |
| **[ADR-0002](./0002-starlark-capability-context-stdlib.md)** | Biblioteca Padrão de Capabilities no Starlark (`ctx.fs`, `ctx.crypto`, `ctx.fuel`) | **Concluído** | 2026-09-17 | `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-cli` |
| **[ADR-0003](./0003-aot-grammar-compiler-gbnf-regex.md)** | Compilador AOT de Gramáticas de Amostragem LLM (GBNF / Regex-CFG) | **Concluído** | 2026-09-17 | `asl-core-traits`, `asl-parser`, `asl-cli` |
| **[ADR-0004](./0004-low-latency-c-abi-ffi.md)** | Interface C-ABI de Baixa Latência In-Process (`libasl` / `asl-ffi`) | **Concluído** | 2026-09-17 | `asl-ffi`, `runtime/Cargo.toml` |
| **[ADR-0005](./0005-wasm-wasi-engine-adapter.md)** | Adaptador de Motor de Execução WebAssembly / WASI (`asl-vm-wasm`) | **Concluído** | 2026-09-17 | `asl-vm-wasm`, `runtime/Cargo.toml` |
| **[ADR-0006](./0006-mcp-http-sse-transport.md)** | Transporte MCP Remoto sobre HTTP / Server-Sent Events (SSE) | **Concluído** | 2026-09-17 | `asl-protocol-http`, `asl-cli`, `runtime/Cargo.toml` |
| **[ADR-0007](./0007-ed25519-skill-signatures.md)** | Assinatura Criptográfica Ed25519 & Cadeia de Custódia de Skills | **Aceito** | 2026-09-17 | `asl-spec`, `asl-security`, `asl-cli` |
