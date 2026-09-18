# ADR-0015: Daemon Universal Multiplataforma de FSEvents e Parser Tolerante para Zero-Touch Ingestion

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-parser`, `asl-spec`, `asl-cli`, `install.sh`, Serviços de Sistema (`launchd`, `systemd`, Windows Tasks)
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 3 (OCAP Restrito), Axioma 5 (Término Determinístico)

---

## 1. Contexto e Declaração do Problema

O **ADR-0009** definiu a promessa arquitetural de **Adoção de Toque Zero (*Zero-Touch Ingestion*)**: sempre que um desenvolvedor ou agente criar um arquivo `.skill` do zero (via `touch`, editor ou IA) ou renomear um arquivo `.md` legado para `.skill` (`mv foo.md foo.skill`), o sistema deve projetar e manter automaticamente o arquivo sombra correspondente `foo.md` no mesmo diretório em menos de 1 milissegundo, **sem exigir a invocação manual de nenhum comando CLI**.

No entanto, testes práticos em ambientes reais (como macOS Apple Silicon M1–M5 e distribuições Linux) revelaram **três quebras críticas de conformidade**:

1. **Ausência de Serviço Daemon no Sistema Operacional**:
   O comando `touch` ou `mv` é uma operação nativa do kernel do SO. Sem um serviço de background ouvindo os eventos do sistema de arquivos (`FSEvents` no macOS, `inotify` no Linux, `ReadDirectoryChangesW` no Windows), o sistema operacional não despacha notificações para o binário `asl`. O ASL possuía apenas o comando em primeiro plano `asl watch`, que exigia que o usuário mantivesse um terminal aberto manualmente.
2. **Crash do Parser em Arquivos Greenfield de 0 Bytes**:
   O comando canônico do Unix `touch nova-skill.skill` cria um arquivo vazio de exatamente zero bytes. O `CommonMarkYamlParser` tratava arquivos sem delimitadores `---` como erro fatal (`Frontmatter invalid: YAML frontmatter was not closed with '---'`), impedindo a criação imediata da sombra inicial (EC-1).
3. **Incompatibilidade com Skills Legadas Renomeadas**:
   Ao renomear um `SKILL.md` legado do Claude Code, Cursor ou Codex para `SKILL.skill`, o arquivo não continha os campos obrigatórios `asl_version: "3.0"` nem a seção `interface:`. A desserialização estrita do `SkillManifest` retornava erro (`missing field asl_version`), inviabilizando a migração suave de arquivos legados (EC-5).

---

## 2. Proposta Detalhada da Decisão

Propomos uma solução arquitetural tripartite e universal para garantir 100% de compatibilidade em **qualquer sistema operacional** (macOS, Ubuntu, Debian, Arch Linux, Raspberry Pi/Alpine, Windows 10/11):

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                  ARQUITETURA ZERO-TOUCH MULTIPLATAFORMA (ASL)                │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   [ macOS ]                [ Linux (Debian/Arch/RPi) ]       [ Windows ]     │
│   FSEvents API             inotify / fanotify API            ReadDirChangesW │
│   launchd Agent            systemd --user / xdg-autostart    Task Scheduler  │
│        │                               │                            │        │
│        └───────────────────────┬───────┴────────────────────────────┘        │
│                                │ Evento de SO (Create / Modify / Rename)     │
│                                ▼                                             │
│                 ┌─────────────────────────────┐                              │
│                 │      ASL NATIVE DAEMON      │                              │
│                 │  (asl daemon / zero CPU)   │                              │
│                 └──────────────┬──────────────┘                              │
│                                │                                             │
│                                ▼                                             │
│                 ┌─────────────────────────────┐                              │
│                 │   TOLERANT PARSER ENGINE    │                              │
│                 │  - 0 Bytes Draft Ingestion  │                              │
│                 │  - Legacy Frontmatter Defaults                             │
│                 └──────────────┬──────────────┘                              │
│                                │                                             │
│                                ▼                                             │
│              Projeção Atômica Imediata: <nome>.md                            │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

### 2.1 Camada 1: Parser Tolerante a Greenfield e Migração Legada

O parser (`asl-parser` e `asl-spec`) é estendido para implementar a **Degradação Graciosa de Ingestão**:

1. **Tratamento de Arquivos Greenfield de 0 Bytes (Draft Inicial)**:
   - Se o arquivo `.skill` estiver vazio (`0 bytes`) ou contiver apenas espaços em branco, o parser não entra em pânico nem retorna erro.
   - Ele sintetiza um documento de rascunho canônico (*Greenfield Scaffold*) derivando o nome da skill a partir do próprio nome do arquivo:
     ```yaml
     asl_version: "3.0"
     name: "<file_stem>"
     description: "Draft skill under construction"
     interface:
       entrypoint: "run"
     ```
   - O motor de sombra projeta imediatamente o `<nome>.md` informando que a skill foi inicializada e aguarda preenchimento.

2. **Ingestão Tolerante de Skills Legadas (Zero-Touch Migration)**:
   - `asl_version`: Se ausente, assume automaticamente o valor padrão `"3.0"`.
   - `interface`: Se ausente, assume automaticamente o entrypoint canônico `"run"` com esquemas genéricos (`input_schema: {}`, `output_schema: {}`).
   - `name`: Se ausente no frontmatter, é inferido a partir do primeiro título `# Título` do Markdown ou do nome do arquivo.
   - `description`: Se ausente, é inferida a partir do primeiro parágrafo de texto após o título.

---

### 2.2 Camada 2: Motor Universal de Daemon em Background (`asl-daemon`)

O CLI `asl` ganha o módulo de daemon nativo de alta eficiência, com consumo desprezível (< 8 MB de RAM e 0% de CPU em repouso):

```rust
pub enum OsServiceKind {
    Launchd,        // macOS (~/Library/LaunchAgents)
    SystemdUser,    // Linux standard (Ubuntu, Debian, Arch, Fedora)
    GenericXdg,     // Linux minimal / Alpine / Raspberry Pi sem systemd
    WindowsTask,    // Windows 10/11 (schtasks / Startup Folder)
}
```

#### Ciclo de Vida do Daemon (`asl daemon`):
- `asl daemon start`: Inicia o processo observador desanexado em background.
- `asl daemon status`: Exibe PID, SO, consumo de memória, quantidade de `.skill` monitorados e integridade.
- `asl daemon stop`: Encerra o processo em background de forma segura.
- `asl daemon install`: Registra a inicialização automática no boot/login do usuário no sistema operacional hospedeiro.
- `asl daemon uninstall`: Desinstala e remove o serviço do sistema operacional.

---

### 2.3 Camada 3: Integração Específica por Sistema Operacional

#### A. macOS (Apple Silicon M1–M5 e Intel)
- **API de Kernel**: `FSEvents` via backend nativo do Rust (`notify-rs` com `kqueue`/`fsevents`).
- **Serviço de Inicialização**: `~/Library/LaunchAgents/org.asl-lang.daemon.plist`.
- **Comando de Ativação**: `launchctl load -w ~/Library/LaunchAgents/org.asl-lang.daemon.plist`.
- **Garantia**: Funciona tanto no terminal quanto em renomeações/criações feitas graficamente pelo Finder ou VS Code.

#### B. Linux (Ubuntu, Debian, Arch Linux, Fedora, Raspberry Pi OS)
- **API de Kernel**: `inotify` (Linux 2.6.13+) com buffer estendido e debounce de 50ms contra rajadas de eventos.
- **Serviço de Inicialização Primário**: `systemd --user` em `~/.config/systemd/user/asl-watcher.service`.
- **Comando de Ativação**: `systemctl --user enable --now asl-watcher.service`.
- **Fallback para Sistemas sem systemd (Alpine / Contêineres / Proxmox LXC)**:
  - Inicialização via script em `~/.config/autostart/asl-watcher.desktop` ou chamada em segundo plano via `nohup asl daemon start &`.

#### C. Windows (Windows 10, Windows 11, Windows Server)
- **API de Kernel**: `ReadDirectoryChangesW` com I/O Completion Ports (IOCP).
- **Serviço de Inicialização**: Tarefa Agendada no Logon do Usuário via `schtasks.exe`:
  ```cmd
  schtasks /Create /TN "ASLWatcher" /TR "asl daemon run" /SC ONLOGON /RL LIMITED /F
  ```
- **Fallback**: Atalho na pasta de Inicialização do Usuário (`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`).

#### D. Ambientes em Contêineres, Docker e WSL
- Suporte a diretórios montados via volume (onde notificações de kernel às vezes falham):
- O daemon detecta se o backend nativo não emite batimentos e ativa automaticamente um **Poll Híbrido Amigável à CPU** (intervalo de 1 segundo), garantindo paridade funcional universal.

---

### 2.4 Camada 4: Automação no Instalador (`install.sh`)

O instalador oficial `install.sh` passa a oferecer a **Ativação Automática do Daemon**:
Ao finalizar o download e instalação do binário `asl`, o script detecta o sistema operacional hospedeiro e executa `asl daemon install` silenciosamente. 
A partir do momento em que o instalador termina, qualquer `touch`, `mv` ou salvamento de arquivo `.skill` na máquina do usuário gera instantaneamente a projeção sombra `.md` sem nenhuma intervenção.

---

## 3. Alternativas Consideradas

- **Alternativa A: Exigir que o usuário rode sempre `asl watch` manualmente em um terminal**:
  - *Descartada*: Quebra diretamente a premissa de toque zero do ADR-0009. Se o usuário esquecer o terminal fechado, o `.md` não é atualizado, quebrando os agentes legados.
- **Alternativa B: Utilizar extensões de editor (ex: plugin VS Code)**:
  - *Descartada*: Falsa solução. Não funcionaria para scripts bash, CLI, agentes autônomos que operam fora do editor ou outros editores (Cursor, Zed, Neovim, Emacs).
- **Alternativa C: Daemon Nativo Multiplataforma Integrado ao SO com Parser Tolerante (Escolhida)**:
  - *Justificativa*: Resolve o problema na raiz do sistema operacional, funcionando de forma idêntica para humanos, IAs, editores gráficos e comandos de terminal em qualquer SO do mercado.

---

## 4. Consequências e Trade-offs

### Positivas
- **100% de Cumprimento da Especificação**: O comando `touch test.skill` ou a renomeação `mv old.md old.skill` gera o `.md` correspondente imediatamente em qualquer SO.
- **Zero Fricção para Desenvolvedores**: Não requer comandos adicionais nem dependências externas.
- **Consumo Mínimo de Recursos**: Menos de 8 MB de RAM e 0% de CPU em repouso.
- **Resiliência a Falhas**: Tolera arquivos de 0 bytes, drafts e formatos legados sem travar.

### Mitigações de Riscos
- **Consumo de Bateria / Disco**: O daemon utiliza listeners de eventos do kernel orientados a interrupção (não polling contínuo), acordando a CPU apenas quando um arquivo com extensão `.skill` ou `.md` é efetivamente tocado.
- **Limites de inotify no Linux**: O daemon monitora o diretório de desenvolvimento do usuário (ou o workspace atual), sem registrar watch recursivo em pastas desnecessárias (`/proc`, `/sys`, `node_modules`, `target`).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Preserva `.skill` como fonte canônica da verdade.
- [x] **Axioma 2 (Zero Dependências Externas)**: Implementado no binário `asl` em Rust puro, sem dependência de runtimes externos (Node, Python).
- [x] **Axioma 3 (OCAP Restrito)**: O daemon roda estritamente no espaço de usuário, sem privilégios de root/administrador.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Adaptadores de SO isolados em módulos claros dentro de `asl-cli`.
- [x] **Axioma 5 (Término Determinístico)**: Eventos tratados de forma assíncrona com limites estritos de tempo.
- [x] **Axioma 6 (Prefixo Estático)**: Projeções sombra mantêm prefixos estáticos canônicos.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Módulos divididos em micro-arquivos com menos de 450 linhas cada.
