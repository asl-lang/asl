# Plano de Implementação: Ergonomia Integral, Confinamento OCap e Diagnósticos Transparentes

- **ADR Vinculado**: `docs/adrs/0022-unified-ocap-capabilities-and-developer-ergonomics.md`
- **Data**: 2026-09-18
- **Responsável**: Jean Catarina & Antigravity (IA)
- **Status**: Em Planejamento
- **Meta**: Resolver de forma estrutural na arquitetura os 11 pontos de atrito identificados no feedback real de desenvolvedores.

---

## Fases de Execução (SDD)

### Fase 1: Especificação de Esquema Resiliente e Traits OCap (`asl-spec`, `asl-core-traits`)
- Deserialização tolerante e expressiva para `SkillCapabilities` (suporte a listas e structs para `fs`, `net`, `domains`, `env`).
- Extensão do `CapabilityContext` com `write_file`, `file_exists`, `list_dir`.

### Fase 2: Confinamento Seguro de Sistema de Arquivos (`asl-security`)
- Implementação de `write_file` em `ConfinedSecurityContext` com checagem rigorosa de `allow_write` / `write_roots`, resolução canônica e consumo de fuel (1 opcode por 16 bytes).
- Implementação de `file_exists` e `list_dir`.

### Fase 3: Motor Starlark, Diagnósticos Transparentes e Ergonomia (`asl-vm-starlark`)
- Conexão dos primitivos de escrita e listagem de filesystem em `ctx.fs`.
- Registro de helpers globais seguros: `chars`, `is_digit`, `to_int`, `is_int`, `to_float`, `try_json`, `try_call`.
- Sanitização de erros: remoção de menções ao Starlark (`.star`), remapeamento de linhas para o arquivo `.skill` do usuário e tradução de atributos inexistentes para sugestões contextuais no frontmatter.

### Fase 4: Pré-processamento e Pattern Matching Universal (`asl-parser`)
- Desugaring de blocos `match <expr>:` em código ASL procedural para sequências `if/elif/else`.

### Fase 5: Ferramental CLI, REPL e Guardrails (`asl-cli`)
- `asl check`: verificação de compilação da função entrypoint, flag `--dry-run` e anúncio explícito de projeção sombra. Suporte a `--no-shadow`.
- `asl repl`: REPL interativo para experimentação ágil com `ctx`.
- `asl template`: templates starter completos com capacidades, limites e schema comentados.
- Execução e aprovação em 100% dos guardrails (`./scripts/guardrail_check.sh`).

---

## Commit & Push

```bash
git add .
git commit -m "feat(spec,vm,cli): unified ocap capabilities, developer ergonomics and transparent diagnostics (ADR-0022)"
git push origin main
```

