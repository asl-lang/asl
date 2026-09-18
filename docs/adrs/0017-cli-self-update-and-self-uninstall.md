# ADR-0017: Ciclo de Vida de Auto-Atualização e Auto-Desinstalação na CLI (asl update & asl uninstall)

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-cli` (`lifecycle_cmds.rs`, `main.rs`), Gestão de Serviços de Sistema Operacional
- **Axiomas Relacionados**: Axioma 2 (Zero Dependências Externas), Axioma 3 (Segurança OCAP e Consentimento Explícito), Axioma 7 (Design Amigável e Limite Cognitivo)

---

## 1. Contexto e Declaração do Problema

O ecossistema do ASL evolui de maneira contínua, com atualizações frequentes de desempenho, suporte a novos modelos de linguagem, refinamentos de parser e melhorias na integração com ferramentas de IA (Claude Code, Cursor, Gemini). 

Entretanto, até este momento, manter o ASL atualizado ou removê-lo completamente da máquina exigia comandos manuais externos:
1. **Para atualizar**: O usuário precisava reexecutar o script `curl -fsSL ... | bash` ou compilar novamente via `cargo install --path ...`.
2. **Para desinstalar**: O usuário precisava lembrar de parar o daemon (`asl daemon stop`), desinstalar o serviço (`asl daemon uninstall`), localizar o caminho do binário (`which asl`) e removê-lo manualmente (`rm -f $(which asl)`), além de limpar diretórios em `~/.asl`.

Essa ausência de comandos de ciclo de vida nativos prejudica a experiência do desenvolvedor e contraria as melhores práticas de ferramentas modernas de linha de comando (como `rustup`, `brew` e `gh`).

---

## 2. Proposta Detalhada da Decisão

Decidimos introduzir dois comandos oficiais de ciclo de vida diretamente na CLI do ASL: `asl update` e `asl uninstall`.

### 2.1. Comando de Auto-Atualização: `asl update`

Permite atualizar o binário do ASL para a versão mais recente publicada nos canais oficiais do GitHub Releases:
- **Detecção de Versão sem Rate-Limit**: Utiliza inspeção de redirecionamento HTTP da URL canônica `https://github.com/asl-lang/asl/releases/latest`, evitando o esgotamento de cotas de API não autenticadas do GitHub.
- **Detecção Automática de Plataforma**: Mapeia o binário em execução para a tríade de destino correspondente (`aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`).
- **Substituição Atômica in-place**:
  - Em sistemas POSIX (macOS e Linux): Escreve o novo binário em um arquivo temporário adjacente, define permissões `0o755` e realiza substituição atômica via `fs::rename`.
  - No Windows: Renomeia o binário atual para extensão temporária e posiciona o novo executável.
- **Preservação de Estado do Daemon**: Se o daemon de segundo plano estiver ativo no momento da atualização, o comando reinicia o serviço para carregar a versão recém-instalada.
- **Flags Suportadas**:
  - `--check`: Apenas verifica se há atualização disponível sem baixar.
  - `--force`: Força a reinstalação mesmo se a versão instalada for idêntica.

### 2.2. Comando de Auto-Desinstalação: `asl uninstall`

Permite remover completamente o ASL da máquina hospedeira de forma limpa, segura e transparente:
- **Confirmação Interativa por Padrão**:
  ```text
  ⚠️  This will completely uninstall ASL from your system:
     - Stop and deregister background services (launchd/systemd/tasks)
     - Delete the binary at /path/to/asl

  Are you sure you want to proceed? [y/N]: 
  ```
- **Deregistro de Serviços de Sistema**: Interrompe e remove agentes `launchd` (macOS), serviços `systemd --user` (Linux) e tarefas agendadas `schtasks` (Windows).
- **Remoção do Executável**:
  - Em POSIX: Deleta o próprio arquivo executável em execução (`fs::remove_file(&current_exe)`).
  - No Windows: Despacha processo assíncrono temporário para desvincular o arquivo após término do processo.
- **Flags Suportadas**:
  - `-y`, `--yes`: Pula a confirmação interativa para scripts e automações.
  - `--purge`: Remove também o diretório de dados em repouso `~/.asl` (logs, PIDs e configurações locais).

---

## 3. Alternativas Consideradas

- **Alternativa A: Delegar atualização exclusivamente a gerenciadores externos (Homebrew, Cargo, Apt)**:
  - *Descartada*: Gera dependência de aprovação externa de pacotes, lentidão para correções de segurança imediatas e não atende quem instalou via script direto `install.sh`.
- **Alternativa B: Deixar a desinstalação manual via instruções no README**:
  - *Descartada*: Gera resíduos no sistema operacional (como LaunchAgents órfãos tentando reiniciar um binário apagado).
- **Alternativa C: Módulo Nativo `lifecycle_cmds` com Isolamento Hexagonal (Escolhida)**:
  - *Justificativa*: Oferece controle total, simplicidade com 1 comando, zero atrito para o usuário e garantia de desinstalação limpa e atômica.

---

## 4. Consequências e Benefícios

### Positivas
- **Manutenibilidade Instantânea**: Atualização para novas versões com um simples `asl update`.
- **Transparência e Respeito ao Usuário**: O desenvolvedor pode remover 100% dos rastros do ASL a qualquer momento com `asl uninstall`.
- **Segurança de Execução**: Previne inconsistências entre o daemon de background e a CLI.
- **Conformidade de Linhas**: A lógica foi isolada em `lifecycle_cmds.rs`, mantendo todos os arquivos abaixo do limite de 450 linhas.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Não altera o formato das skills.
- [x] **Axioma 2 (Zero Dependências Externas)**: Utiliza ferramentas padrão do sistema (`curl`, `tar`) sem exigir dependências extras.
- [x] **Axioma 3 (OCAP Restrito e Consentimento)**: Exige confirmação explícita antes de desinstalar.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Comandos de ciclo de vida isolados no adaptador de CLI.
- [x] **Axioma 5 (Término Determinístico)**: Timeouts e saídas limpas em caso de falha de rede.
- [x] **Axioma 6 (Prefixo Estático)**: Não aplicável a comandos de ciclo de vida.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Novo arquivo `lifecycle_cmds.rs` implementado com ~200 linhas (< 450).
