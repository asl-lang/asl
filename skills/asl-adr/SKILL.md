---
name: asl-adr
description: >-
  Orienta e padroniza a criação de Registros de Decisão de Arquitetura (ADRs)
  na pasta docs/adrs/. Toda proposta técnica de arquitetura, protocolo, crate
  ou subsistema deve ter sua proposta detalhada documentada em docs/adrs/
  antes de qualquer plano de implementação ou escrita de código.
---

# Skill: Criação e Gestão de ADRs (Architectural Decision Records)

Esta skill define o processo obrigatório para elaboração de propostas arquiteturais detalhadas na pasta `docs/adrs/`.

## 📌 Quando Utilizar Esta Skill

Toda IA ou desenvolvedor DEVE redigir um ADR em `docs/adrs/` sempre que:
1. Propor uma nova micro-crate ou adaptador (ex: `asl-vm-wasm`, `asl-transport-http`).
2. Modificar ou adicionar traits em `asl-core-traits` (Ports).
3. Alterar estruturas da camada de domínio em `asl-spec`.
4. Mudar decisões de transporte, isolamento de segurança (ocap) ou sandboxing.
5. Alterar a sintaxe ou formato do arquivo `.skill`.

---

## 📁 Estrutura e Convenção de Nomenclatura

Todos os ADRs devem ser armazenados em:
```
docs/adrs/
├── README.md                          # Índice consolidado de todos os ADRs
├── 0001-hexagonal-ports-adapters.md   # Exemplo canônico
└── NNNN-<titulo-em-kebab-case>.md     # Novos ADRs numerados sequencialmente
```

O formato do arquivo deve ser `NNNN-<nome-curto>.md`, onde `NNNN` é um número sequencial de 4 dígitos (ex: `0002-wasm-engine-adapter.md`).

---

## 📝 Conteúdo Obrigatório do ADR

A pasta `docs/adrs/` deve conter a **proposta detalhada da decisão**, abordando os seguintes tópicos obrigatórios:

1. **Status e Metadados**: Data, Autor/Agente, Status (`Proposto`, `Aceito`, `Rejeitado`, `Substituído por ADR-XXXX`).
2. **Contexto e Declaração do Problema**: Qual problema estamos resolvendo? Quais são as dores da arquitetura atual? Quais forças e restrições técnicas existem?
3. **Proposta Detalhada da Arquitetura**:
   - Definição formal da solução proposta.
   - Assinaturas de tipos, structs e interfaces (Traits).
   - Diagrama de fluxo ou dependências (ASCII ou Mermaid).
   - Relação com as micro-crates existentes.
4. **Alternativas Consideradas e Trade-offs**:
   - Pelo menos duas alternativas analisadas com seus pontos fortes e fracos.
   - Justificativa clara do motivo da escolha da solução proposta.
5. **Consequências Positivas e Negativas**:
   - O que se ganha (latência, simplicidade, segurança)?
   - O que se paga (complexidade adicional, curva de aprendizado)?
6. **Aderência aos 7 Axiomas do ASL**:
   - Verificação explícita contra atomicidade, ausência de dependências externas, ocap, acoplamento hexagonal, etc.

---

## 📋 Template Canônico de ADR

Ao criar um novo ADR em `docs/adrs/`, utilize esta estrutura de base:

````markdown
# ADR-NNNN: <Título Claro da Decisão Arquitetural>

- **Status**: Proposto | Aceito | Depreciado | Substituído
- **Data**: AAAA-MM-DD
- **Autores**: <Nome ou Agente IA>
- **Decisores**: Conselho de Arquitetura ASL / Equipe
- **Crates Afetadas**: `asl-core-traits`, `asl-parser`, etc.

## 1. Contexto e Problema
<Descreva o cenário técnico atual e os gaps que motivam esta decisão.>

## 2. Proposta Detalhada da Decisão
<Apresente exaustivamente o desenho da solução proposta, incluindo structs, traits e fluxo de dados.>

```rust
// Exemplo das novas interfaces propostas
pub trait NewCapabilityPort: Send + Sync {
    fn execute_op(&self, input: &Value) -> Result<ExecutionResult>;
}
```

## 3. Alternativas Consideradas
- **Alternativa A**: <Descrição e por que foi descartada>
- **Alternativa B**: <Descrição e por que foi descartada>

## 4. Consequências e Trade-offs
- **Positivas**: <Benefícios diretos>
- **Negativas / Riscos**: <Custos ou mitigações necessárias>

## 5. Conformidade com os 7 Axiomas do ASL
- [x] Axioma 1 (Atomicidade do .skill preservada)
- [x] Axioma 2 (Zero dependências externas mantido)
- [x] Axioma 3 (Confinamento ocap sem autoridade ambiente)
- [x] Axioma 4 (Isolamento hexagonal sem import cruzado de adaptadores)
- [x] Axioma 5 (Término determinístico com Fuel Metering)
- [x] Axioma 6 (Prefixo estático imutável mantido)
- [x] Axioma 7 (Limite de < 400 linhas por arquivo respeitado)
````

---

## 🔄 Ciclo de Vida: Do ADR para o Plano

1. **Elaborar ADR** em `docs/adrs/` com a proposta técnica detalhada.
2. **Obter aprovação / consenso** de arquitetura.
3. **Mudar status para `Aceito`**.
4. **Criar o Plano de Implementação** em `docs/plans/` utilizando a skill `asl-plan`.
