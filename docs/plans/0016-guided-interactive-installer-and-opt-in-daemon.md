# Plano de Implementação: Instalação Interativa Guiada e Ativação Consentida do Daemon (ADR-0016)

- **Status**: Em Execução
- **Data**: 2026-09-18
- **Meta**: Oferecer instalação guiada transparente e não-intrusiva onde o desenvolvedor escolhe se deseja ativar o daemon em segundo plano, com suporte a `/dev/tty`, flags automatizadas e comando `asl setup`.

---

## Fases de Execução

### Fase 1: Atualização do `install.sh`
- Suporte a flags `--enable-daemon`, `--no-daemon`, `-y`, `--yes`, `-n`, `--no`.
- Detecção de tty interativo com fallback para `/dev/tty` (compatível com `curl ... | bash`).
- Prompt explicativo com default `(Recommended) [Y/n]`.
- Fluxo de ativação via `asl daemon install` ou fluxo informativo sob demanda (`asl sync`).

### Fase 2: Implementação do Comando CLI `asl setup`
- Inclusão do subcommand `Setup` no enum `Commands` em `main.rs`.
- Função `handle_setup()` em `daemon_cmds.rs` implementando o wizard interativo no terminal.
- Garantia estrita de limite cognitivo < 450 linhas em todos os arquivos `.rs`.

### Fase 3: Validação dos Guardrails e Testes
- Execução do script `./scripts/guardrail_check.sh`.
- Testes manuais do script `install.sh` e do comando `asl setup`.
- Verificação de conformidade com a política English-only.

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.
