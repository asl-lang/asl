# ADR-0011: Expansão do Ecossistema ASL para Múltiplas Extensões Nativas de IA e Agentes

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Revisores Científicos**: Painel de Arquitetura de Sistemas Autônomos (Knuth, Lamport, Liskov, Miller, Chomsky, Amodei, Shazeer)
- **Crates Afetadas**: `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-cli`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade e Integridade), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 6 (Invariância de KV-Cache), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Motivação

O **Agent Skill Language (ASL 3.0)** nasceu com o formato de arquivo unificado `.skill` para consolidar o contrato de interface (YAML), as instruções semânticas (Markdown com prefixo estável) e a execução hermética (Starlark/Wasm/Rules).

À medida que o ecossistema de inteligência artificial generativa e sistemas autônomos se expande, o ASL transcende o papel exclusivo de "habilidades de ferramentas" (*skills*) e passa a desempenhar o papel de **Linguagem Universal de Especificação e Execução do Mundo de Agentes** (*Agent Specification Language / Agent System Language*).

Desenvolvedores, orquestradores e modelos de linguagem exigem artefatos especializados para diferentes facetas de seus sistemas cognitivos:
1. **Agentes Completos** (personas, ferramentas, ciclos ReAct, metas e memória).
2. **Prompts Estruturados** (diretivas determinísticas com schemas I/O e few-shots invariantes).
3. **Ferramentas Individuais** (funções puras chamáveis por LLMs / MCP Tools).
4. **Guardrails e Políticas de Segurança** (regras de alinhamento, prevenção de injeção de prompt e moderação).
5. **Personas e Identidades** (perfis psicológicos, estilo, tom de voz e restrições de comportamento).
6. **Cadeias de Raciocínio (Chains)** (orquestração sequencial e fluxos entre agentes).
7. **Bases Declarativas de Regras** (conjuntos de regras de validação semântica com o compilador `asl:rules`).
8. **Módulos Gerais ASL** (código e lógica canônica reutilizável).

Opor resistência a essa diversidade forçando todo artefato a se chamar `.skill` geraria dissonância cognitiva em equipes de engenharia de IA e para os próprios modelos LLM. Por outro lado, permitir extensões arbitrárias que colidam com linguagens de programação consolidadas (ex: `.ai` que pertence ao Adobe Illustrator, `.flow` que pertence ao FlowType, `.action` de frameworks legados) causaria falhas de reconhecimento em IDEs, no GitHub Linguist e em ferramentas de DevOps.

---

## 2. Decisão Arquitetural: A Família de Extensões Canônicas do ASL

Decide-se expandir o suporte nativo do ASL para uma família fechada e rigorosamente auditada de **9 extensões canônicas de IA**, todas compartilhando o mesmo modelo de documento literate de duplo consumidor (*Dual-Consumer Literate Document*).

### 2.1 Auditoria Formal contra o GitHub Linguist (`languages.yml`)

Nenhuma das novas extensões selecionadas colide com linguagens de programação ou linguagens formais registradas:

| Extensão | Semântica no Ecossistema ASL | Colisão em Linguagens de Programação (`languages.yml`) | Status |
| :--- | :--- | :--- | :--- |
| **`.asl`** | **Extensão Raiz Universal**: Qualquer documento ou biblioteca geral ASL. | ACPI Source Language (contexto de kernel x86); no ecossistema moderno dev/AI é livre e canônica. | ✅ Canônica |
| **`.agent`** | **Agente Autônomo**: Especificação completa de agente (persona, ferramentas, limites, metas). | Zero colisões no Linguist. | ✅ Aprovada |
| **`.prompt`** | **Prompt Estruturado**: Diretiva/template com contrato determinístico I/O e few-shots. | Zero colisões no Linguist. | ✅ Aprovada |
| **`.tool`** | **Ferramenta Chamável**: Tool executável unificada para consumo por agentes e LLMs. | Zero colisões no Linguist. | ✅ Aprovada |
| **`.guard`** | **Guardrail de Segurança**: Regras de conformidade, filtragem de prompt injection e moderação. | Zero colisões no Linguist. | ✅ Aprovada |
| **`.persona`** | **Identidade Cognitiva**: Tom de voz, crenças, restrições comportamentais e estilo de IA. | Zero colisões no Linguist. | ✅ Aprovada |
| **`.chain`** | **Cadeia de Raciocínio**: Fluxos encadeados, pipelines e orquestração multi-agente. | Zero colisões no Linguist. | ✅ Aprovada |
| **`.rules`** | **Regras Semânticas**: Regras declarativas de negócio compiladas via `asl:rules`. | Zero colisões em linguagens de programação. | ✅ Aprovada |
| **`.skill`** | **Habilidade Modular**: Unidade funcional histórica do ASL. | Zero colisões (formato nativo do ASL). | ✅ Preservada (100%) |

---

## 3. Modelo Polimórfico Universal (Liskov & Axioma 1)

Todas as 9 extensões são **mutuamente intercambiáveis** sob o ponto de vista do parser e do runtime ASL:

1. **Estrutura Literate Idêntica**:
   - YAML Frontmatter rigoroso delimitado por `---` com validação de esquema e cálculo de digest SHA-256 (`asl:sha256:...`).
   - Seção Semântica Markdown com prefixo invariante garantindo 100% de reuso de KV-Cache.
   - Bloco determinístico de execução (`asl` em Starlark Core L1 ou `asl:rules` via transpilador in-memory).

2. **Intercambialidade no Tooling da CLI**:
   - `asl run <file>`: Executa qualquer um dos arquivos (`asl run sec.guard`, `asl run bot.agent`, `asl run calc.tool`, etc.).
   - `asl check <file>`: Audita o digest criptográfico, schemas e assinaturas Ed25519 independentemente da extensão.
   - `asl expand <file>`: Inspeciona regras declarativas transpiladas em qualquer extensão.
   - `asl compile-grammar <file>`: Compila gramáticas GBNF/Regex para amostragem guiada de LLM.
   - `asl sign <file>` & `asl verify <file>`: Assinatura criptográfica Ed25519 preservada.
   - `asl serve`: Varredura recursiva descobre e expõe ferramentas de arquivos com qualquer uma das 9 extensões.

---

## 4. Projeção Sombra Automática Multi-Extensão (Shadow Projection)

A projeção sombra Markdown (ADR-0009) é estendida com suporte completo ao conjunto:

1. **Derivação Canônica de Alvo (`get_shadow_target_path`)**:
   - `nome.<ext>` projeta deterministicamente para `nome.md`.
   - Nomes canônicos em caixa alta têm mapeamento espelhado:
     - `AGENT.agent` ──► `AGENT.md`
     - `PROMPT.prompt` ──► `PROMPT.md`
     - `TOOL.tool` ──► `TOOL.md`
     - `GUARD.guard` ──► `GUARD.md`
     - `PERSONA.persona` ──► `PERSONA.md`
     - `CHAIN.chain` ──► `CHAIN.md`
     - `RULES.rules` ──► `RULES.md`
     - `ASL.asl` ──► `ASL.md`
     - `SKILL.skill` ──► `SKILL.md`
   - Em caso de colisão com arquivo de documentação manual pré-existente (sem marca d'água ASL), a projeção é protegida criando `nome.asl.md`.

2. **Rastreamento de Origem e Prevenção de Falsos Órfãos (`clean_orphaned_shadows`)**:
   - Ao inspecionar um arquivo `.md` contendo a marca d'água ASL, o motor extrai `asl_canonical_source` do frontmatter ou testa a existência de arquivo base com qualquer uma das 9 extensões válidas (`ASL_EXTENSIONS`).
   - Um arquivo `.md` sombra é considerado órfão e removido **exclusivamente se nenhuma das 9 extensões de origem existir**.

---

## 5. Consequências e Benefícios

- **Expressividade Semântica Máxima**: Engenheiros de IA e agentes organizam seus repositórios com clareza cristalina (`guardrails/`, `prompts/`, `agents/`, `tools/`).
- **Zero Fricção**: Renomear de `.skill` para `.agent`, `.guard` ou `.prompt` funciona instantaneamente sem alteração de código.
- **Ecossistema Unificado**: O ASL consolida-se como o padrão definitivo para artefatos determinísticos em inteligência artificial.
