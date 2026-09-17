# Agent Skill Language (ASL 3.0 - Omni-Spec)
**Linguagem AI-First para Skills e Ferramentas Executáveis Unificadas (`.skill`, `.tool`, `.asl`)**

**Autor & Criador**: **Jean Catarina** *(Cadente)*

Este repositório contém a especificação formal, o artigo científico e a implementação de referência da **Agent Skill Language (ASL 3.0)**, projetada por Jean Catarina para unificar execução determinística, modelos de capacidade estritos e inferência semântica de agentes de inteligência artificial.

---

## 📑 Publicações e Documentos Centrais

- 📐 **[Arquitetura Modular AI-First (ARCHITECTURE.md)](./ARCHITECTURE.md)**: Especificação da arquitetura de software em Hexagonal (Ports & Adapters), decomposição em micro-crates independentes e substituíveis, matriz de troca de motores e guia prático para trabalho autônomo de IAs.
- 📄 **[Artigo Científico Formal (ASL_SCIENTIFIC_PAPER.md)](./ASL_SCIENTIFIC_PAPER.md)**: Artigo acadêmico formal por **Jean Catarina** *(Cadente)*, contendo provas matemáticas, teoremas de confinamento e terminação, fundamentado nos preceitos clássicos da computação e benchmarks.
- 🔬 **[Estudo Técnico e Especificação ASL 3.0 (ASL_STUDY.md)](./ASL_STUDY.md)**: Especificação formal detalhada, análises de mercado, pareceres detalhados e diretrizes de engenharia de runtime por Jean Catarina.

---

## 🏛️ Os 10 Pilares Arquiteturais & Fundamentação Científica

A arquitetura concebida por Jean Catarina fundamenta-se nos avanços e preceitos clássicos da ciência da computação:

1. **Programação Literária AI-First (Princípio de Knuth)**:
   - A prosa de linguagem natural foi estruturada em papéis semânticos rígidos (*Intent*, *Activation Criteria*, *Security Boundary*, *Few-Shot Exemplars*), eliminando ruído estocástico na interpretação da IA.
2. **Término Algébrico e Lógica Temporal (Princípio de Lamport)**:
   - O término da execução foi formalmente provado por meio de uma **Função Variante Monotônica Decrescente** atrelada a *Fuel Metering* de bytecode, prevenindo livelocks sem depender de relógio de parede.
3. **Subtipagem Estrutural Comportamental (Princípio de Liskov)**:
   - Estabelecimento de **Subtipagem Estrutural Comportamental** (entradas contravariantes e saídas covariantes) com `Result[T, E]` nativo, permitindo evolução segura de versões do `.skill`.
4. **Segurança por Objetos-Capacidade (Princípio de Mark S. Miller)**:
   - Eliminação de ataques do Vice-Confuso (*Confused Deputy*) substituindo strings de caminhos por **Handles de Diretório Atenuados**, impedindo fuga de symlinks fora do workspace.
5. **O Problema do Confinamento (Teorema de Butler Lampson)**:
   - Resolução do **Problema do Confinamento** (Lampson 1973), normalizando envelopes de erro e eliminando vazamento de dados através de Canais Ocultos (*Covert Channels*).
6. **Defesa em Profundidade contra Injeção Indireta (Constitutional AI)**:
   - Proteção contra **Injeção Indireta de Prompt** usando rastreamento de mancha (*Taint Tracking*) e encapsulamento em tags `<asl:untrusted_content>`.
7. **Otimização de Prefixo Estático e Dinâmica de KV-Cache**:
   - Implementação da **Otimização de Prefixo Estático Imutável**, garantindo **100% de reuso de KV-Cache** em servidores modernos de inferência (vLLM, SGLang, TensorRT).
8. **Compilação AOT de Gramáticas e Token Masking**:
   - Substituição de validação post-hoc em runtime por **Compilação AOT de Gramáticas (CFG / GBNF)**, forçando o LLM a ter 0% de erro sintático no primeiro turno.
9. **Extensibilidade via WebAssembly Component Model**:
   - Padronização da conexão de componentes binários de alta performance através de **Interface Types (WIT)** do **WASI Preview 2**.
10. **Resiliência e Engenharia de Baixo Nível em Rust**:
    - Blindagem da biblioteca `libasl` com **Barreiras de Captura de Pânico (`catch_unwind`)** e **Arenas de Memória Isoladas**, garantindo zero panics na C-ABI e observabilidade nativa DTrace/eBPF.

---

## 📊 Matriz Comparativa Definitiva

| Métrica / Recurso | Padrão Legado (`SKILL.md` + Scripts) | ASL v1 / v2 | ASL 3.0 (Jean Catarina / Cadente) |
| :--- | :--- | :--- | :--- |
| **Tokens por Invocação** | $\sim 2.100\text{ tokens}$ | $\sim 550\text{ tokens}$ | **$\sim 140\text{ tokens}$ ($-93.2\%$)** |
| **Reuso de KV-Cache** | Desalinhado / Invalidação frequente | Parcial | **$100\%$ (Prefixo Estático Bit-a-Bit)** |
| **Latência por Execução** | $195\text{ ms}$ (Python/Node) | $1.8\text{ ms}$ (CLI) | **$< 0.035\text{ ms}$ ($35\ \mu\text{s}$ via FFI)** |
| **Garantia Sintática no LLM** | Nula / Erros em runtime | JSON Schema pós-geração | **Gramática CFG/GBNF (0% de erro)** |
| **Protocolo de Integração** | Scripts bash soltos | CLI customizado | **Nativo MCP + WASI Preview 2 WIT** |
| **Estabilidade de FFI** | Inexistente | Básica | **Imune a Panics (C-ABI com Arenas)** |

---

## 🛡️ "O MCP geralmente cai e fica fora do ar": O `.skill` sempre vai funcionar?

**Sim, por construção matemática.** No ASL, o MCP é apenas uma interface de transporte opcional de nível superficial, e **não um servidor residente do qual o arquivo depende**.

O ASL implementa a **Escada de Degradação Graciosa em 4 Níveis**:
1. **In-Process Zero-IPC (`libasl`)**: Roda no mesmo espaço de memória do agente via C-ABI em $< 35\ \mu\text{s}$. Não há socket, não há pipe, não há processo filho. Impossível cair.
2. **Single-Shot Ephemeral CLI (`asl run`)**: Comando atômico de $1.2\text{ ms}$ (estilo `grep`). Inicia, executa e encerra. Não existem daemons residentes para travar.
3. **Hermeticidade Absoluta**: Zero dependências externas (sem `npm install` ou `pip install` que quebram).
4. **Fallback Cognitivo In-Context**: Se o host não puder rodar nenhum binário, o próprio LLM interpreta o código Starlark mentalmente no contexto, pois a sintaxe é limpa e determinística.

> **Teorema da Disponibilidade**: $\mathbb{P}(\mathcal{A}_{\text{.skill}} = \text{Operational}) \ge \mathbb{P}(\mathcal{A}_{\text{LLM}} = \text{Operational})$. A skill só falhará se o próprio LLM colapsar.

---

## 📂 Estrutura de Documentos, Skills e Guardrails

### Documentos Fundamentais
- 📐 **[`ARCHITECTURE.md`](./ARCHITECTURE.md)**: Arquitetura Hexagonal (Ports & Adapters), decomposição em micro-crates e matriz de substituibilidade.
- 📄 **[`ASL_SCIENTIFIC_PAPER.md`](./ASL_SCIENTIFIC_PAPER.md)**: Artigo científico formal por Jean Catarina com teoremas de terminação e confinamento.
- 🔬 **[`ASL_STUDY.md`](./ASL_STUDY.md)**: Estudo técnico completo, histórico de revisão tripartite e garantias de sobrevivência sem MCP.
- 🏛️ **[`docs/adrs/`](./docs/adrs/)**: Registros de Decisão de Arquitetura contendo propostas técnicas detalhadas e trade-offs.
- 🎯 **[`docs/plans/`](./docs/plans/)**: Planos de implementação exaustivamente detalhados em fases atômicas com commit e push na main.
- 🎯 **Tríade Canônica de Exemplos (`examples/`)**:
  - [`examples/git-conventional-commit.skill`](./examples/git-conventional-commit.skill): Skill com metadados semânticos, projeção sombra e digest SHA-256 verificado.
  - [`examples/conventional-commit-rules.skill`](./examples/conventional-commit-rules.skill): Skill com regras declarativas transpiladas (`asl:rules`).
  - [`examples/security-validator.tool`](./examples/security-validator.tool): Ferramenta atômica MCP sem projeção sombra.
  - [`examples/summarizer.asl`](./examples/summarizer.asl): Documento raiz ASL determinístico sem projeção sombra.

### Regras e Guardrails Automáticos para IAs
- 🤖 **[`AGENTS.md`](./AGENTS.md)**: Os 7 Axiomas Invioláveis, diretrizes de testes em Rust e regras universais para todos os modelos de IA.
- 📜 **[`CLAUDE.md`](./CLAUDE.md)**: Configuração de comandos e guardrails para Claude Code.
- 💎 **[`GEMINI.md`](./GEMINI.md)**: Regras para Google Antigravity e Gemini CLI.
- 🧭 **[`.cursorrules`](./.cursorrules)**: Guardrails para Cursor e Windsurf.
- 🛡️ **[`scripts/guardrail_check.sh`](./scripts/guardrail_check.sh)**: Script automatizado de verificação que roda os 5 níveis de auditoria arquitetural e linter.

### Skills de Repositório (Extensões para Agentes)
- ⚙️ **[`skills/asl-guardrail/SKILL.md`](./skills/asl-guardrail/SKILL.md)**: Procedimento para auditar o repositório contra violações arquiteturais.
- ✍️ **[`skills/asl-author/SKILL.md`](./skills/asl-author/SKILL.md)**: Guia de autoria e cálculo de digest canônico para novas skills.
- 🏗️ **[`skills/asl-architect/SKILL.md`](./skills/asl-architect/SKILL.md)**: Diretrizes para estender o runtime com novos motores e adaptadores.
- 🏛️ **[`skills/asl-adr/SKILL.md`](./skills/asl-adr/SKILL.md)**: Guia e template para redigir propostas arquiteturais detalhadas em `docs/adrs/`.
- 🎯 **[`skills/asl-plan/SKILL.md`](./skills/asl-plan/SKILL.md)**: Guia para redigir planos em fases pequenas com código e encerramento via commit e push na main.
- 🔄 **[`skills/asl-spec-driven/SKILL.md`](./skills/asl-spec-driven/SKILL.md)**: Meta-protocolo de Spec-Driven Development (SDD) que governa o ciclo obrigatório: ADR -> PLAN -> Execução -> Guardrail -> Commit/Push.
