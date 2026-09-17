# Plano de Implementação: Projeção Sombra Automática de Markdown (Shadow Projection)

- **ADR Vinculado**: `docs/adrs/0009-automatic-markdown-shadow-projection.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (4/4 Fases - 100%)
- **Meta**: Implementar o motor de projeção sombra automática (`.skill` -> `.md`), cobrindo criação do zero, migração suave e a totalidade dos 11 edge cases do ADR-0009.

---

## Fase 1: Suporte a Skills Puras de Prompt e Flexibilização no Parser (`asl-parser`)

### 1.1 Objetivo da Fase
Permitir que arquivos `.skill` puramente semânticos (sem bloco determinístico `asl`, EC-10) sejam analisados com sucesso, injetando um código padrão de identidade (`def run(ctx, input): return input`).

### 1.2 Código a Implementar
Em `runtime/crates/asl-parser/src/lib.rs`:
Se `deterministic_code.trim().is_empty()`, definir código padrão em vez de retornar erro fatal:
```rust
let code = if deterministic_code.trim().is_empty() {
    "def run(ctx, input):\n    return input".to_string()
} else {
    deterministic_code
};
```

### 1.3 Verificação Local
```bash
cargo test -p asl-parser
```

---

## Fase 2: Motor de Projeção Sombra (`asl-parser::shadow`)

### 2.1 Objetivo da Fase
Criar o módulo `runtime/crates/asl-parser/src/shadow.rs` com:
- `generate_shadow_content(doc: &SkillDocument, skill_file_name: &str) -> String`: Monta o Markdown sombra com carimbo de digest.
- `project_shadow_markdown(skill_path: &Path, doc: &SkillDocument) -> Result<ShadowProjectResult>`:
  - Verifica se o `.md` já existe e tem o mesmo digest (No-op / loop prevention).
  - Protege arquivos `.md` pré-existentes que não tenham a marcação do ASL (EC-4).
  - Escreve atomicamente usando `.tmp` e `rename`.
- `clean_orphaned_shadows(dir: &Path) -> Result<usize>`:
  - Remove arquivos `.md` sombra cujos `.skill` originais foram apagados (EC-2).

### 2.2 Testes Unitários da Fase
- Geração de `.md` a partir de `.skill` novo (EC-1).
- Idempotência: não reescrever se o digest não mudou.
- Atualização quando a descrição ou regras mudarem.
- Remoção de `.md` órfão quando `.skill` é excluído (EC-2).
- Proteção de colisão com `.md` manual legítimo (EC-4).

---

## Fase 3: Integração no CLI e Sincronização Automática (`asl-cli`)

### 3.1 Objetivo da Fase
Criar `runtime/crates/asl-cli/src/shadow_cmds.rs` e plugar no `asl-cli`:
- `asl sync-shadows [path]`: Sincroniza todas as sombras de um diretório ou workspace.
- Hooks automáticos:
  - `asl check <skill>`: Garante que o `.md` sombra correspondente esteja gerado e atualizado.
  - `asl run <skill>`: Garante a presença do `.md` sombra.
  - `asl sign <skill>`: Atualiza o digest e assinatura no `.md` sombra.
- Comando `asl watch [path]`: Monitora o diretório e projeta as sombras em tempo real.

### 3.2 Verificação Local
```bash
cargo check -p asl-cli
cargo test --workspace
```

---

## Fase 4: Validação de Guardrails e Finalização

### 4.1 Objetivo da Fase
Executar o script oficial de guardrails `./scripts/guardrail_check.sh`.

### 4.2 Finalização (Commit & Push)
```bash
git add docs/ runtime/
git commit -m "feat(shadow): implementar motor de projecao sombra automatica de markdown (Marco 6)"
git push origin main
```
