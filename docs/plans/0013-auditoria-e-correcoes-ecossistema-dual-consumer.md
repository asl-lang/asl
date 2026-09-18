# Plano de Implementação: Auditoria Científica e Correções do Ecossistema Dual-Consumer

- **ADR Vinculado**: `docs/adrs/0013-auditoria-e-correcoes-ecossistema-dual-consumer.md`
- **Status**: Concluído (5/5 Fases - 100%)
- **Data**: 2026-09-18
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Corrigir com segurança e minimalismo os problemas confirmados na auditoria profunda das 31 hipóteses, preservando integralmente o conceito Dual-Consumer e atendendo a 100% dos guardrails do ASL.

---

## Fase 1: Motor Starlark e Transpilação de Regras (`asl-vm-starlark`, `asl-parser`)
Injeção de `_asl_matches_regex` em `asl_natives` e transpilação de `when matches("pattern"):` para o helper de regex com fallthrough de blocos match múltiplos.
- Validação: `cargo test -p asl-vm-starlark && cargo test -p asl-parser`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 2: Digest Canônico e Fallback de Entrypoint (`asl-parser`)
Ajuste do cálculo de digest para omitir `digest:` e `signature:` estritamente no bloco de frontmatter, e adoção de `manifest.interface.entrypoint` no gerador determinístico fallback.
- Validação: `cargo test -p asl-parser`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 3: Conformidade GBNF RFC 8259, Unicode e Escrita Atômica (`asl-parser`)
Ajuste dos terminais numéricos do GBNF, contagem escalar Unicode no `prefix_analyzer` e escape YAML com gravação atômica em `shadow.rs`.
- Validação: `cargo test -p asl-parser`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 4: Política de Host, Bind Local e Streaming SSE (`asl-cli`, `asl-protocol-http`)
Adição de `--allowed-root` no CLI, bind padrão em `127.0.0.1` no servidor HTTP MCP, suporte a streaming keepalive em `/sse` e aviso para documentos autoassinados.
- Validação: `cargo test -p asl-cli -p asl-protocol-http`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 5: Integração Completa, CI e Guardrails de 5 Estágios
Criação da pipeline GitHub Actions `.github/workflows/ci.yml` e validação de 100% no script `./scripts/guardrail_check.sh`.
- Validação: `./scripts/guardrail_check.sh`
- Finalização: Commit & Push com `git push origin main`.
