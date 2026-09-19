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
| **[ADR-0007](./0007-ed25519-skill-signatures.md)** | Assinatura Criptográfica Ed25519 & Cadeia de Custódia de Skills | **Concluído** | 2026-09-17 | `asl-spec`, `asl-security`, `asl-cli` |
| **[ADR-0008](./0008-static-prefix-kv-cache-optimizer.md)** | Otimizador de Prefixo Estático e Analisador de KV-Cache para LLMs | **Concluído** | 2026-09-17 | `asl-parser`, `asl-cli` |
| **[ADR-0009](./0009-automatic-markdown-shadow-projection.md)** | Projeção Sombra Automática de Markdown (Shadow Projection) para Adoção de Toque Zero | **Aceito** | 2026-09-17 | `asl-parser`, `asl-cli` |
| **[ADR-0010](./0010-declarative-semantic-rules-transpiler.md)** | Transpilador Semântico Declarativo para Starlark Hermético (ASL Rules Transpiler) | **Concluído** | 2026-09-17 | `asl-core-traits`, `asl-parser`, `asl-vm-starlark`, `asl-cli` |
| **[ADR-0011](./0011-multi-extension-ai-ecosystem.md)** | Tríade Canônica de Extensões do ASL (`.skill`, `.tool`, `.asl`) e Isolamento de Sombra | **Aceito** | 2026-09-17 | `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-cli` |
| **[ADR-0012](./0012-github-pages-documentation-platform.md)** | GitHub Pages Documentation Platform for ASL (Next.js Design Aesthetic) | **Aceito** | 2026-09-17 | `docs/`, `website/`, `.github/workflows/` |
| **[ADR-0013](./0013-auditoria-e-correcoes-ecossistema-dual-consumer.md)** | Auditoria Científica e Correções do Ecossistema Dual-Consumer | **Concluído** | 2026-09-18 | `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-vm-starlark`, `asl-protocol-http`, `asl-cli` |
| **[ADR-0014](./0014-prebuilt-binary-distribution-and-fast-installer.md)** | Distribuição de Binários Pré-Compilados e Instalador Instantâneo de Zero Dependências | **Aceito** | 2026-09-18 | `install.sh`, `scripts/package_release.sh`, `asl-cli` |
| **[ADR-0015](./0015-daemon-universal-multiplataforma-e-parser-tolerante-zero-touch.md)** | Daemon Universal Multiplataforma de FSEvents e Parser Tolerante para Zero-Touch Ingestion | **Aceito** | 2026-09-18 | `asl-parser`, `asl-spec`, `asl-cli`, `install.sh` |
| **[ADR-0016](./0016-guided-interactive-installer-and-opt-in-daemon.md)** | Instalação Interativa Guiada e Ativação Consentida do Daemon de Background | **Aceito** | 2026-09-18 | `install.sh`, `asl-cli` |
| **[ADR-0017](./0017-cli-self-update-and-self-uninstall.md)** | Ciclo de Vida de Auto-Atualização e Auto-Desinstalação na CLI (`asl update` & `asl uninstall`) | **Aceito** | 2026-09-18 | `asl-cli` |
| **[ADR-0018](./0018-minimalist-shadow-markdown-and-self-describing-skill.md)** | Projeção Sombra Minimalista e Auto-Descoberta Segura de Execução no `.skill` | **Aceito** | 2026-09-18 | `asl-parser`, `asl-spec`, `asl-cli` |
| **[ADR-0019](./0019-self-teaching-cli-learning-system.md)** | Subsistema de Auto-Instrução na CLI e Descoberta para IAs (`asl docs` / `asl learn`) | **Aceito** | 2026-09-18 | `asl-cli` |
| **[ADR-0020](./0020-pure-asl-semantic-language-architecture.md)** | Arquitetura Pura da Linguagem Semântica ASL e Tag Universal Única (```asl) | **Aceito** | 2026-09-18 | `asl-parser`, `asl-spec`, `asl-cli`, `website/` |
| **[ADR-0021](./0021-sandboxed-autonomous-io-capabilities.md)** | Capacidades OCap Autônomas (`ctx.env`, `ctx.http` e Base64) para Skills Zero-MCP | **Proposto** | 2026-09-18 | `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-cli` |
| **[ADR-0022](./0022-unified-ocap-capabilities-and-developer-ergonomics.md)** | Ergonomia de Linguagem, Confinamento OCap Unificado e Diagnósticos Transparentes | **Aceito** | 2026-09-18 | `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-parser`, `asl-cli` |
| **[ADR-0023](./0023-spec-driven-documentation-ssot-cli-website.md)** | Spec-Driven Documentation (SDD) e Single Source of Truth (SSOT) para CLI e Website | **Aceito** | 2026-09-18 | `docs/spec/`, `asl-cli`, `website/`, `scripts/` |
| **[ADR-0024](./0024-runtime-security-hardening-ocap-and-semantic-alignment.md)** | Endurecimento de Segurança do Runtime, Confinamento OCap Estrito e Alinhamento Semântico | **Aceito** | 2026-09-19 | `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-vm-wasm`, `asl-parser`, `asl-protocol-mcp`, `asl-cli`, `asl-ffi` |
