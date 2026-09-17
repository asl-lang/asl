# Plano de Implementação: Compilador AOT de Gramáticas de Amostragem LLM (GBNF / Regex-CFG)

- **ADR Vinculado**: `docs/adrs/0003-aot-grammar-compiler-gbnf-regex.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Transformar esquemas JSON declarados em `.skill` em gramáticas formais compiladas antecipadamente (GBNF para llama.cpp/Ollama e Regex DFA para vLLM/SGLang/Outlines), com 0% de erro sintático na amostragem de tokens.

---

## Fase 1: Especificação Arquitetural e Plano Canônico

### 1.1 Objetivo da Fase
Formalizar o ADR-0003 e o Plano 0003 com índices devidamente atualizados, garantindo conformidade com o ciclo de Spec-Driven Development (SDD).

### 1.2 Verificação Local
```bash
git status
```

### 1.3 Finalização da Fase (Commit & Push)
```bash
git add docs/adrs/ docs/plans/
git commit -m "docs(spec): formalizar ADR-0003 e PLAN-0003 do compilador AOT de gramaticas"
git push origin main
```

---

## Fase 2: Módulo Compilador GBNF (`asl-parser::grammar::gbnf`)

### 2.1 Objetivo da Fase
Implementar a geração formal de gramáticas GBNF (GGML BNF) para *llama.cpp* e *Ollama* a partir de nós JSON Schema arbitrários.

### 2.2 Código a Implementar
No arquivo `runtime/crates/asl-parser/src/grammar/gbnf.rs`:
```rust
use asl_spec::{AslError, Result};
use serde_json::Value;

pub fn compile_schema_to_gbnf(schema: &Value) -> Result<String> {
    let mut rules = Vec::new();
    let mut rule_counter = 0;
    let root_rule = compile_node(schema, "root", &mut rules, &mut rule_counter)?;
    
    let mut out = String::new();
    out.push_str(&format!("root ::= {}\n", root_rule));
    for (name, expr) in rules {
        out.push_str(&format!("{} ::= {}\n", name, expr));
    }
    out.push_str("ws ::= [ \\t\\n\\r]*\n");
    out.push_str("string ::= \"\\\"\" ([^\"\\\\] | \"\\\\\" .)* \"\\\"\"\n");
    out.push_str("number ::= \"-\"? [0-9]+ (\".\" [0-9]+)? ([eE] [-+]? [0-9]+)?\n");
    out.push_str("integer ::= \"-\"? [0-9]+\n");
    out.push_str("boolean ::= \"true\" | \"false\"\n");
    out.push_str("null ::= \"null\"\n");
    Ok(out)
}
```

### 2.3 Testes Unitários da Fase
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_gbnf_enum() {
        let schema = json!({
            "type": "string",
            "enum": ["feat", "fix"]
        });
        let gbnf = compile_schema_to_gbnf(&schema).unwrap();
        assert!(gbnf.contains("\"feat\" | \"fix\"") || gbnf.contains("\"fix\" | \"feat\""));
    }

    #[test]
    fn test_compile_gbnf_object() {
        let schema = json!({
            "type": "object",
            "required": ["intent"],
            "properties": {
                "intent": { "type": "string" }
            }
        });
        let gbnf = compile_schema_to_gbnf(&schema).unwrap();
        assert!(gbnf.contains("\"intent\""));
    }
}
```

### 2.4 Verificação Local
```bash
cargo test -p asl-parser
cargo clippy -p asl-parser -- -D warnings
```

### 2.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser/src/grammar/gbnf.rs
git commit -m "feat(parser): implementar gerador AOT de gramatica GBNF para llama.cpp"
git push origin main
```

---

## Fase 3: Módulo Compilador Regex / DFA (`asl-parser::grammar::regex_cfg`)

### 3.1 Objetivo da Fase
Implementar a geração formal de expressões regulares estruturadas para motores DFA (*vLLM*, *SGLang*, *Outlines*).

### 3.2 Código a Implementar
No arquivo `runtime/crates/asl-parser/src/grammar/regex_cfg.rs`:
```rust
use asl_spec::{AslError, Result};
use serde_json::Value;

pub fn compile_schema_to_regex(schema: &Value) -> Result<String> {
    compile_regex_node(schema)
}
```

### 3.3 Testes Unitários da Fase
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_regex_object() {
        let schema = json!({
            "type": "object",
            "required": ["status"],
            "properties": {
                "status": { "type": "string", "enum": ["ok", "err"] }
            }
        });
        let regex = compile_schema_to_regex(&schema).unwrap();
        assert!(regex.contains("\"status\""));
        assert!(regex.contains("(\"ok\"|\"err\")"));
    }
}
```

### 3.4 Verificação Local
```bash
cargo test -p asl-parser
cargo clippy -p asl-parser -- -D warnings
```

### 3.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser/src/grammar/regex_cfg.rs
git commit -m "feat(parser): implementar gerador AOT de expressoes regulares CFG para vLLM"
git push origin main
```

---

## Fase 4: Integração no Adaptador `asl-parser`, CLI e Validação de Guardrails

### 4.1 Objetivo da Fase
Conectar os compiladores em `GbnfGrammarCompiler` (implementando `GrammarCompilerPort`), expor via CLI `asl compile-grammar` e rodar a suíte completa de guardrails.

### 4.2 Verificação Local
```bash
./scripts/guardrail_check.sh
cargo run --bin asl -- compile-grammar examples/git-conventional-commit.skill --format gbnf
cargo run --bin asl -- compile-grammar examples/git-conventional-commit.skill --format regex
```

### 4.3 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser/ docs/plans/
git commit -m "feat(cli): integrar compilador de gramatica AOT no CLI e validar guardrails"
git push origin main
```
