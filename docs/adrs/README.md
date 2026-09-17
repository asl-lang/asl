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
