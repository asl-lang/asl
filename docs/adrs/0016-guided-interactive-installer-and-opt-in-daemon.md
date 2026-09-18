# ADR-0016: Instalação Interativa Guiada e Ativação Consentida do Daemon de Background

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `install.sh`, `asl-cli` (`daemon_cmds`, `main.rs`), Documentação e Onboarding
- **Axiomas Relacionados**: Axioma 2 (Zero Dependências Externas), Axioma 3 (Segurança OCAP e Consentimento Explícito), Axioma 7 (Design Amigável e Limite Cognitivo)

---

## 1. Contexto e Declaração do Problema

O **ADR-0015** introduziu o daemon nativo multiplataforma (`asl daemon install`) integrado ao gerenciador de inicialização do sistema operacional (`launchd` no macOS, `systemd` no Linux, `schtasks` no Windows). No instalador `install.sh`, esse daemon era instalado e iniciado de forma incondicional.

Entretanto, para uma parcela significativa de engenheiros de software e administradores de sistemas, ter processos ou serviços registrados para inicialização automática em segundo plano sem aviso prévio ou consentimento explícito pode parecer **intrusivo**, opaco ou contrário aos princípios da filosofia Unix de transparência e controle total da máquina pelo usuário.

Além disso, em ambientes conteinerizados, servidores de CI/CD (GitHub Actions, GitLab CI) ou pipelines automatizados, a tentativa de registrar agentes `launchd` ou `systemd --user` de forma silenciosa e obrigatória pode causar falhas espúrias ou poluição desnecessária do ambiente.

---

## 2. Proposta Detalhada da Decisão

Decidimos transformar o processo de instalação e configuração do ASL em uma **Instalação Guiada e Transparente (*Guided Interactive Setup*)**, onde o desenvolvedor é apresentado a uma explicação concisa e pode escolher livremente se deseja ativar a projeção automática em segundo plano, com a opção recomendada destacada.

### 2.1. Fluxo Interativo no `install.sh`

Durante a execução de `install.sh`, após a instalação ou compilação do binário `asl`:

1. **Detecção de Terminal Interativo e `/dev/tty`**:
   Mesmo quando o usuário executa o comando via pipe curl (`curl -fsSL https://... | bash`), o instalador tenta ler do `/dev/tty` caso o `stdin` padrão esteja conectado ao pipe de download.
2. **Apresentação Guiada com Opção Recomendada**:
   ```text
   ========================================================
   💡 Guided Configuration: Automatic Zero-Touch Sync
   ========================================================
   ASL includes a lightweight background watcher that automatically
   projects and updates .md files whenever a .skill is created or edited
   in Claude Code, Cursor, Gemini, or your dev workspaces.

   Enable automatic background sync? (Recommended) [Y/n]: 
   ```
3. **Comportamento da Escolha**:
   - **Sim / Enter (`Y`, `y`, `Enter`)**: Registra e inicia o serviço daemon nativo via `asl daemon install`. Exibe mensagem de confirmação e comandos de controle (`asl daemon status`, `asl daemon stop`, `asl daemon uninstall`).
   - **Não (`n`, `N`)**: Não instala nem inicia nenhum serviço em segundo plano. Orienta o usuário sobre o modo sob demanda (`asl sync <caminho>` e `asl watch <caminho>`) e informa que o daemon pode ser ativado futuramente quando desejar com `asl daemon install`.

### 2.2. Flags de Automação para CI e Scripts Headless

Para garantir a compatibilidade com automações, scripts headless e pipelines:
- Flags CLI em `install.sh`:
  - `--enable-daemon` ou `-y` ou `--yes`: Ativa o daemon sem perguntar.
  - `--no-daemon` ou `-n` ou `--no`: Pula a instalação do daemon sem perguntar.
- Variáveis de ambiente:
  - `ASL_ENABLE_DAEMON=1` ou `true`: Força ativação.
  - `ASL_ENABLE_DAEMON=0` ou `false`: Força desativação.
- **Fallback Headless**: Se nenhum terminal interativo (`/dev/tty`) estiver disponível e nenhuma variável for fornecida (ex.: container Docker ou runner CI), o instalador opera em modo não-intrusivo (não instala o daemon) e emite uma nota informativa, nunca travando o script.

### 2.3. Comando Nativo na CLI: `asl setup`

Adiciona o comando de alto nível `asl setup` na CLI oficial, permitindo que qualquer desenvolvedor execute novamente o assistente guiado a qualquer momento:
```bash
asl setup
```

---

## 3. Alternativas Consideradas

- **Alternativa A: Manter ativação obrigatória e silenciosa no instalador**:
  - *Descartada*: Gera desconfiança para desenvolvedores preocupados com privacidade, consumo e integridade de seus processos de inicialização.
- **Alternativa B: Nunca instalar o daemon automaticamente, exigindo comando manual**:
  - *Descartada*: Aumenta o atrito e quebra a expectativa de iniciantes que desejam que a renomeação automática funcione imediatamente após a instalação.
- **Alternativa C: Instalação Guiada com Recomendação Destacada [Y/n] e Fallback Headless (Escolhida)**:
  - *Justificativa*: Equilibra perfeitamente a melhor experiência out-of-the-box (basta teclar Enter) com respeito absoluto à soberania do desenvolvedor sobre seu ambiente operacional.

---

## 4. Consequências e Benefícios

### Positivas
- **Confiança e Transparência**: O usuário mantém controle total sobre o que roda em sua máquina.
- **Onboarding Educativo**: O usuário aprende o que o daemon faz e que pode utilizar `asl sync` caso prefira fluxos manuais.
- **Amigável a CI/CD**: Automações não falham em ambientes headless.
- **Flexibilidade**: A qualquer momento o usuário pode alterar sua preferência com `asl setup`, `asl daemon install` ou `asl daemon stop`.

### Mitigações
- Para pipelines que desejam o daemon ativo sem interação humana, a flag `--enable-daemon` ou `ASL_ENABLE_DAEMON=1` permite automação total sem interrupções.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Não altera o formato do `.skill`.
- [x] **Axioma 2 (Zero Dependências Externas)**: O instalador e o comando `asl setup` não necessitam de bibliotecas externas.
- [x] **Axioma 3 (OCAP Restrito e Consentimento)**: Serviços de background operam com consentimento do usuário.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Modificações mantidas isoladas em `daemon_cmds.rs` na CLI.
- [x] **Axioma 5 (Término Determinístico)**: Prompt lê uma linha ou avança em caso de EOF/timeout.
- [x] **Axioma 6 (Prefixo Estático)**: Inalterado.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Todos os arquivos `.rs` modificados permanecem estritamente abaixo de 450 linhas.
