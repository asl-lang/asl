# Plano de Implementação: Transpilador Semântico Declarativo para Starlark Hermético (ASL Rules)

- **ADR Vinculado**: `docs/adrs/0010-declarative-semantic-rules-transpiler.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Implementar na íntegra a sintaxe semântica declarativa `asl:rules`, seu parser EBNF, gerador Strict Starlark L1 com verificação AOT, integração in-memory no runtime sem geração de arquivos lixo no disco, subcomando `asl expand` e suite exaustiva de testes.

---

## Fase 1: Portas e Modelos de Domínio (`asl-spec` e `asl-core-traits`)

### 1.1 Objetivo da Fase
Expandir `asl-spec` com o campo `rules_code: Option<String>` em `SkillDocument` e definir a porta `RulesTranspilerPort` com tipos de resultado, source map e erros em `asl-core-traits`.

### 1.2 Arquivos a Modificar
- `runtime/crates/asl-spec/src/lib.rs`
- `runtime/crates/asl-core-traits/src/lib.rs`

### 1.3 Verificação Local
```bash
cargo check -p asl-spec -p asl-core-traits
cargo test -p asl-spec -p asl-core-traits
cargo clippy -p asl-spec -p asl-core-traits -- -D warnings
```

### 1.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-spec runtime/crates/asl-core-traits
git commit -m "feat(spec): adicionar portas e tipos do transpilador de regras"
git push origin main
```

---

## Fase 2: Lexer, Parser EBNF e AST de Regras (`asl-parser::rules`)

### 2.1 Objetivo da Fase
Implementar o módulo `rules` no `asl-parser`:
- `ast.rs`: Estrutura de dados da AST (`RulesBlock`, `GuardClause`, `MatchSection`, `WhenClause`, etc.).
- `lexer.rs`: Tokenizador com normalização de indentação (passos de 2 espaços) e rastreamento de linha/coluna.
- `parser.rs`: Analisador sintático recursivo descendente formal EBNF.

### 2.2 Arquivos a Criar/Modificar
- `runtime/crates/asl-parser/src/rules/ast.rs`
- `runtime/crates/asl-parser/src/rules/lexer.rs`
- `runtime/crates/asl-parser/src/rules/parser.rs`
- `runtime/crates/asl-parser/src/rules/mod.rs`
- `runtime/crates/asl-parser/src/lib.rs`

### 2.3 Verificação Local
```bash
cargo test -p asl-parser --lib rules
cargo clippy -p asl-parser -- -D warnings
```

### 2.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser
git commit -m "feat(parser): implementar lexer e parser EBNF para asl:rules"
git push origin main
```

---

## Fase 3: Gerador Strict Starlark L1 e Verificação Pré-Voo AOT (`asl-parser::rules::transpiler`)

### 3.1 Objetivo da Fase
Implementar a geração de código Starlark L1 hermético:
- Preâmbulo universal puro: `_asl_get`, `_asl_contains_any`, `_asl_starts_with_any`, `_asl_ends_with_any`.
- Navegação imune a nulo e serialização RFC 8259 via `serde_json::to_string()`.
- Verificação de exaustividade de padrões (obrigatoriedade de `otherwise:` ou cobertura total).
- Verificação estática de tipos contra o `output_schema`.
- Validação em memória via `starlark::syntax::AstModule::parse`.
- Geração de Source Maps bidirecionais.

### 3.2 Arquivos a Criar/Modificar
- `runtime/crates/asl-parser/src/rules/transpiler.rs`
- `runtime/crates/asl-parser/src/rules/mod.rs`

### 3.3 Verificação Local
```bash
cargo test -p asl-parser
cargo clippy -p asl-parser -- -D warnings
```

### 3.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser
git commit -m "feat(parser): implementar gerador Strict Starlark L1 com AOT"
git push origin main
```

---

## Fase 4: Integração de Toque Zero no Parser Principal (`CommonMarkYamlParser`)

### 4.1 Objetivo da Fase
Atualizar `CommonMarkYamlParser` em `asl-parser/src/lib.rs` para:
- Detectar blocos `asl:rules`.
- Disparar transpilação JIT 100% in-memory para `deterministic_code` sem tocar no disco.
- Se houver `asl:rules` E `asl:deterministic`, acoplar validação de guardas antes da chamada da função determinística.

### 4.2 Verificação Local
```bash
cargo test -p asl-parser
cargo clippy -p asl-parser -- -D warnings
```

### 4.3 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser
git commit -m "feat(parser): integrar transpilacao JIT in-memory no CommonMarkYamlParser"
git push origin main
```

---

## Fase 5: Ferramental CLI (`asl expand`) e Suporte no `asl run` / `asl check`

### 5.1 Objetivo da Fase
Expandir a CLI em `asl-cli`:
- Adicionar comando `asl expand <skill>` para inspecionar o Starlark gerado e mapa de origem no terminal.
- Atualizar `asl check` para auditar a integridade das regras semânticas declarativas.
- Atualizar `asl run` para executar skills baseadas em regras de forma transparente.

### 5.2 Arquivos a Criar/Modificar
- `runtime/crates/asl-cli/src/main.rs`
- `runtime/crates/asl-cli/src/commands/expand.rs` (ou equivalente)

### 5.3 Verificação Local
```bash
cargo run --bin asl -- expand examples/git-conventional-commit.skill
cargo test -p asl-cli
cargo clippy --all-targets --all-features -- -D warnings
```

### 5.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-cli
git commit -m "feat(cli): adicionar subcomando asl expand e suporte a regras no run/check"
git push origin main
```

---

## Fase 6: Exemplo Canônico, Testes End-to-End e Guardrails

### 6.1 Objetivo da Fase
- Criar exemplo canônico demonstrativo: `examples/conventional-commit-rules.skill` com sintaxe declarativa limpa.
- Adicionar testes de integração de arquitetura cobrindo todos os edge cases de variáveis, injeção e Starlark universal.
- Executar `./scripts/guardrail_check.sh` garantindo 100% de conformidade nos 5 níveis.

### 6.2 Verificação Local e Global
```bash
./scripts/guardrail_check.sh
```

### 6.3 Finalização da Fase (Commit & Push)
```bash
git add examples/ runtime/crates/asl-cli/tests/ docs/plans/
git commit -m "feat(rules): exemplo canonico, testes e conformidade total de guardrails"
git push origin main
```
