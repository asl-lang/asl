# ADR-0011: Tríade Canônica de Extensões do ASL (.skill, .tool, .asl) e Isolamento de Sombra

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Revisores Científicos**: Painel de Arquitetura de Sistemas Autônomos (Knuth, Lamport, Liskov, Miller, Chomsky, Amodei, Shazeer)
- **Crates Afetadas**: `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-cli`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade e Integridade), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 6 (Invariância de KV-Cache), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Motivação

O **Agent Skill Language (ASL 3.0)** nasceu com o formato `.skill` para unificar contrato (YAML), semântica (Markdown estável) e execução hermética (Starlark/Wasm/Rules).

Ao analisar a expansão do ecossistema e auditar o mercado global de formatos de arquivos de IA e agentes, foram identificados riscos graves de colisão com ecossistemas externos:
- `.agent` e `.prompt`: Formatos capturados pelo ecossistema Microsoft / VS Code Copilot (`*.agent.md`, `*.prompt.md`).
- Formatos genéricos dispersos causam inflação de conceitos e confusão cognitiva para desenvolvedores e modelos.

Decide-se adotar um modelo conciso, robusto e minimalista baseado em uma **Tríade Canônica de Extensões**:
1. **`.skill`**: A unidade clássica e rica de habilidade de agente (com suporte total ao motor de projeção sombra `.md` para descoberta por agentes de IA como Claude Code, Cursor e Antigravity).
2. **`.tool`**: A unidade executável pura / MCP Tool atômica (função determinística sem projeção sombra, 100% livre de colisões industriais).
3. **`.asl`**: A extensão raiz da própria linguagem (*Agent Skill Language*), atômica e sem projeção sombra.

---

## 2. Proposta Detalhada da Arquitetura

### 2.1 A Tríade Canônica de Extensões

| Extensão | Semântica no Ecossistema | Projeção Sombra (`.md`) | Propósito e Caso de Uso |
| :--- | :--- | :---: | :--- |
| **`.skill`** | **Habilidade Modular Canônica** | ✅ **Sim** (`.md`) | Skills completas que necessitam de retrocompatibilidade com descoberta baseada em Markdown (`SKILL.md`). |
| **`.tool`** | **Ferramenta Executável / MCP Tool** | ❌ Não | Funções determinísticas puras acionáveis por IA; sem arquivos `.md` adicionais. |
| **`.asl`** | **Raiz da Linguagem ASL** | ❌ Não | Código geral, bibliotecas e rotinas atômicas da linguagem; sem arquivos `.md` adicionais. |

### 2.2 Princípio da Sombra Restrita à Skill

A Projeção Sombra (`.md`, ADR-0009) é um mecanismo concebido para interoperabilidade com sistemas que leem documentação de habilidades em Markdown.
- **Apenas arquivos `.skill` geram `.md`** (`is_shadow_eligible`).
- Arquivos `.tool` e `.asl` operam de forma 100% limpa, garantindo que nenhum arquivo `.md` residual seja criado no repositório.

### 2.3 Modelo Polimórfico Universal de Execução

Os 3 formatos compartilham o mesmo compilador, o mesmo parser e o mesmo runtime:
- Frontmatter YAML padronizado.
- Seção Semântica Markdown com prefixo estático (100% de KV-Cache reusável).
- Bloco determinístico de código (`asl` ou `asl:rules`).
- Execução direta com `asl run`, auditoria com `asl check`, expansão com `asl expand` e serviço MCP com `asl serve`.

---

## 3. Alternativas Consideradas

### Alternativa A: Adotar 9+ Extensões Especializadas (.agent, .prompt, .guard, .chain, etc.)
- **Por que foi descartada**: Sobrecarga conceitual. `.agent` e `.prompt` colidem com as convenções do VS Code/Copilot (`*.agent.md`, `*.prompt.md`). O minimalismo da tríade `.skill`, `.tool` e `.asl` cobre 100% dos casos de uso de forma muito mais elegante.

### Alternativa B: Manter Apenas .skill
- **Por que foi descartada**: Não permite distinguir uma ferramenta pura (MCP tool simples) de uma habilidade completa documentada para agentes. A inclusão de `.tool` e `.asl` oferece a flexibilidade necessária com zero risco de colisão.

### Alternativa C (Escolhida): Tríade Focada (.skill, .tool, .asl) com Sombra Exclusiva para .skill
- **Por que foi escolhida**: Clareza cristalina, zero arquivos indesejados no disco e total robustez.

---

## 4. Consequências e Benefícios

### Positivas:
- **Simplicidade Máxima**: Desenvolvedores e IAs precisam memorizar apenas 3 extensões claras.
- **Repositório Limpo**: Nenhum arquivo `.md` é gerado para ferramentas (`.tool`) ou arquivos nativos (`.asl`).
- **Compatibilidade Total**: O padrão de skills com `SKILL.md` sombra continua funcionando perfeitamente.

### Negativas e Mitigações:
- Nenhuma desvantagem técnica identificada; simplificação radical do escopo de manutenção.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do Documento)**: Todos os 3 formatos são autocontidos.
- [x] **Axioma 2 (Zero Dependências Externas)**: Implementação 100% Rust nativo.
- [x] **Axioma 3 (Confinamento OCap)**: Interface de segurança uniforme.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Constantes em `asl-spec`, parsing em `asl-parser`.
- [x] **Axioma 5 (Término Determinístico com Fuel)**: Execução limitada e segura.
- [x] **Axioma 6 (Prefixo Estático Imutável)**: Invariância de KV-Cache em todos os 3 formatos.
- [x] **Axioma 7 (Limite Cognitivo < 450 linhas)**: Todos os arquivos estritamente conformes.
