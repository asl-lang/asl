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
| **[PLAN-0003](./0003-aot-grammar-compiler-gbnf-regex.md)** | Compilador AOT de Gramáticas de Amostragem LLM (GBNF / Regex-CFG) | [ADR-0003](../adrs/0003-aot-grammar-compiler-gbnf-regex.md) | **Em Progresso** | 0/4 Fases (0%) |
