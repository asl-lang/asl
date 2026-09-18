# 🎯 Planos de Implementação (Implementation Plans)

Este diretório contém os planos executáveis de implementação técnica do projeto **Agent Skill Language (ASL)**.

## 📜 Regra do Diretório

> **O que deve conter em `docs/plans/`**:
> Cada arquivo neste diretório deve apresentar as **fases exaustivamente detalhadas** de como deve ser feita a implementação, contendo:
> 1. **Exemplos concretos de código** (Rust, tipos, assinaturas, testes unitários).
> 2. **Fases pequenas e atômicas** (uma unidade de trabalho concisa por vez).
> 3. **Critérios e comandos de validação local** (`cargo test`, `cargo clippy`).
> 4. **Finalização obrigatória de cada fase com Commit e Push na main** (`git commit -m "..." && git push origin main`).

## 📖 Como Criar e Executar um Plano

Utilize a skill `asl-plan` (`skills/asl-plan/SKILL.md`):
1. Crie o arquivo `NNNN-<titulo-em-kebab-case>.md` vinculado a um ADR de `docs/adrs/`.
2. Detalhe cada fase com o código exato planejado (nada de instruções genéricas ou pseudocódigo vago).
3. Execute estritamente uma fase de cada vez.
4. Valide a fase com testes antes de comitar.
5. Finalize cada fase com commit convencional e `git push origin main`.

## 📑 Índice de Planos

| ID | Título | ADR Vinculado | Status | Fases Concluídas |
| :--- | :--- | :--- | :--- | :--- |
| **[PLAN-0001](./0001-modular-hexagonal-runtime.md)** | Construção do Runtime Modular Hexagonal em Rust | [ADR-0001](../adrs/0001-hexagonal-ports-adapters.md) | **Concluído** | 5/5 Fases (100%) |
| **[PLAN-0002](./0002-starlark-capability-context-stdlib.md)** | Implementação da Biblioteca Padrão de Capabilities no Starlark | [ADR-0002](../adrs/0002-starlark-capability-context-stdlib.md) | **Concluído** | 3/3 Fases (100%) |
| **[PLAN-0003](./0003-aot-grammar-compiler-gbnf-regex.md)** | Compilador AOT de Gramáticas de Amostragem LLM (GBNF / Regex-CFG) | [ADR-0003](../adrs/0003-aot-grammar-compiler-gbnf-regex.md) | **Concluído** | 4/4 Fases (100%) |
| **[PLAN-0004](./0004-low-latency-c-abi-ffi.md)** | Interface C-ABI de Baixa Latência In-Process (`libasl` / `asl-ffi`) | [ADR-0004](../adrs/0004-low-latency-c-abi-ffi.md) | **Concluído** | 3/3 Fases (100%) |
| **[PLAN-0005](./0005-wasm-wasi-engine-adapter.md)** | Adaptador de Motor de Execução WebAssembly / WASI (`asl-vm-wasm`) | [ADR-0005](../adrs/0005-wasm-wasi-engine-adapter.md) | **Concluído** | 3/3 Fases (100%) |
| **[PLAN-0006](./0006-mcp-http-sse-transport.md)** | Transporte MCP Remoto sobre HTTP / Server-Sent Events (SSE) | [ADR-0006](../adrs/0006-mcp-http-sse-transport.md) | **Concluído** | 3/3 Fases (100%) |
| **[PLAN-0007](./0007-ed25519-skill-signatures.md)** | Assinatura Criptográfica Ed25519 & Cadeia de Custódia de Skills | [ADR-0007](../adrs/0007-ed25519-skill-signatures.md) | **Concluído** | 4/4 Fases (100%) |
| **[PLAN-0008](./0008-static-prefix-kv-cache-optimizer.md)** | Otimizador de Prefixo Estático e Analisador de KV-Cache | [ADR-0008](../adrs/0008-static-prefix-kv-cache-optimizer.md) | **Concluído** | 4/4 Fases (100%) |
| **[PLAN-0009](./0009-automatic-markdown-shadow-projection.md)** | Projeção Sombra Automática de Markdown (Shadow Projection) | [ADR-0009](../adrs/0009-automatic-markdown-shadow-projection.md) | **Concluído** | 4/4 Fases (100%) |
| **[PLAN-0010](./0010-declarative-semantic-rules-transpiler.md)** | Transpilador Semântico Declarativo para Starlark Hermético (ASL Rules) | [ADR-0010](../adrs/0010-declarative-semantic-rules-transpiler.md) | **Concluído** | 6/6 Fases (100%) |
| **[PLAN-0011](./0011-multi-extension-ai-ecosystem.md)** | Suporte Nativo a Múltiplas Extensões de IA/Agentes/LLM no ASL | [ADR-0011](../adrs/0011-multi-extension-ai-ecosystem.md) | **Concluído** | 5/5 Fases (100%) |
| **[PLAN-0012](./0012-github-pages-documentation-platform.md)** | Plataforma de Documentação no GitHub Pages para ASL | [ADR-0012](../adrs/0012-github-pages-documentation-platform.md) | **Concluído** | 5/5 Fases (100%) |
| **[PLAN-0013](./0013-auditoria-e-correcoes-ecossistema-dual-consumer.md)** | Auditoria Científica e Correções do Ecossistema Dual-Consumer | [ADR-0013](../adrs/0013-auditoria-e-correcoes-ecossistema-dual-consumer.md) | **Concluído** | 5/5 Fases (100%) |
| **[PLAN-0014](./0014-prebuilt-binary-distribution-and-fast-installer.md)** | Distribuição de Binários Pré-Compilados e Instalador Instantâneo | [ADR-0014](../adrs/0014-prebuilt-binary-distribution-and-fast-installer.md) | **Concluído** | 4/4 Fases (100%) |



