# Plano de Implementação: Suporte Nativo a Múltiplas Extensões de IA/Agentes/LLM no ASL

- **ADR Vinculado**: `docs/adrs/0011-multi-extension-ai-ecosystem.md`
- **Status**: Concluído (5/5 Fases - 100%)
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Implementar o suporte nativo e polimórfico para a família de 9 extensões canônicas de IA (`.asl`, `.agent`, `.prompt`, `.tool`, `.guard`, `.persona`, `.chain`, `.rules`, `.skill`), projeção sombra compatível, resolução de órfãos sem falsos positivos, varredura MCP e suite completa de testes e exemplos.

---

## Fase 1: Taxonomia e Reconhecimento de Extensões (`asl-spec`)

### 1.1 Objetivo da Fase
Definir a lista canônica `ASL_EXTENSIONS` e funções utilitárias `is_asl_extension` e `is_asl_file` na crate fundamental `asl-spec`, acompanhadas de testes unitários exaustivos.

### 1.2 Arquivos a Modificar
- `runtime/crates/asl-spec/src/lib.rs`

### 1.3 Verificação Local
```bash
cargo check -p asl-spec
cargo test -p asl-spec
cargo clippy -p asl-spec -- -D warnings
```

### 1.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-spec
git commit -m "feat(spec): definir familia de extensoes nativas do ecossistema ASL (Fase 1)"
git push origin main
```

---

## Fase 2: Projeção Sombra e Detecção de Órfãos Multi-Extensão (`asl-parser`)

### 2.1 Objetivo da Fase
Adaptar `asl-parser/src/shadow.rs` para projetar arquivos `.md` a partir de qualquer uma das extensões suportadas (incluindo tratamento de nomes canônicos em maiúsculas como `AGENT.agent` -> `AGENT.md`) e atualizar a detecção de arquivos órfãos em `clean_orphaned_shadows` para inspecionar todas as extensões válidas.

### 2.2 Arquivos a Modificar
- `runtime/crates/asl-parser/src/shadow.rs`

### 2.3 Verificação Local
```bash
cargo check -p asl-parser
cargo test -p asl-parser
cargo clippy -p asl-parser -- -D warnings
```

### 2.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-parser
git commit -m "feat(parser): suporte a projecao sombra e limpeza de orfaos para multiplas extensoes (Fase 2)"
git push origin main
```

---

## Fase 3: Tooling da CLI, Servidor MCP e Comandos Sombra (`asl-cli`)

### 3.1 Objetivo da Fase
Atualizar a descoberta recursiva de arquivos em `main.rs` (`load_skills_recursive`) e os subcomandos `sync-shadows` e `watch` em `shadow_cmds.rs` para consumir qualquer arquivo do ecossistema ASL, além de atualizar descrições no `clap`.

### 3.2 Arquivos a Modificar
- `runtime/crates/asl-cli/src/main.rs`
- `runtime/crates/asl-cli/src/shadow_cmds.rs`

### 3.3 Verificação Local
```bash
cargo check -p asl-cli
cargo test -p asl-cli
cargo clippy -p asl-cli -- -D warnings
```

### 3.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-cli
git commit -m "feat(cli): integrar reconhecimento multi-extensao no runner, mcp e shadow sync (Fase 3)"
git push origin main
```

---

## Fase 4: Exemplos Canônicos de IA e Atualização do Script de Guardrails

### 4.1 Objetivo da Fase
Criar arquivos canônicos demonstrando o uso de extensões especializadas de IA (`examples/security-validator.guard`, `examples/code-reviewer.agent`, `examples/summarizer.prompt`), calcular seus digests e sincronizar sombras, e atualizar `scripts/guardrail_check.sh` para auditar todas as extensões canônicas em `examples/`.

### 4.2 Arquivos a Criar/Modificar
- `examples/security-validator.guard`
- `examples/code-reviewer.agent`
- `examples/summarizer.prompt`
- `scripts/guardrail_check.sh`

### 4.3 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 4.4 Finalização da Fase (Commit & Push)
```bash
git add examples/ scripts/guardrail_check.sh
git commit -m "feat(examples): adicionar artefatos canonicos .guard, .agent e .prompt (Fase 4)"
git push origin main
```

---

## Fase 5: Testes de Integração de Guardrail e Conclusão

### 5.1 Objetivo da Fase
Adicionar testes arquiteturais em `architectural_guardrails.rs` validando parsing, execução e sombras para toda a família de extensões, atualizar `README.md`, recompilar o binário global e validar 100% de conformidade.

### 5.2 Arquivos a Modificar
- `runtime/crates/asl-cli/tests/architectural_guardrails.rs`
- `docs/plans/README.md`
- `docs/plans/0011-multi-extension-ai-ecosystem.md`

### 5.3 Verificação Local
```bash
./scripts/guardrail_check.sh
cargo install --path runtime/crates/asl-cli --force
```

### 5.4 Finalização da Fase (Commit & Push)
```bash
git add runtime/crates/asl-cli/tests/ docs/plans/
git commit -m "feat(guardrails): validar conformidade total do ecossistema multi-extensao (Fase 5)"
git push origin main
```
