# GEMINI.md: Regras Hierárquicas e Guardrails para o Google Antigravity / Gemini

## Guardrails Obrigatórios para o Projeto ASL

Toda ação de edição, refatoração ou extensão neste repositório DEVE satisfazer os seguintes critérios formais:

1. **Adesão à Arquitetura Hexagonal (ARCHITECTURE.md)**:
   - Toda nova funcionalidade deve ser implementada como um **Port** em `asl-core-traits` ou como um **Adaptador** isolado em sua própria micro-crate.
   - Nenhuma micro-crate adaptadora pode importar outra crate adaptadora.
2. **Conformidade Científica (ASL_SCIENTIFIC_PAPER.md)**:
   - Proibido adicionar dependências de runtime como Python, Node.js, V8 ou JVM.
   - Preservar o isolamento Object-Capability (ocap) sem autoridade ambiente.
   - Preservar o cálculo de digest canônico (SHA-256) em arquivos `.skill`.
3. **Limite Cognitivo por Arquivo**:
   - Manter todo arquivo de código abaixo de 400 linhas para garantir leitura instantânea sem truncamento de contexto.
4. **Verificação Pré-Commit**:
   - Sempre rodar `cargo test` e `./scripts/guardrail_check.sh` antes de concluir turnos de desenvolvimento.
5. **Spec-Driven Development Obrigatório (SDD)**:
   - Toda evolução técnica deve seguir estritamente o ciclo: ADR em `docs/adrs/` -> Plano com código em `docs/plans/` -> Execução fase a fase -> Guardrail limpo -> Commit e push na main.
