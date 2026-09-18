# ADR-0023: Arquitetura Spec-Driven Documentation (SDD) e Single Source of Truth (SSOT) para CLI e Website

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina (Cadente) & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `docs/spec/`, `asl-cli` (`docs_cmds.rs`, `main.rs`), `website/` (`CliCommandsSection.tsx`), `scripts/`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências no Runtime), Axioma 4 (Desacoplamento Hexagonal), Axioma 6 (Prefixo Estático e KV-Cache), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Declaração do Problema

Com a evolução rápida do ASL 3.0, a introdução de novos subcomandos (`sync`, `watch`, `update`, `uninstall`, `daemon`, `docs`, `template`) e o lançamento do portal de documentação oficial em Next.js (`website/`), surgiu um problema clássico de dispersão de conhecimento:

1. **Drift entre CLI e Website**: O portal web continha componentes estáticos com apenas um subconjunto de comandos legados, enquanto a CLI em Rust definia 19 subcomandos completos.
2. **Duplicação e Manutenção Manual**: Toda vez que uma flag ou parâmetro era adicionado à CLI no Clap (`main.rs`), a documentação web ficava desatualizada até que um desenvolvedor editasse manualmente múltiplos arquivos React.
3. **Strings Hardcoded no Binário**: O arquivo `runtime/crates/asl-cli/src/docs_cmds.rs` acumulava centenas de linhas de texto estático, aumentando o custo cognitivo de manutenção e aproximando-se do limite de 450 linhas do Axioma 7.
4. **Agentes de IA e Descoberta de APIs**: Agentes autônomos de IA dependem de esquemas determinísticos e densos para executar comandos sem alucinações de parâmetros ou flags inexistentes.

Portanto, tornou-se imperativo adotar uma arquitetura de **Spec-Driven Documentation (SDD)** baseada em uma **Fonte Única de Verdade (Single Source of Truth - SSOT)**, onde tanto a CLI quanto o Website consumam os mesmos dados declarativos e testes de contrato impeçam qualquer divergência.

---

## 2. Proposta Detalhada da Decisão

Estabelecemos o diretório canônico `docs/spec/` como a única autoridade definidora de comandos, flags, tópicos conceituais, exemplos e primers para IA.

```
                              ┌──────────────────────────────────────────────┐
                              │     SINGLE SOURCE OF TRUTH (SSOT)            │
                              │     docs/spec/commands.json / topics.json    │
                              │     (Auditados por cli-command.schema.json)  │
                              └──────────────────────┬───────────────────────┘
                                                     │
                                         [SDD Schema Validator]
                                         scripts/sync_docs_spec.py
                                                     │
                     ┌───────────────────────────────┴───────────────────────────────┐
                     ▼                                                               ▼
    ┌─────────────────────────────────┐                             ┌─────────────────────────────────┐
    │       CLI BUILD & RUNTIME       │                             │      WEBSITE REACT / NEXT.JS    │
    │  (asl-cli / docs_cmds.rs)       │                             │  (website/src/app/docs)         │
    ├─────────────────────────────────┤                             ├─────────────────────────────────┤
    │ • Embutido via include_str!     │                             │ • Ingestão de cli-spec.json     │
    │ • Zero dependências I/O runtime │                             │ • Renderização dinâmica React   │
    │ • `asl docs <topic>` tipado     │                             │ • Paleta de busca instantânea ⌘K │
    │ • `asl docs --ai` (alta densidade)│                           │ • Badges de flags e exemplos    │
    │ • `asl docs --json` estruturado │                             │ • Zero comandos hardcoded       │
    └─────────────────────────────────┘                             └─────────────────────────────────┘
                     ▲                                                               ▲
                     └───────────────────────────────┬───────────────────────────────┘
                                                     │
                                      ┌─────────────────────────────┐
                                      │   ZERO-DRIFT CI GUARDRAIL   │
                                      │   scripts/guardrail_check   │
                                      │   test_docs_ssot_drift.rs   │
                                      └─────────────────────────────┘
```

### 2.1. O Contrato Canônico (`docs/spec/commands.json`)
Cada subcomando é declarado com:
- `id` e `name`: Identificadores canônicos e aliases.
- `category`: Agrupamento funcional (`execution`, `verification`, `server`, `compiler`, `crypto`, `lifecycle`, `docs`, `daemon`).
- `summary` e `description`: Resumo conciso e descrição técnica.
- `arguments`: Argumentos posicionais com tipos e restrições.
- `flags`: Flags opcionais com formas curta (`-i`), longa (`--input`), valores padrão e explicações.
- `examples`: Comandos executáveis prontos para cópia e teste.
- `ai_primer`: Padrão de uso otimizado e estimativa de tokens.

### 2.2. Consumo na CLI (`asl-cli`)
- O binário compila o manifesto em tempo de build usando `include_str!`.
- Não há chamadas de rede ou leituras de disco no runtime.
- O `docs_cmds.rs` utiliza desserialização via `serde_json` (já presente no workspace) para responder dinamicamente a `asl docs <topic>`, `asl docs --ai` e `asl docs --json`.

### 2.3. Consumo no Portal Web (`website/`)
- O script `scripts/sync_docs_spec.py` exporta o manifesto validado para `website/src/data/cli-spec.json`.
- O componente `CliCommandsSection.tsx` renderiza todos os subcomandos, badges de flags e exemplos diretamente do JSON gerado, eliminando completamente arrays estáticos.

### 2.4. Blindagem Contra Desatualização (Zero-Drift Invariant)
Para garantir matematicamente que nenhum componente fique desatualizado:
1. **Teste de Reflexão no Clap**: Um teste de integração em Rust (`test_docs_ssot_drift.rs`) percorre a árvore de comandos gerada pelo Clap (`Cli::command()`) e compara com os comandos e flags da especificação. Se um desenvolvedor adicionar uma flag no Rust sem atualizar o SSOT, o teste falha.
2. **Auditoria no Guardrail**: O `./scripts/guardrail_check.sh` executa a validação do SSOT e o teste de drift em toda verificação pré-commit.

---

## 3. Alternativas Consideradas e Trade-offs

| Critério | Alternativa A: Código Manual Duplicado (Status Quo) | Alternativa B: Exportar do Clap via Macro Rust | Alternativa C: SSOT Declarativo em `docs/spec/` (Escolhida) |
| :--- | :---: | :---: | :---: |
| **Risco de Desatualização** | Altíssimo (drift constante) | Baixo | **Zero (auditado por teste de contrato)** |
| **Ergonomia para Agentes de IA** | Ruim (agente precisa ler TSX e Rust) | Média (depende de compilação Rust) | **Ótima (especificação JSON/YAML direta)** |
| **Portabilidade para Ferramentas Externas** | Nula | Limitada | **Universal (IDE plugins, manpages, web)** |
| **Zero Dependências no Runtime (Axioma 2)** | Sim | Sim | **Sim (embutido via include_str!)** |

---

## 4. Consequências e Benefícios

### Positivas
- **Garantia de Zero-Drift**: CLI e website sempre apresentam as mesmas flags, tipos e exemplos.
- **Redução Drástica de Linhas em Rust**: O `docs_cmds.rs` reduz de 430 para menos de 180 linhas, respeitando o Axioma 7 com folga.
- **Ecossistema AI-First**: Agentes de IA podem ler diretamente `docs/spec/commands.json` para saber todas as ferramentas disponíveis sem alucinação.

### Negativas
- Adiciona um passo de sincronização durante o ciclo de build e guardrail (`scripts/sync_docs_spec.py`), mitigado por ser executado em milissegundos via biblioteca padrão Python.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: Cada comando na spec é atômico e documenta suas entradas/saídas.
- [x] **Axioma 2 (Zero Dependências no Runtime)**: O compilador Rust embute a especificação em tempo de build (`include_str!`).
- [x] **Axioma 3 (Segurança OCap)**: Flags de capacidades (`--allowed-root`) são documentadas formalmente.
- [x] **Axioma 4 (Desacoplamento Hexagonal)**: A camada de especificação permanece pura e desacoplada dos adaptadores de exibição.
- [x] **Axioma 5 (Término Determinístico)**: Comandos de documentação executam em tempo $O(1)$.
- [x] **Axioma 6 (Prefixo Estático e KV-Cache)**: O `--ai` primer mantém formato estático imutável de alta densidade.
- [x] **Axioma 7 (Limite Cognitivo < 450 linhas)**: Permite manter `docs_cmds.rs` e componentes do website abaixo de 200 linhas cada.
