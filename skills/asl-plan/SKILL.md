---
name: asl-plan
description: >-
  Orienta e padroniza a criação e execução de planos de implementação exaustivos
  em docs/plans/. Exige decomposição em fases pequenas e atômicas, exemplos
  concretos de código de implementação para cada etapa, critérios de validação
  e finalização obrigatória de cada fase com validação, commit convencional e push na main.
---

# Skill: Elaboração e Execução de Planos de Implementação

Esta skill define o padrão obrigatório para transformar decisões arquiteturais (ADRs) em código executado através de planos em `docs/plans/`.

## 📌 Quando Utilizar Esta Skill

Toda IA ou desenvolvedor DEVE redigir um plano em `docs/plans/` antes de iniciar qualquer ciclo de implementação técnica que envolva:
1. Implementação de um ADR aprovado em `docs/adrs/`.
2. Criação de novas crates, motores ou adaptadores.
3. Refatorações estruturais ou expansão de protocolos.
4. Adição de novos subcomandos CLI ou endpoints.

---

## 📁 Estrutura e Convenção de Nomenclatura

Todos os planos devem ser armazenados em:
```
docs/plans/
├── README.md                           # Índice e guia dos planos de implementação
├── 0001-modular-hexagonal-runtime.md   # Exemplo canônico
└── NNNN-<titulo-do-plano>.md           # Planos numerados sequencialmente
```

O formato do arquivo deve ser `NNNN-<nome-curto>.md`, onde `NNNN` é um número de 4 dígitos sincronizado com o ADR correspondente quando aplicável (ex: `0002-wasm-engine-adapter.md`).

---

## 🎯 As 4 Regras de Ouro dos Planos em `docs/plans/`

```
┌─────────────────────────────────────────────────────────────────────────┐
│                 AS 4 REGRAS DE OURO DOS PLANOS EM ASL                   │
├─────────────────────────────────────────────────────────────────────────┤
│ 1. FASES PEQUENAS E ATÔMICAS: Uma unidade lógica testável por fase      │
│ 2. CÓDIGO EXAUSTIVO: Exemplos reais de código Rust, tipos e testes      │
│ 3. VALIDAÇÃO LOCAL IMEDIATA: Comandos exatos de teste a cada fase       │
│ 4. COMMIT + PUSH MAIN NO FINAL: Cada fase encerra com push na main      │
└─────────────────────────────────────────────────────────────────────────┘
```

### Regra 1: Fases Pequenas e Atômicas
- Cada fase deve ser mínima, alterando preferencialmente um único componente ou contrato.
- Evite fases gigantescas que tocam múltiplos subsistemas simultaneamente.
- Se uma fase tocar mais de 2 arquivos complexos, divida-a em subfases (ex: Fase 1A, Fase 1B).

### Regra 2: Exemplos de Código Exaustivamente Detalhados
- O plano **NÃO PODE** conter instruções vagas como "criar a trait de conexão" ou "implementar lógica aqui".
- O plano **DEVE** conter os blocos de código concretos que serão escritos, incluindo assinaturas, imports, tratamento de erro com `Result<T, AslError>` e os testes unitários da fase.

### Regra 3: Critérios Claros de Aceite
- Cada fase deve especificar os comandos determinísticos de validação (ex: `cargo test -p asl-parser`, `cargo clippy`).
- A fase só é dada como concluída quando o terminal retornar `test result: ok` e zero warnings.

### Regra 4: Finalização Obrigatória de Cada Fase com Commit e Push na Main
- Ao concluir com sucesso uma fase:
  1. `git add <arquivos-alterados>`
  2. `git commit -m "<tipo>(<escopo>): <descrição no padrão Conventional Commits>"`
  3. `git push origin main` (ou `git push main`)
- Isso garante progresso incremental auditável, histórico limpo e checkpoints seguros.

---

## 📋 Template Canônico de Plano de Implementação

````markdown
# Plano de Implementação: <Título da Tarefa / ADR Vinculado>

- **ADR Vinculado**: `docs/adrs/ADR-NNNN-<nome>.md`
- **Data**: AAAA-MM-DD
- **Responsável**: <Nome / Agente IA>
- **Meta**: <Resumo claro do resultado esperado após todas as fases>

---

## Fase 1: <Nome da Unidade Atômica 1 - ex: Definição de Tipos e Traits>

### 1.1 Objetivo da Fase
Definir as interfaces fundamentais em `asl-core-traits` e tipos de domínio em `asl-spec`.

### 1.2 Código a Implementar (Exemplo Exaustivo)
No arquivo `runtime/crates/asl-spec/src/lib.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    pub memory_pages: u32,
    pub exported_symbol: String,
}
```

No arquivo `runtime/crates/asl-core-traits/src/lib.rs`:
```rust
pub trait WasmEnginePort: Send + Sync {
    fn load_module(&self, bytes: &[u8]) -> Result<()>;
}
```

### 1.3 Testes Unitários da Fase
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wasm_config_defaults() {
        let cfg = WasmConfig { memory_pages: 16, exported_symbol: "run".into() };
        assert_eq!(cfg.memory_pages, 16);
    }
}
```

### 1.4 Verificação Local
```bash
cargo check -p asl-spec -p asl-core-traits
cargo test -p asl-spec
cargo clippy -p asl-spec -- -D warnings
```

### 1.5 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-spec runtime/crates/asl-core-traits docs/plans/
git commit -m "feat(spec): adicionar tipos e traits para suporte wasm"
git push origin main
```

---

## Fase 2: <Nome da Unidade Atômica 2 - ex: Adaptador Concreto>
...
### Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-vm-wasm/
git commit -m "feat(vm-wasm): implementar adaptador concreto WasmEngine"
git push origin main
```
````

---

## ⚡ Protocolo de Execução para Agentes de IA

1. Abra o plano correspondente em `docs/plans/`.
2. Execute estritamente uma fase de cada vez.
3. Rode os comandos de verificação indicados.
4. Execute o commit convencional e o push na `main`.
5. Marque o checkbox da fase no plano como concluído antes de passar para a próxima.
