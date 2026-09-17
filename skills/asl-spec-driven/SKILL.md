---
name: asl-spec-driven
description: >-
  Meta-skill obrigatória que rege o protocolo de Desenvolvimento Orientado a
  Especificação (Spec-Driven Development - SDD) no projeto ASL. Toda solicitação
  de funcionalidade, refatoração ou evolução de runtime deve seguir estritamente
  o ciclo: 1) ADR em docs/adrs/, 2) Plano em docs/plans/, 3) Implementação em
  fases atômicas com código, 4) Verificação de guardrail e 5) Commit e push na main.
---

# Skill: Spec-Driven Development (SDD) - O Ciclo Obrigatório do ASL

Esta meta-skill estabelece o **protocolo de execução rígido e inquebrável** para qualquer agente de IA ou desenvolvedor operando no projeto ASL.

> 🛑 **REGRA DE OURO**: É terminantemente proibido pular direto para o código. Nenhuma alteração estrutural ou funcionalidade pode ser implementada sem que a especificação (ADR) e o plano detalhado (PLAN) estejam formalizados e comitados.

---

## 🔁 O Ciclo Universal de 5 Etapas

```
┌─────────────────────────────────────────────────────────────────────────┐
│              FLUXO SPEC-DRIVEN DEVELOPMENT (SDD) DO ASL                 │
├─────────────────────────────────────────────────────────────────────────┤
│ 1. ESPECIFICAÇÃO DETALHADA (ADR) ──► docs/adrs/NNNN-<nome>.md           │
│    Usa skill `asl-adr`: Problema, Desenho, Contratos, Trade-offs        │
├─────────────────────────────────────────────────────────────────────────┤
│ 2. PLANO EXAUSTIVO EM FASES      ──► docs/plans/NNNN-<nome>.md          │
│    Usa skill `asl-plan`: Fases atômicas, exemplos reais de código       │
├─────────────────────────────────────────────────────────────────────────┤
│ 3. IMPLEMENTAÇÃO DA FASE         ──► runtime/crates/<crate>/            │
│    Escrever o código exato da fase e seus testes mais próximos          │
├─────────────────────────────────────────────────────────────────────────┤
│ 4. VERIFICAÇÃO AUTOMATIZADA      ──► ./scripts/guardrail_check.sh       │
│    Garantir 0 erros, 0 clippy warnings, < 400 linhas e 100% testes ok   │
├─────────────────────────────────────────────────────────────────────────┤
│ 5. ENCERRAMENTO COM COMMIT/PUSH  ──► git push origin main               │
│    Commit convencional atômico e push na main antes da próxima fase     │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Passo a Passo Operacional para a IA

Quando o usuário solicitar uma nova funcionalidade, alteração técnica ou evolução da linguagem:

### Etapa 1: Formalizar a Proposta Arquitetural (ADR)
1. Invoque a skill `asl-adr`.
2. Identifique o próximo número sequencial em `docs/adrs/` (ex: `0002`).
3. Crie `docs/adrs/NNNN-<titulo>.md` detalhando:
   - O problema concreto e motivação.
   - O desenho formal das interfaces, structs e portas hexagonais.
   - Alternativas consideradas e os trade-offs avaliados.
   - Aderência explícita aos 7 Axiomas do ASL.
4. Registre o novo ADR no índice de `docs/adrs/README.md`.

### Etapa 2: Elaborar o Plano de Execução (PLAN)
1. Invoque a skill `asl-plan`.
2. Crie `docs/plans/NNNN-<titulo>.md` vinculado ao ADR criado.
3. Divida o trabalho em **fases pequenas e atômicas**.
4. Cada fase **DEVE conter blocos reais de código** demonstrando o que será implementado (proibido usar pseudocódigo vago ou comentários genéricos).
5. Inclua os testes unitários da fase e os comandos de verificação.
6. Registre o novo plano no índice de `docs/plans/README.md`.

### Etapa 3: Executar Estritamente Uma Fase por Vez
1. Abra o arquivo do plano.
2. Implemente o código da Fase $N$.
3. Aplique as diretrizes de testes em Rust (testes unitários concisos no mesmo arquivo via `#[cfg(test)] mod tests`, ou extração adjacente se exceder tamanho).

### Etapa 4: Rodar o Guardrail Completo
```bash
./scripts/guardrail_check.sh
```
A fase só é aprovada se passar em todos os 5 estágios:
- [x] Limites cognitivos (< 450 linhas por arquivo).
- [x] Compilação do workspace.
- [x] Clippy com zero advertências (`-D warnings`).
- [x] Suíte de testes unitários e de integração ($100\%$ ok).
- [x] Integridade dos digests SHA-256 dos arquivos `.skill`.

### Etapa 5: Commit Convencional e Push na Main
```bash
git add <arquivos-alterados-na-fase>
git commit -m "<tipo>(<escopo>): <descrição no padrão Conventional Commits>"
git push origin main
```
Marque a fase como concluída `[x]` no plano antes de avançar para a próxima fase.

---

## 🛑 Exceções Permitidas ao Ciclo Completo
O ciclo ADR + PLAN só pode ser dispensado para:
- Correções cosméticas de documentação (typos, formatação markdown).
- Ajustes pontuais de comentários de código que não alterem lógica ou tipos.
- Qualquer outra solicitação que altere tipos, interfaces, crates, CLI, runtime ou gramática **EXIGE O CICLO COMPLETO**.
