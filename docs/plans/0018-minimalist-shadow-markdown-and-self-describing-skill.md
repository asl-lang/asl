# Plano de Implementação: Projeção Sombra Minimalista e Auto-Descoberta Segura (ADR-0018)

- **Status**: Concluído
- **Data**: 2026-09-18
- **Meta**: Remover todo o ruído visual, referências a marcas de IA, benchmarks e tutoriais da projeção sombra `.md`, transferindo a auto-descoberta de execução de forma segura para a tríade `.skill`, `.tool` e `.asl` com shebang executável e gestão 100% autônoma pelo daemon, sem tocar em arquivos do Claude (`CLAUDE.md`).

---

## Fases de Execução

### Fase 1: Limpeza da Projeção Sombra em `asl-parser` (`shadow.rs`)
- Refatorar `generate_shadow_content`:
  - Eliminar menções a providers de LLM (`Claude Code, Cursor, Codex`).
  - Eliminar afirmações de benchmark de tokens.
  - Eliminar tutorial de `asl run` do Markdown sombra.
  - Eliminar cabeçalho redundante `## Official Semantic Instructions`.
  - Evitar duplicação de `# <nome>` quando as instruções já iniciarem com cabeçalho Markdown.

### Fase 2: Suporte a Shebang e Auto-Scaffold da Tríade pelo Daemon (`shadow_cmds.rs`)
- Suporte a `#!/usr/bin/env -S asl run` na extração de frontmatter e digest do `asl-parser`.
- O daemon detecta arquivos vazios (0-bytes via `touch`) da tríade `.skill`, `.tool` e `.asl`:
  - Popula automaticamente com shebang e template canônico limpo.
  - Atribui permissão de execução (`chmod 0755` no Unix).
  - Projeta sombra `.md` limpa exclusivamente para arquivos `.skill`.
  - Preserva integralmente e nunca altera arquivos de terceiros como `~/.claude/CLAUDE.md`.

### Fase 3: Validação de Guardrails e Testes
- Adicionar testes de unidade para suporte a shebang e scaffold da tríade.
- Executar `./scripts/guardrail_check.sh` (todos os 6 passos aprovados).
- Teste real com execução via shebang `./demo.skill` e `./demo.tool`.

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.

