# ADR-0008: Otimizador de Prefixo Estático e Analisador de KV-Cache para LLMs

- **Status**: Concluído
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Jean Catarina, Arquitetura ASL
- **Crates Afetadas**: `asl-parser`, `asl-cli`
- **Axiomas Relacionados**: Axioma 6 (Invariância de Prefixo Estático para KV-Cache), Axioma 1 (Determinismo Hermético)

---

## 1. Contexto e Problema

Provedores modernos de modelos de linguagem (Anthropic Prompt Caching, OpenAI Prefix Caching, DeepSeek, vLLM, SGLang) dependem da correspondência exata de prefixos de tokens para reutilizar o estado de atenção já calculado (KV-Cache) na memória GPU. Quando o prefixo de um prompt é 100% idêntico entre requisições:
1. O **Time-to-First-Token (TTFT)** é reduzido em até 80-90%.
2. Os **custos de inferência** são reduzidos tipicamente em 50-90% em provedores de nuvem.

No ecossistema de agentes, instruções semânticas, regras de governança e descrições de ferramentas frequentemente contêm interpolações prematuras de variáveis dinâmicas (ex.: `timestamp`, `session_id`, `user_name`, `input`) no topo do prompt. Isso invalida todo o cache subsequente em cada chamada, degradando a eficiência do agente.

O ASL 3.0 consagra o Axioma 6 ("Invariância de Prefixo Estático para KV-Cache"). Para garantir essa propriedade de forma auditável e automática na linguagem, faz-se necessário um analisador estático e otimizador de prefixo embutido nas ferramentas oficiais do ASL.

---

## 2. Proposta Detalhada da Decisão

1. **Módulo Analisador em `asl-parser` (`prefix_analyzer`)**:
   - Analisa a seção semântica Markdown de um `SkillDocument`.
   - Detecta padrões de variáveis dinâmicas e interpolações (`{{variable}}`, `${variable}`, `{variable}`).
   - Segmenta o prompt em:
     - **Prefixo Estático Canônico**: Bloco de texto contínuo e invariante desde o início até a primeira variável dinâmica encontrada.
     - **Sufixo Dinâmico**: Bloco contendo variáveis interpoláveis e parâmetros de runtime.
   - Calcula métricas quantitativas:
     - Comprimento estático (caracteres e tokens estimados a ~4 chars/token).
     - Taxa de acerto projetada de KV-cache (0.0% a 100.0%).
     - Identificação de riscos de invalidação de cache (ex.: variáveis dinâmicas antes das regras estáticas).

2. **Otimizador de Prefixo (`optimize_prefix`)**:
   - Reorganiza a seção semântica para consolidar todas as instruções invariantes (regras, papéis, esquemas) no topo do prompt.
   - Isola placeholders dinâmicos em uma seção dedicada de sufixo (`## Contexto e Parâmetros Dinâmicos`).
   - Garante que a semântica seja preservada enquanto maximiza a extensão do prefixo estático contínuo.

3. **Interface CLI Integrada (`asl analyze-prefix` e `asl optimize-prefix`)**:
   - `asl analyze-prefix <skill_file>`: Relatório visual com hit-rate projetado, tokens de prefixo e diagnóstico de riscos.
   - `asl optimize-prefix <skill_file> [--in-place]`: Aplica a otimização de prefixo e reescreve a skill mantendo a validade de digest e assinaturas.

---

## 3. Alternativas Consideradas

- **Alternativa A: Delegar caching exclusivamente para bibliotecas externas de Python**:
  - *Descarte*: Viola o princípio de autossuficiência e o Axioma 2 do ASL.
- **Alternativa B: Parser apenas em tempo de execução de inferência**:
  - *Descarte*: Não fornece auditoria estática prévia (`asl check` ou CI/CD) para garantir a governança das skills antes do deploy.
- **Alternativa C: Analisador e otimizador estático integrado no parser ASL (Escolhida)**:
  - *Justificativa*: Permite validação estática imediata, otimização automática `in-place` e métricas quantitativas de economia de tokens.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - **Eficiência Extrema de LLM**: Skills ASL são otimizadas nativamente para alto hit rate no KV-cache em Anthropic, OpenAI e vLLM.
  - **Auditoria Proativa**: Desenvolvedores e agentes detectam imediatamente quebras de cache durante `asl analyze-prefix`.
  - **Compatibilidade Completa**: O otimizador preserva o formato CommonMark e a integridade de execução determinística.
- **Negativas**:
  - Reordenação de blocos semânticos exige que seções dinâmicas sejam marcadas com identificadores reconhecíveis.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: Toda a análise e otimização operam dentro da unidade atômica `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: Implementado puramente em Rust no parser padrão.
- [x] **Axioma 3 (Confinamento ocap)**: Operação puramente funcional em memória sem efeitos colaterais descontrolados.
- [x] **Axioma 4 (Isolamento hexagonal)**: Lógica contida no núcleo `asl-parser` com interface no `asl-cli`.
- [x] **Axioma 5 (Término determinístico)**: Varredura de strings linear $O(N)$ garantindo término imediato.
- [x] **Axioma 6 (Invariância de Prefixo Estático para KV-Cache)**: Atendido de forma direta e central por este ADR.
- [x] **Axioma 7 (Limite de < 450 linhas)**: Módulos divididos em `prefix_analyzer.rs` (210 linhas) e `prefix_cmds.rs` (95 linhas).
