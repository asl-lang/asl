# ADR-0001: Arquitetura Hexagonal (Ports & Adapters) em Micro-Crates para Runtime ASL

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Arquitetura Central ASL 3.0
- **Crates Afetadas**: `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-security`, `asl-vm-starlark`, `asl-protocol-mcp`, `asl-cli`

---

## 1. Contexto e Declaração do Problema

O projeto **Agent Skill Language (ASL)** visa executar habilidades determinísticas unificadas para IAs (`.skill`) eliminando scripts soltos (`SKILL.md + scripts/`) e interpretadores pesados (Python/Node). 

No entanto, sistemas de runtime tradicionais frequentemente sofrem de:
1. **Acoplamento Monolítico**: Dificuldade de substituir motores de execução (ex: trocar Starlark por Wasmtime ou QuickJS) sem reescrever o parser ou o transporte.
2. **Dependências Circulares**: Dificuldade de agentes autônomos de IA alterarem tipos básicos sem quebrar pipelines inteiros.
3. **Sobrecarga Cognitiva de Contexto**: Arquivos gigantes (> 600 linhas) que excedem as janelas de atenção das LLMs, provocando alucinações e erros de edição.

---

## 2. Proposta Detalhada da Decisão

Decidimos adotar a **Arquitetura Hexagonal (Ports & Adapters)** dividida em um **Workspace Cargo de Micro-Crates Ortogonais**:

```
                              ┌───────────────────────┐
                              │        asl-cli        │ (Aplicações / Entrypoint)
                              └───────────┬───────────┘
                                          │ [Dependency Injection]
                ┌─────────────────────────┼─────────────────────────┐
                ▼                         ▼                         ▼
      ┌───────────────────┐     ┌───────────────────┐     ┌───────────────────┐
      │ asl-protocol-mcp  │     │  asl-vm-starlark  │     │   asl-security    │ (Adaptadores)
      │ (Transporte MCP)  │     │ (Engine Starlark) │     │ (Sandbox & Ocap)  │
      └─────────┬─────────┘     └─────────┬─────────┘     └─────────┬─────────┘
                │                         │                         │
                └─────────────────────────┼─────────────────────────┘
                                          │ [Implements]
                                          ▼
                                ┌───────────────────┐
                                │  asl-core-traits  │ (Portas / Interfaces Puras)
                                └─────────┬─────────┘
                                          │ [Uses]
                                          ▼
                                ┌───────────────────┐
                                │     asl-spec      │ (Domínio Puro / Zero I/O)
                                └───────────────────┘
```

### 2.1 Camadas e Responsabilidades
1. **`asl-spec` [Nível 0]**: Contém os tipos puros de domínio (`SkillManifest`, `SkillDocument`, `Limits`, `ExecutionResult`, `AslError`). Zero I/O, zero tokio, zero dependências de SO.
2. **`asl-core-traits` [Nível 1]**: Contém apenas contratos abstratos (Traits):
   - `EnginePort`: Avaliação de lógica determinística.
   - `ParserPort`: Parsing de arquivo `.skill`.
   - `CapabilityContext`: Injeção de recursos seguros atenuados (ocap).
   - `GrammarCompilerPort`: Compilação de esquemas em GBNF/CFG.
3. **Adaptadores [Nível 2]**: Micro-crates que implementam os traits sem conhecer outros adaptadores:
   - `asl-parser`: Parser híbrido CommonMark/YAML e compilador GBNF.
   - `asl-security`: Implementação de confinamento e mock em memória (`MockSecurityContext`).
   - `asl-vm-starlark`: Motor determinístico hermético com Fuel Metering.
   - `asl-protocol-mcp`: Servidor Model Context Protocol sobre `stdio`.
4. **Entrypoints [Nível 3]**:
   - `asl-cli`: Faz a injeção de dependências conectando os adaptadores concretos às portas.

---

## 3. Alternativas Consideradas

- **Alternativa A: Crate Monolítica Única (`asl-runtime`)**:
  - *Descarte*: Um único crate com módulos internos (`mod parser`, `mod engine`) não impede que um desenvolvedor ou IA importe estruturas internas de forma espaguete. Micro-crates forçam o compilador a auditar a fronteira de dependência no `Cargo.toml`.
- **Alternativa B: Dinâmica via Plugins Compartilhados (`.so` / `dlopen`)**:
  - *Descarte*: Viola o Axioma 2 (hermeticidade e zero dependências de C-ABI dinâmica no host), adicionando complexidade frágil de linkagem.

---

## 4. Consequências e Trade-offs

### Positivas
- **Substituibilidade Total**: Qualquer motor (Starlark, WASM, Lua) pode ser plugado bastando implementar `EnginePort`.
- **Testes em Microssegundos**: Mocks puros em memória (`MockSecurityContext`, `MockEngine`) rodam sem tocar no disco.
- **Desenvolvimento Concorrente de IAs**: Múltiplos agentes podem trabalhar em crates distintas sem conflito de merge.
- **Arquivos Menores**: Nenhuma micro-crate excede o limite cognitivo de 400 linhas.

### Negativas / Custos
- Necessidade de manter múltiplos `Cargo.toml` no workspace.
- Injeção de dependência explícita necessária no `asl-cli`.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: Preservada; a CLI consome `.skill` único.
- [x] **Axioma 2 (Zero Dependências)**: Todas as micro-crates são $100\%$ Rust puro.
- [x] **Axioma 3 (Confinamento ocap)**: Isolado na porta `CapabilityContext` e adaptador `asl-security`.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Adaptadores não se importam mutuamente (validado via teste automatizado).
- [x] **Axioma 5 (Término por Fuel)**: Porta `EnginePort` exige fuel check.
- [x] **Axioma 6 (Prefixo Estático)**: Parser isola seções semânticas e calcula digest estável.
- [x] **Axioma 7 (Limite Cognitivo)**: Todos os arquivos têm < 300 linhas de código.
