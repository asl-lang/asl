# Plano de Implementação: Arquitetura Pura da Linguagem Semântica ASL e Tag Universal Única (ADR-0020)

- **Status**: Em Progresso
- **Data**: 2026-09-18
- **Meta**: Eliminar suporte retrocompatível a tags antigas (`asl:deterministic`, `asl:rules`, `starlark`, `python-deterministic`), unificando tudo na tag universal única ````asl````. Detectar automaticamente sintaxe declarativa vs procedural dentro do bloco ````asl````. Fornecer a Rosetta Stone canônica e alinhar CLI, runtime, testes e website para 100% de coesão.

---

## Fases de Execução

### Fase 1: Especificação e SDD
- Atualizar ADR-0020 (`docs/adrs/0020-pure-asl-semantic-language-architecture.md`).
- Atualizar PLAN-0020 (`docs/plans/0020-pure-asl-semantic-language-architecture.md`).
- Atualizar índices em `docs/adrs/README.md` e `docs/plans/README.md`.

### Fase 2: Tag Única e Detecção Semântica no Parser (`asl-parser`, `asl-spec`)
- Em `asl-spec/src/lib.rs`: Atualizar mensagem de erro para `Missing ASL code block (```asl)`.
- Em `asl-parser/src/lib.rs`:
  - Apenas aceitar a tag `asl` (rejeitando `asl:rules`, `asl:deterministic`, `starlark`, etc.).
  - Dentro do bloco `asl`, detectar automaticamente se o conteúdo é declarativo (`guard:`, `match`, `otherwise:`) ou procedural (`def run...`).
  - Encaminhar para o transpilador de regras se declarativo, ou direto para a ASL VM se procedural.
- Atualizar testes de unidade do parser e fixtures para usar unicamente ````asl````.

### Fase 3: Atualização de Testes Existentes e Fixtures (`asl-cli/tests/`)
- Atualizar todos os testes de integração em `runtime/crates/asl-cli/tests/` substituindo ````asl:deterministic```` e ````asl:rules```` por ````asl````.
- Adicionar testes comprovando que tags legadas (`asl:rules`, `starlark`, etc.) NÃO são executadas.

### Fase 4: Rosetta Stone e Rebranding na CLI (`asl-cli`)
- Em `prefix_cmds.rs`:
  - Atualizar cabeçalhos para "ASL VM Deterministic Engine (Pure ASL)".
  - Exibir bloco em formato canônico ````asl````.
  - Garantir limite estrito < 450 linhas (Axioma 7).
- Em `docs_cmds.rs`:
  - Adicionar tópico `migration` e aliases (`rosetta`, `bash`, `python`, `starlark`).
  - Documentar a tag universal única ````asl```` em `overview`, `syntax`, `rules` e `--ai`.
  - Garantir limite estrito < 450 linhas (Axioma 7).

### Fase 5: Alinhamento do Website (`website/`)
- Atualizar páginas e componentes em `website/src/` para referenciar exclusivamente ````asl```` e a ASL VM.
- Remover referências a "Starlark L1" e tags antigas.
- Adicionar seção da Rosetta Stone na documentação do website.

### Fase 6: Validação de Guardrails e Testes
- Executar `./scripts/guardrail_check.sh` (todas as 6 fases aprovadas).
- Testes manuais na CLI: `asl docs migration`, `asl docs bash`, `asl template skill`, `asl expand`.

### Fase 7: Empacotamento, Release e Commit
- Recompilar e instalar localmente: `cargo install --path runtime/crates/asl-cli --force`.
- Empacotar releases darwin arm64 e x86_64.
- Atualizar release v0.3.0 no GitHub.

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.
