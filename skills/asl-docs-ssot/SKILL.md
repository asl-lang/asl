---
name: asl-docs-ssot
description: >-
  Governa a arquitetura de Spec-Driven Documentation (SDD) e Single Source of
  Truth (SSOT) no ASL. Toda alteração, adição de comando CLI, flag, parâmetro
  ou guia conceitual DEVE ser feita primariamente em docs/spec/ e sincronizada
  automaticamente para a CLI (asl-cli) e o Website (website/), garantindo
  zero-drift através de testes automatizados de contrato no CI.
---

# Skill: Spec-Driven Documentation (SDD) & Unified SSOT

Esta skill estabelece o **protocolo operacional obrigatório** para manter a documentação da CLI (`asl-cli`), as saídas para agentes de IA (`asl docs --ai`, `asl docs --json`) e o portal web oficial (`cadente-hub.github.io/asl`) 100% sincronizados a partir de uma **Fonte Única de Verdade (Single Source of Truth - SSOT)**.

---

## 🛑 Regra de Ouro do SDD

> **NUNCA modifique comandos no Clap (`main.rs`) ou crie componentes manuais no Website sem atualizar antes a especificação em `docs/spec/`.**
> Qualquer divergência entre a especificação e o código acionará falha imediata nos testes de contrato do CI.

---

## 📁 Estrutura de Arquivos da SSOT

```
docs/spec/
├── schema/
│   └── cli-command.schema.json     # Schema formal JSON Schema
├── commands.json                   # Especificação pura de todos os subcomandos e flags
├── topics.json                     # Tópicos didáticos, sintaxe e primers de IA
├── commands.yaml                   # Versão declarativa YAML dos comandos
└── topics.yaml                     # Versão declarativa YAML dos tópicos
```

---

## 🔁 Procedimento de Evolução em 4 Passos

Ao introduzir um novo subcomando, nova flag ou alterar a documentação de um comando existente:

### Passo 1: Spec-First (Atualizar a SSOT)
Abra `docs/spec/commands.json` e registre o comando ou adicione a flag seguindo o schema:
```json
{
  "name": "minha_flag",
  "short": "m",
  "long": "minha-flag",
  "type": "string",
  "required": false,
  "description": "Explicação concisa em inglês da flag"
}
```

### Passo 2: Implementar no Clap da CLI
Adicione o campo correspondente no enum `Commands` em `runtime/crates/asl-cli/src/main.rs`:
```rust
#[arg(short, long)]
minha_flag: Option<String>,
```

### Passo 3: Executar Sincronização do Website
Execute o sincronizador hermético:
```bash
python3 scripts/sync_docs_spec.py
```
Isso atualizará `website/src/data/cli-spec.json`, refletindo automaticamente os novos comandos no portal web Next.js e na paleta de busca `⌘K`.

### Passo 4: Auditar com Teste de Drift e Guardrail
Execute os testes de reflexão e conformidade:
```bash
cargo test -p asl-cli --test test_docs_ssot_drift
./scripts/guardrail_check.sh
```

Se o teste de drift passar sem erros, a documentação está matematicamente sincronizada. Finalize a fase com commit convencional e push na main.
