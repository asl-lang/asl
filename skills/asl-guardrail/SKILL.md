---
name: asl-guardrail
description: >-
  Audita e valida a conformidade estrita do repositório contra os 7 Axiomas
  Arquiteturais do ASL e os Teoremas Científicos do artigo formal. Use sempre
  antes de concluir tarefas ou submeter alterações de código.
---

# Skill: Guardrail e Auditoria de Conformidade ASL

Esta skill deve ser executada por qualquer IA que realize alterações no projeto ASL para garantir que nenhuma premissa científica ou arquitetural foi violada.

## Procedimento Passo a Passo de Auditoria

### 1. Auditoria de Limite Cognitivo de Contexto (< 400 linhas)
Nenhum arquivo `.rs` ou `.skill` pode ultrapassar 400 linhas.
```bash
# Verificação de tamanho de arquivos
find runtime/crates -name "*.rs" -exec wc -l {} + | sort -n
```
Se algum arquivo exceder 400 linhas, desmembre-o imediatamente em submódulos ortogonais.

### 2. Auditoria do Grafo Acíclico Hexagonal
Verifique se nenhum adaptador importa outro adaptador:
- `asl-parser` NÃO pode importar `asl-security`, `asl-vm-starlark` ou `asl-protocol-mcp`.
- `asl-security` NÃO pode importar `asl-parser` ou `asl-vm-starlark`.
- `asl-vm-starlark` NÃO pode importar `asl-parser` ou `asl-protocol-mcp`.
Inspecione os `Cargo.toml` em `runtime/crates/*/Cargo.toml`.

### 3. Auditoria de Pureza da Camada de Domínio (`asl-spec`)
A crate `asl-spec` deve ser $100\%$ pura.
Inspecione `runtime/crates/asl-spec/src/lib.rs` e garanta que:
- Não há referências a `std::fs`, `std::net`, `std::process`.
- Não há dependências com efeitos colaterais de I/O.

### 4. Execução dos Testes Automatizados de Arquitetura
```bash
cd runtime && cargo test
```
Todos os testes de todas as micro-crates devem retornar `test result: ok`.

### 5. Auditoria de Validação de Arquivos Canônicos da Tríade
```bash
for ext in skill tool asl; do
    for f in ../examples/*.${ext}; do
        [ -f "$f" ] && cargo run --bin asl -- check "$f"
    done
done
```
O digest de cada arquivo deve bater bit-a-bit com o digest declarado no frontmatter YAML.

### 6. Execução Rápida Automatizada (All-in-One)
A partir da raiz do projeto (`agent skill language/`):
```bash
./scripts/guardrail_check.sh
```
Executa atomicamente todas as 5 etapas acima, garantindo que o PR ou modificação é $100\%$ aprovado.
