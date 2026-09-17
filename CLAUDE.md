# CLAUDE.md: Diretrizes e Guardrails do Projeto ASL para Claude Code

> 🛑 **MANDATO DE ENGENHARIA**: Este projeto implementa a **Agent Skill Language (ASL 3.0)**. 
> Consulte [`ARCHITECTURE.md`](./ARCHITECTURE.md), [`ASL_SCIENTIFIC_PAPER.md`](./ASL_SCIENTIFIC_PAPER.md) e [`AGENTS.md`](./AGENTS.md) (para diretrizes e organização de testes) como fontes primárias de verdade.

---

## ⚡ Comandos Canônicos do Workspace

Execute todos os comandos a partir de `agent skill language/runtime/`:

```bash
# Build e Verificação
cargo check                          # Verifica todas as micro-crates
cargo test                           # Executa todos os testes unitários em paralelo
cargo clippy                         # Linter estrito de código Rust

# Testes de Ferramenta ASL (CLI)
cargo run --bin asl -- check "../examples/git-conventional-commit.skill"
cargo run --bin asl -- run "../examples/git-conventional-commit.skill" --input '{"intent": "feat: test"}'
cargo run --bin asl -- compile-grammar "../examples/git-conventional-commit.skill" --format gbnf

# Execução do Guardrail Completo
../scripts/guardrail_check.sh
```

---

## 🔒 Leis Arquiteturais Invioláveis para o Claude

1. **Arquitetura Hexagonal Estrita**:
   - `crates/asl-spec`: Camada de domínio pura ($0$ deps de IO/rede).
   - `crates/asl-core-traits`: Portas abstratas (Traits).
   - Adaptadores (`asl-parser`, `asl-security`, `asl-vm-starlark`, `asl-protocol-mcp`): Implementam traits.
   - **PROIBIDO**: Fazer um adaptador depender de outro adaptador. O `asl-cli` faz a injeção de dependências.
2. **Atomicidade do .skill**:
   - NUNCA crie diretórios `scripts/` ou scripts em Python/Bash. O arquivo `.skill` deve conter $100\%$ de sua lógica determinística em Starlark embutido.
3. **Imunidade a Panics na C-ABI**:
   - Qualquer código na camada FFI ou Starlark deve conter barreiras `std::panic::catch_unwind`. Panics de Rust não podem vazar para chamadores externos.
4. **Limite Cognitivo por Arquivo**:
   - Nenhum arquivo `.rs` deve ultrapassar 400 linhas.
5. **Prefixo Estático Imutável**:
   - Em arquivos `.skill`, a seção semântica e os metadados YAML devem ser estáticos para 100% de reuso de KV-Cache.
6. **Spec-Driven Development Obrigatório (SDD)**:
   - Proibido implementar código sem antes redigir o ADR em `docs/adrs/` (skill `asl-adr`) e o plano em fases em `docs/plans/` (skill `asl-plan`). Toda fase deve encerrar com `guardrail_check.sh`, commit e push na main.
