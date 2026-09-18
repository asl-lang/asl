# Plano de Implementação: Ciclo de Vida de Auto-Atualização e Auto-Desinstalação (ADR-0017)

- **Status**: Em Execução
- **Data**: 2026-09-18
- **Meta**: Implementar os comandos `asl update` e `asl uninstall` na CLI do ASL para gerenciamento autônomo do binário e serviços do sistema.

---

## Fases de Execução

### Fase 1: Criação do Micro-Módulo `lifecycle_cmds.rs`
- Implementar `handle_update(check_only: bool, force: bool)` com resolução de tags de release e substituição atômica de binário.
- Implementar `handle_uninstall(yes: bool, purge: bool)` com encerramento de daemons, remoção de serviços do SO e auto-deleção do binário.
- Manter o arquivo estritamente abaixo do limite de 450 linhas.

### Fase 2: Integração na CLI (`main.rs`)
- Adicionar subcommands `Update` e `Uninstall` no enum `Commands` com aliases `upgrade`, `self-update`, `self-uninstall`.
- Conectar o despacho de comandos a `lifecycle_cmds`.

### Fase 3: Validação de Guardrails e Testes
- Executar `./scripts/guardrail_check.sh` garantindo limites de linhas, linter Clippy e conformidade English-only.
- Testar execução manual de `asl update --check` e `asl uninstall` (com cancelamento e flags).

### Commit & Push
- Validar todos os guardrails com `./scripts/guardrail_check.sh`.
- Executar commit convencional e `git push origin main`.
