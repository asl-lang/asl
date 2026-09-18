# ADR-0019: Subsistema de Auto-Instrução na CLI e Descoberta para IAs (asl docs / asl learn)

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-cli` (`docs_cmds.rs`, `main.rs`)
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 6 (Prefixo Estático e KV-Cache), Axioma 7 (Design Amigável e Limite Cognitivo)

---

## 1. Contexto e Declaração do Problema

Quando desenvolvedores e agentes autônomos de IA (como Claude Code, Cursor, Copilot, Codex, ChatGPT) interagem com o ASL pela primeira vez em um ambiente de desenvolvimento ou container:
1. Frequentemente desconhecem a sintaxe canônica do ASL 3.0 (YAML frontmatter, blocos de código Starlark determinístico `asl:deterministic`, e regras semânticas declarativas `asl:rules`).
2. Agentes de IA consomem dezenas de milhares de tokens navegando na web ou lendo documentações externas caso não exista uma referência de alta densidade no próprio binário da CLI.
3. Não havia uma forma imediata de gerar scaffolding sob demanda no terminal com um comando conciso como `asl template skill` ou `asl template tool`.

Portanto, a própria CLI do ASL deve fornecer comandos nativos, de baixa latência e auto-contidos para instruir tanto seres humanos quanto agentes de IA sobre a linguagem, sintaxe, regras e exemplos.

---

## 2. Proposta Detalhada da Decisão

Decidimos introduzir o comando canônico `asl docs` (com os aliases universais `asl learn`, `asl syntax`, `asl guide` e `asl cheat`), juntamente com o comando gerador `asl template`.

### 2.1. Tópicos Suportados em `asl docs [tópico]`
- **`overview` (ou padrão sem argumentos)**: Visão geral da arquitetura do ASL 3.0, isolamento hermético, sandbox Starlark e execução via shebang `#!/usr/bin/env -S asl run`.
- **`syntax`**: Campos obrigatórios e opcionais do frontmatter YAML (`name`, `asl_version`, `description`, `capabilities`, `limits`, `interface`) e tags dos blocos de código Markdown (`asl`, `asl:rules`, `asl:deterministic`).
- **`rules`**: Guia da sintaxe declarativa de regras semânticas (`match <alvo>:`, cláusulas `when <padrão>:`, guardas `guard <expressão>`, e ações `return <valor>`).
- **`triad`**: Explicação da tríade de arquivos (`.skill` com projeção sombra `.md`, `.tool` para ferramentas MCP puras e `.asl` para módulos nativos).
- **`practices` (ou `best-practices`)**: Boas práticas canônicas para desenvolvedores e agentes que optam por **não utilizar o daemon de background** (uso de shebang na linha 1, comandos sob demanda `asl sync` e `asl check`, e watcher efêmero de primeiro plano `asl watch`).
- **`mcp`**: Como expor habilidades como servidores MCP herméticos via stdio ou HTTP/SSE (`asl serve`).
- **`examples`**: Exemplos canônicos completos e testados para cada extensão e funcionalidade.

### 2.2. Otimização para Modelos de Linguagem (`asl docs --ai`)
Com a flag `--ai`, o comando emite um resumo de altíssima densidade semântica (cerca de ~250 tokens), projetado especificamente para ser injetado em prompts de sistema de agentes de IA, contendo:
- Esquema canônico em formato BNF/Markdown ultra-conciso.
- Regras de sandbox e execução.
- Modelo mental de transcompilação em memória Starlark L1.

### 2.3. Emissão de Templates Prontos (`asl template <tipo>`)
- `asl template skill`: Emite um rascunho canônico de `.skill`.
- `asl template tool`: Emite uma ferramenta MCP com interface e código determinístico.
- `asl template rules`: Emite uma habilidade governada por regras declarativas.
- `asl template asl`: Emite um módulo raiz ASL.

### 2.4. Ensino de Boas Práticas e Fluxo Sem Daemon
Para atender usuários que não desejam serviços em segundo plano:
1. **Shebang na Linha 1**: Ensinar o uso de `#!/usr/bin/env -S asl run` para que tanto IAs quanto o terminal executem o arquivo diretamente.
2. **Sincronização Sob Demanda**: Ensinar `asl sync .` para gerar sombras `.md` em batch e `asl check <arquivo>` para atualizar no salvamento.
3. **Watcher Efêmero**: Ensinar `asl watch .` para sincronização em foreground apenas durante sessões ativas de codificação.


---

## 3. Alternativas Consideradas

- **Alternativa A: Exigir que a IA acesse a documentação web via curl**:
  - *Descartada*: Lenta, requer conexão com a internet, gasta tokens excessivos e pode falhar em ambientes isolados (air-gapped/sandbox).
- **Alternativa B: Depender exclusivamente do `asl --help` do Clap**:
  - *Descartada*: Apenas descreve argumentos de CLI, sem ensinar a sintaxe da linguagem ASL, a DSL de regras declarativas ou como estruturar arquivos `.skill`.
- **Alternativa C: Subsistema Nativo Auto-Instrutivo Integrado na CLI (Escolhida)**:
  - *Justificativa*: Zero dependências externas (Axioma 2), latência de execução instantânea (< 5ms), suporte nativo a flags de IA (`--ai`) e geração de templates padronizados.

---

## 4. Consequências e Benefícios

### Positivas
- **Aceleração de Adoção por IAs**: Qualquer agente de IA pode executar `asl learn --ai` ou `asl syntax` e dominar a escrita de skills em uma única chamada.
- **Ergonomia e Produtividade**: Desenvolvedores podem gerar templates prontos com `asl template tool > minha.tool`.
- **100% Offline e Hermético**: Funciona sem internet em qualquer ambiente.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Os templates gerados são autocontidos e atômicos.
- [x] **Axioma 2 (Zero Dependências Externas)**: Toda a documentação e templates estão embutidos diretamente no binário Rust.
- [x] **Axioma 3 (Segurança OCAP e Sandbox)**: Ensina a declarar limites e capabilities estritas.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Isolado como comando adaptador na camada `asl-cli`.
- [x] **Axioma 5 (Término Determinístico)**: Comandos de documentação retornam imediatamente sem loops de I/O.
- [x] **Axioma 6 (Prefixo Estático)**: A flag `--ai` é formatada de forma densa e estável, otimizando o KV-cache dos agentes.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Módulo `docs_cmds.rs` e `main.rs` mantidos rigorosamente abaixo de 450 linhas.
