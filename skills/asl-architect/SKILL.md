---
name: asl-architect
description: >-
  Guia de arquitetura para agentes de IA estenderem o runtime ASL sem violar o
  padrão Ports & Adapters (Hexagonal). Use ao adicionar novos motores de execução,
  novos parsers ou novos transportes.
---

# Skill: Extensão Arquitetural Segura do Runtime ASL

Esta skill orienta agentes autônomos na evolução e extensão do código do runtime ASL.

## Regras de Extensão Hexagonal

### Como Adicionar um Novo Motor de Execução (ex: WASM ou Lua)
1. **NÃO altere `asl-core-traits`** a menos que uma nova capacidade fundamental seja indispensável.
2. Crie uma nova micro-crate em `runtime/crates/asl-vm-<nome>`:
   ```toml
   [dependencies]
   asl-spec.workspace = true
   asl-core-traits.workspace = true
   ```
3. Implemente o trait `EnginePort`:
   ```rust
   impl EnginePort for MyNewEngine {
       fn name(&self) -> &'static str { "my-engine" }
       fn execute(...) -> Result<ExecutionResult> { ... }
   }
   ```
4. Adicione a nova crate ao `runtime/Cargo.toml` como membro do workspace.
5. Injete o novo motor no `asl-cli` através de flag ou configuração sem acoplamento.

### Como Adicionar um Novo Transporte (ex: HTTP Server)
1. Crie uma nova micro-crate em `runtime/crates/asl-transport-<nome>`.
2. Dependa exclusivamente de `asl-spec` e `asl-core-traits`.
3. Nunca importe detalhes de implementação do motor Starlark ou do parser.

### Princípio do Linter Arquitetural
Antes de finalizar qualquer PR de arquitetura, execute:
```bash
./scripts/guardrail_check.sh
```
O script falhará caso haja acoplamento circular ou arquivos excessivamente longos.
