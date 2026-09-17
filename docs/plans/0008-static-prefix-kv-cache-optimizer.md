# Plano de Implementação: Otimizador de Prefixo Estático e Analisador de KV-Cache

- **ADR Vinculado**: `docs/adrs/0008-static-prefix-kv-cache-optimizer.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (4/4 Fases - 100%)
- **Meta**: Desenvolver o analisador estático de KV-Cache e otimizador de prefixo invariante em `asl-parser` e comandos CLI correspondentes no `asl-cli` (Marco 5).

---

## Fase 1: Módulo Analisador de Prefixo (`asl-parser::prefix_analyzer`)

### 1.1 Objetivo da Fase
Criar o módulo `prefix_analyzer.rs` em `asl-parser` capaz de:
- Identificar marcadores de variáveis dinâmicas (`{{...}}`, `${...}`, `{...}`).
- Dividir a seção semântica em blocos estáticos e dinâmicos.
- Calcular:
  - Tamanho em caracteres do prefixo estático contíguo inicial.
  - Tamanho em caracteres do corpo semântico total.
  - Estimativa de tokens do prefixo estático (~4 chars/token).
  - Taxa projetada de acerto no KV-cache (`static_len / total_len * 100`).
  - Lista de avisos / riscos de invalidação de cache (ex.: variáveis encontradas no primeiro parágrafo).

### 1.2 Código a Implementar
`runtime/crates/asl-parser/src/prefix_analyzer.rs`:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixAnalysisReport {
    pub total_semantic_chars: usize,
    pub static_prefix_chars: usize,
    pub total_estimated_tokens: usize,
    pub static_estimated_tokens: usize,
    pub projected_cache_hit_rate_pct: f64,
    pub dynamic_variables: Vec<String>,
    pub cache_invalidation_hazards: Vec<String>,
}

pub fn analyze_semantic_prefix(semantic_text: &str) -> PrefixAnalysisReport;
```

### 1.3 Testes Unitários
- Teste com texto puramente estático (100% hit-rate).
- Teste com texto contendo variáveis dinâmicas no final (alto hit-rate).
- Teste com variáveis dinâmicas no início (baixo hit-rate e hazards detectados).

---

## Fase 2: Otimizador de Prefixo (`optimize_prefix`)

### 2.1 Objetivo da Fase
Criar a rotina de reordenação estrutural do prompt que:
- Detecta parágrafos e seções que contêm interpolações dinâmicas.
- Move o bloco estático consolidado para o topo do prompt semântico.
- Anexa as variáveis dinâmicas em uma seção final de parâmetros (`## Dynamic Context & Parameters`), maximizando o prefixo estático contínuo.

### 2.2 Código a Implementar
`runtime/crates/asl-parser/src/prefix_analyzer.rs`:
```rust
pub fn optimize_semantic_prefix(semantic_text: &str) -> String;
```

---

## Fase 3: Comandos CLI `analyze-prefix` e `optimize-prefix`

### 3.1 Objetivo da Fase
Integrar os novos comandos no `asl-cli`:
- `asl analyze-prefix <skill_file>`: Imprime relatório detalhado e formatado de KV-cache.
- `asl optimize-prefix <skill_file> [--in-place]`: Otimiza a seção semântica e imprime ou atualiza o arquivo.
- Refatorar a CLI para manter todos os arquivos Rust < 450 linhas (Axioma 2).

### 3.2 Verificação Local
```bash
cargo test -p asl-parser
cargo test -p asl-cli
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Fase 4: Validação de Guardrails e Fechamento do Roadmap

### 4.1 Objetivo da Fase
Executar a suíte de guardrails completa `./scripts/guardrail_check.sh`.

### 4.2 Finalização (Commit & Push)
```bash
git add docs/ runtime/
git commit -m "feat(parser): implementar analisador de KV-cache e otimizador de prefixo (Marco 5)"
git push origin main
```
