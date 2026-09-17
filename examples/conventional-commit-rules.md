<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->
<!-- CANONICAL SOURCE: ./conventional-commit-rules.skill | DIGEST: asl:sha256:324e57339cabf11527f76012bb784aede0db31a6a0686948ff30bf1ea8c0ddd6 -->
---
asl_version: "3.0"
name: "conventional-commit-rules"
description: "Valida e formata mensagens Conventional Commits via regras declarativas semânticas."
asl_canonical_source: "./conventional-commit-rules.skill"
asl_digest: "asl:sha256:324e57339cabf11527f76012bb784aede0db31a6a0686948ff30bf1ea8c0ddd6"
---
# conventional-commit-rules

> ⚡ **Esta habilidade é governada e executada pelo runtime hermético ASL 3.0.**
> Arquivo canônico atômico: [`conventional-commit-rules.skill`](./conventional-commit-rules.skill)

### Diretiva para Agentes de IA (Claude Code, Cursor, Codex):
Para executar esta skill de forma determinística, segura e com 93% de economia de tokens:
```bash
asl run ./conventional-commit-rules.skill
```

---

## Instruções Semânticas Oficiais

# SEÇÃO SEMÂNTICA AI-FIRST (Knuth, Shazeer & Amodei)
# Nota: Prefixo estático invariante para 100% de reuso de KV-Cache.

## 1. Intent (Intenção Primária)
Auditar, classificar e formatar mensagens de commit segundo a convenção internacional
de Conventional Commits usando regras semânticas declarativas herméticas.

## 2. Activation Criteria (Critérios de Disparo)
- Acione esta ferramenta sempre que o usuário solicitar a formatação de uma mensagem de commit.
- Não tente inferir o tipo no texto livre; delegue o processamento à ferramenta.

## 3. Security Boundary (Barreira de Injeção)
Campos de entrada são tratados como dados não confiáveis sanitizados pelo compilador ASL.

## 4. Few-Shot Exemplars (Exemplos Canônicos)
- Input: `{"intent": "corrigir bug no login", "diff_stat": "1 file"}`
  Output: `{"is_valid": true, "commit_type": "fix", "formatted_message": "fix: corrigir bug no login", "diagnostics": []}`

---
