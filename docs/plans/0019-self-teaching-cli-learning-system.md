# Plano de Implementação: Subsistema de Auto-Instrução na CLI e Descoberta para IAs (ADR-0019)

- **Status**: Concluído
- **Data**: 2026-09-18
- **Meta**: Implementar comandos nativos `asl docs` (aliases: `learn`, `syntax`, `guide`, `cheat`) e `asl template` com suporte a resumos densos para LLMs (`--ai`), boas práticas para fluxos sob demanda sem daemon e geração de templates canônicos da tríade.

---

## Fases de Execução

### Fase 1: Criação do Módulo `docs_cmds.rs` em `asl-cli`
- Implementar `handle_docs(topic: &str, ai: bool, json: bool) -> Result<()>`:
  - Formatação Markdown clara, em inglês estrito, com seções temáticas (`overview`, `syntax`, `rules`, `triad`, `mcp`, `examples`).
  - Formatação compacta e de alta densidade semântica para `--ai`.
- Implementar `handle_template(target_type: &str) -> Result<()>`:
  - Templates canônicos e executáveis para `skill`, `tool`, `asl` e `rules`.
- Adicionar testes de unidade no próprio módulo validando todos os tópicos e templates.

### Fase 2: Integração e Otimização de Linhas em `main.rs`
- Declarar `mod docs_cmds;`.
- Adicionar comandos `Commands::Docs` e `Commands::Template`.
- Modularizar comandos longos de `main.rs` para garantir que o arquivo permaneça estritamente < 450 linhas (Axioma 7).

### Fase 3: Validação de Guardrails e Testes
- Executar `./scripts/guardrail_check.sh` (garantir 100% de sucesso nas 6 fases).
- Teste real na CLI com `asl docs`, `asl learn --ai`, `asl syntax`, `asl template tool`.

### Fase 4: Atualização de Release
- Recompilar e instalar localmente em `~/.cargo/bin/asl`.
- Empacotar releases (`aarch64-apple-darwin` e `x86_64-apple-darwin`).
- Atualizar release v0.3.0 no GitHub.

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.

