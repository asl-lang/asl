# Plano de Implementação: Daemon Universal Multiplataforma e Parser Tolerante para Zero-Touch Ingestion

- **ADR Vinculado**: `docs/adrs/0015-daemon-universal-multiplataforma-e-parser-tolerante-zero-touch.md`
- **Data**: 2026-09-18
- **Responsável**: Jean Catarina & Antigravity (IA)
- **Status**: Concluído (5/5 Fases - 100%)
- **Meta**: Garantir que `touch <nome>.skill` ou `mv <legado>.md <nome>.skill` gere imediatamente a projeção sombra `<nome>.md` em 100% dos sistemas operacionais (macOS, Linux Ubuntu/Debian/Arch/RPi, Windows) sem intervenção manual do usuário.

---

## Fase 1: Parser Tolerante a Greenfield e Migração Legada (`asl-spec` e `asl-parser`)

### 1.1 Objetivo da Fase
Permitir que o parser processe:
1. Arquivos de zero bytes (`touch arquivo.skill`) sintetizando um rascunho canônico (*Draft Scaffold*).
2. Arquivos de skills legadas sem `asl_version` ou `interface:`, aplicando valores padrão canônicos (`asl_version: "3.0"`, `entrypoint: "run"`).

### 1.2 Código a Implementar
Em `runtime/crates/asl-spec/src/lib.rs`:
```rust
fn default_asl_version() -> String {
    "3.0".to_string()
}

fn default_interface() -> SkillInterface {
    SkillInterface {
        entrypoint: "run".to_string(),
        input_schema: serde_json::json!({}),
        output_schema: serde_json::json!({}),
    }
}

// Atualizar SkillManifest com serde(default)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    #[serde(default = "default_asl_version")]
    pub asl_version: String,
    // ...
    #[serde(default = "default_interface")]
    pub interface: SkillInterface,
    // ...
}
```

Em `runtime/crates/asl-parser/src/lib.rs`:
Suportar conteúdo vazio sintetizando um documento padrão de draft:
```rust
if raw_content.trim().is_empty() {
    // Greenfield 0 bytes draft
    let scaffold = SkillDocument::draft_scaffold(file_stem);
    return Ok(scaffold);
}
```

### 1.3 Verificação Local
```bash
cargo test -p asl-spec -p asl-parser
```

---

## Fase 2: Módulo de Daemon Multiplataforma no CLI (`asl-cli/src/daemon_cmds.rs`)

### 2.1 Objetivo da Fase
Criar o módulo de gerenciamento de serviços nativos do sistema operacional:
- macOS: Gerador de plist e controle via `launchctl` (`~/Library/LaunchAgents/org.asl-lang.daemon.plist`).
- Linux: Gerador de unit e controle via `systemctl --user` (`~/.config/systemd/user/asl-watcher.service`).
- Windows: Gerador de tarefa agendada via `schtasks`.

### 2.2 Código a Implementar
No arquivo `runtime/crates/asl-cli/src/daemon_cmds.rs`:
- `handle_daemon_start`: Inicia o loop de monitoramento de eventos de SO.
- `handle_daemon_install`: Cria o arquivo de serviço e registra no sistema operacional.
- `handle_daemon_status`: Verifica se o processo está em execução via PID file.
- `handle_daemon_stop`: Encerra o daemon com segurança.
- `handle_daemon_uninstall`: Remove o arquivo de serviço e para a execução.

### 2.3 Verificação Local
```bash
cargo check -p asl-cli
```

---

## Fase 3: Exposição dos Subcomandos `asl daemon` no CLI (`main.rs`)

### 3.1 Objetivo da Fase
Adicionar o comando `asl daemon <ACTION>` ao CLI:
- `asl daemon install`
- `asl daemon start`
- `asl daemon stop`
- `asl daemon status`
- `asl daemon uninstall`

### 3.2 Verificação Local
```bash
cargo run -p asl-cli -- daemon status
```

---

## Fase 4: Automação no Instalador Global (`install.sh`)

### 4.1 Objetivo da Fase
Atualizar o `install.sh` para que, ao instalar o binário `asl`, ele execute silenciosamente:
```bash
asl daemon install
```
Dessa forma, o usuário que instalou o ASL já possui o serviço rodando imediatamente no Mac/Linux/Windows.

### 4.2 Verificação Local
```bash
./scripts/check_english_only.sh
```

---

## Fase 5: Auditoria de Guardrails, Empacotamento de Release e Push na Main

### 5.1 Objetivo da Fase
1. Executar `./scripts/guardrail_check.sh`.
2. Reempacotar e atualizar releases com `./scripts/package_release.sh`.
3. Validar no macOS:
   ```bash
   touch /tmp/teste.skill
   # Verificar se /tmp/teste.md é gerado na hora
   ```
4. `git push origin main`.
