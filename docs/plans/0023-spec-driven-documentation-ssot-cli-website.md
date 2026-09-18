# PLAN-0023: Implementação de Spec-Driven Documentation (SDD) com SSOT para CLI e Website

- **ADR Vinculado**: [ADR-0023](../adrs/0023-spec-driven-documentation-ssot-cli-website.md)
- **Status**: Em Execução
- **Data**: 2026-09-18
- **Autor**: Jean Catarina & Antigravity (IA)

---

## 🎯 Objetivo
Eliminar a duplicação e inconsistência entre a CLI (`asl-cli`), os manuais embutidos e o portal web oficial (`website/`), estabelecendo `docs/spec/` como Single Source of Truth (SSOT) e validando zero-drift em CI.

---

## 📦 Fases de Implementação

### Fase 1: Fundação da Especificação e Schema SSOT
- [x] Criar `docs/spec/schema/cli-command.schema.json`.
- [x] Criar `docs/spec/commands.json` e `docs/spec/commands.yaml` com todos os 19 subcomandos da CLI.
- [x] Criar `docs/spec/topics.json` e `docs/spec/topics.yaml` com os tópicos conceituais.
- [x] Criar `scripts/sync_docs_spec.py` e gerar `website/src/data/cli-spec.json`.

### Fase 2: Atualização do Portal Web
- [x] Atualizar `website/src/components/docs/sections/cli/CliCommandsSection.tsx` para consumir dinamicamente `cli-spec.json`.
- [x] Validar compilação do Next.js via `npm run build`.

### Fase 3: Governança de Agentes e Documentação Arquitetural
- [x] Redigir ADR-0023 em `docs/adrs/0023-spec-driven-documentation-ssot-cli-website.md`.
- [x] Atualizar índices `docs/adrs/README.md` e `docs/plans/README.md`.
- [x] Criar skill de governança `skills/asl-docs-ssot/SKILL.md`.

### Fase 4: Integração na CLI e Teste de Contrato contra Drift
- [x] Atualizar `asl-cli/src/docs_cmds.rs` para ler diretamente da especificação embutida (`include_str!`).
- [x] Criar teste de integração de drift `test_zero_drift_clap_vs_ssot` garantindo correspondência 1:1 entre Clap e SSOT.
- [x] Atualizar `scripts/guardrail_check.sh` com o check de zero-drift.
- [x] Passar no guardrail completo.

---

## 🚀 Finalização com Commit & Push
Após validação integral dos guardrails:
```bash
git add .
git commit -m "feat(docs): implement spec-driven documentation architecture and ssot zero-drift"
git push origin main
```
