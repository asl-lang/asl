<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->
<!-- CANONICAL SOURCE: ./pr-status-teams.skill | DIGEST: asl:sha256:6dee73fe02e2f80736034cc36fb20515021b5ccac2640dea5c2d481c21e16b85 -->
---
asl_version: "3.0"
name: "pr-status-teams"
description: "Gera status de PRs em Code Review (Backend e Mobile) para Microsoft Teams com zero dependências externas e zero MCP."
asl_canonical_source: "./pr-status-teams.skill"
asl_digest: "asl:sha256:6dee73fe02e2f80736034cc36fb20515021b5ccac2640dea5c2d481c21e16b85"
---
# pr-status-teams

> ⚡ **This skill is governed and executed by the ASL 3.0 hermetic runtime.**
> Canonical atomic file: [`pr-status-teams.skill`](./pr-status-teams.skill)

### Directive for AI Agents (Claude Code, Cursor, Codex):
To execute this skill deterministically, securely, and with up to 93% token savings:
```bash
asl run ./pr-status-teams.skill
```

---

## Official Semantic Instructions

# Status dos PRs em Code Review (Backend e Mobile) → Mensagem pro Teams

## 1. Intent (Intenção Primária)
Auditar, consolidar e formatar deterministamente o status dos Pull Requests parados na coluna "Code Review"
dos boards Jira Backend (CORE, board 101) e Mobile (APP, board 202) para envio ao Microsoft Teams.
Garante execução 100% autônoma, sem necessidade de servidores MCP locais nem scripts de shell satélites.

## 2. Activation Criteria (Critérios de Disparo)
- Acione quando o usuário solicitar o status de Code Review, resumo de PRs pendentes, verificação de aprovações ("likes"),
  ou mensagem formatada para o Teams para os boards CORE e APP.
- Separe sempre o relatório nas seções 🏢 Backend (CORE) e 📱 Mobile (APP).
- Identifique o destaque do dia (⭐) com base no maior número de likes por board, tratando empates.
- Destaque impedimentos no Jira com a bandeira 🚩.

## 3. Security Boundary & OCap Confinement
- Requisições de rede são confinadas aos domínios `company.atlassian.net` e `api.github.com`.
- Credenciais são lidas exclusivamente das variáveis autorizadas: `JIRA_CLI_EMAIL`, `JIRA_CLI_TOKEN` e `GITHUB_TOKEN`.
- Todos os payloads de API e URLs de PR são tratados como dados não confiáveis sanitizados hermeticamente.

---
