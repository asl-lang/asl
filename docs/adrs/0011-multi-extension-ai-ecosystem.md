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

Opor resistência a essa diversidade forçando todo artefato a se chamar `.skill` geraria dissonância cognitiva em equipes de engenharia de IA e para os próprios modelos LLM. Por outro lado, permitir extensões arbitrárias que colidam com linguagens de programação consolidadas causaria falhas graves de reconhecimento em IDEs e no GitHub Linguist.

---

## 2. Proposta Detalhada da Arquitetura

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

### 2.2 Modelo Polimórfico Universal e Execução

Todas as 9 extensões compartilham o formato literate tripartido canônico:
1. **YAML Frontmatter**: Schemas de entrada e saída, metadados, capabilities e limites de segurança.
2. **Seção Semântica Markdown**: Prefixo invariante e instruções AI-first para reuso total de KV-Cache.
3. **Bloco Determinístico**: Código Starlark Core L1 (`asl`) ou regras declarativas (`asl:rules`).

O compilador e runtime ASL aceitam todos esses formatos de forma intercambiável (`asl run`, `asl check`, `asl expand`, `asl serve`, `asl sign`, `asl verify`).

### 2.3 Princípio da Sombra Restrita à Skill (Zero Projeção para Outros Formatos)

O mecanismo de Projeção Sombra (`.md`, ADR-0009) foi criado especificamente para retrocompatibilidade com a convenção de descoberta de habilidades por agentes (`SKILL.md`).

Portanto:
- **Apenas arquivos `.skill` geram projeção sombra `.md`** (`is_shadow_eligible`).
- Arquivos `.agent`, `.prompt`, `.tool`, `.guard`, `.persona`, `.chain`, `.rules` e `.asl` **não geram arquivos `.md`**, mantendo os repositórios dos desenvolvedores estritamente limpos e sem redundância documental.

---

## 3. Alternativas Consideradas

### Alternativa A: Manter Exclusivamente a Extensão `.skill`
- **Descrição**: Forçar todos os conceitos de IA (guardrails, personas, prompts, agentes) a usar a extensão `.skill`.
- **Por que foi descartada**: Dissonância semântica. Um guardrail de segurança de 10 linhas ou um template de prompt não é conceitualmente uma "skill" completa, causando estranheza tanto para desenvolvedores quanto para LLMs.

### Alternativa B: Aceitar Qualquer Extensão Arbitrária
- **Descrição**: Permitir extensões livres como `.ai`, `.flow`, `.action`, `.task`.
- **Por que foi descartada**: Colisão severa com formatos existentes. `.ai` pertence ao Adobe Illustrator; `.flow` pertence ao FlowType do Facebook; `.action` pertence ao Automator / Apache Struts. Isso quebraria o tooling de editores e linguagens.

### Alternativa C (Escolhida): Família Fechada de 9 Extensões de IA com Zero Colisão
- **Descrição**: Seleção criteriosa de 9 extensões modernas de IA com auditoria no GitHub Linguist e comportamento polimórfico no runtime.
- **Por que foi escolhida**: Máxima expressividade de engenharia, clareza taxonômica e zero conflito com ecossistemas existentes.

---

## 4. Consequências e Benefícios

### Positivas:
- **Expressividade Semântica Máxima**: Diretórios como `agents/`, `prompts/`, `guards/` usam extensões especializadas auto-explicativas.
- **Zero Fricção de Adopção**: Renomear ou criar qualquer um dos arquivos funciona imediatamente sem etapas adicionais de compilação.
- **Repositório Limpo**: A ausência de projeção sombra para formatos que não sejam `.skill` evita a proliferação desnecessária de arquivos `.md`.
- **Integração MCP Unificada**: O servidor `asl serve` carrega ferramentas provenientes de qualquer uma das extensões suportadas.

### Negativas e Mitigações:
- **Manutenção de Múltiplas Extensões**: Necessidade de manter a lista centralizada.
  - *Mitigação*: A lista canônica `ASL_EXTENSIONS` e as funções utilitárias residem em uma única crate pura de domínio (`asl-spec`).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade e Integridade)**: Todos os formatos são arquivos atômicos completos com frontmatter, semântica e código embutido.
- [x] **Axioma 2 (Zero Dependências Externas)**: Implementação nativa em Rust sem dependências externas de runtime.
- [x] **Axioma 3 (Confinamento OCap)**: Todas as extensões respeitam rigorosamente a barreira de capacidades do manifesto.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Tipos e constantes em `asl-spec`, parsing em `asl-parser` e comandos em `asl-cli`.
- [x] **Axioma 5 (Término com Fuel)**: O motor de execução Starlark e Rules impõe cotas finitas de combustível a todos os arquivos.
- [x] **Axioma 6 (Invariância de KV-Cache)**: Todas as extensões preservam a estrutura de prefixo estável para 100% de cache-hit.
- [x] **Axioma 7 (Limite Cognitivo < 450 linhas)**: Todas as crates e arquivos permanecem estritamente abaixo do limite de 450 linhas.
