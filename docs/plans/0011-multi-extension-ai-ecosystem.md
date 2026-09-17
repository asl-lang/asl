# Plano de Implementação: Tríade Canônica de Extensões do ASL (.skill, .tool, .asl)

- **ADR Vinculado**: `docs/adrs/0011-multi-extension-ai-ecosystem.md`
- **Status**: Concluído (5/5 Fases - 100%)
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Implementar o suporte nativo e polimórfico para a tríade focada de extensões do ASL (`.skill`, `.tool`, `.asl`), garantindo que apenas `.skill` projete sombra Markdown (`.md`), enquanto `.tool` e `.asl` operem de forma limpa e atômica no disco.

---

## Fase 1: Tríade de Extensões e Elegibilidade de Sombra (`asl-spec`)
Definição de `ASL_EXTENSIONS = &["skill", "tool", "asl"]`, `is_asl_extension`, `is_asl_file` e `is_shadow_eligible` com testes unitários.
- Validação: `cargo test -p asl-spec`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 2: Projeção Sombra Restrita a Skills (`asl-parser`)
Garantia de que `project_shadow_markdown` ignora `.tool` e `.asl`, projetando exclusivamente arquivos `.skill`.
- Validação: `cargo test -p asl-parser`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 3: Tooling da CLI e Servidor MCP (`asl-cli`)
Atualização de `load_skills_recursive`, `handle_sync_shadows` e mensagens de ajuda do `clap`.
- Validação: `cargo test -p asl-cli`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 4: Exemplos Canônicos da Tríade e Guardrail Check
Criação dos artefatos canônicos `examples/security-validator.tool` e `examples/summarizer.asl`, além dos existentes `.skill`.
- Validação: `./scripts/guardrail_check.sh`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 5: Testes de Integração e Conformidade de 5 Níveis
Testes dedicados em `multi_extension_tests.rs` e aprovação de 100% no script `./scripts/guardrail_check.sh`.
- Validação: `cargo test -p asl-cli --test multi_extension_tests`
- Finalização: Commit & Push com `git push origin main`.
