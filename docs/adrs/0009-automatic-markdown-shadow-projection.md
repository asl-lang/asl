# ADR-0009: Projeção Sombra Automática de Markdown (Shadow Projection) para Adoção de Toque Zero

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Jean Catarina, Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-parser`, `asl-cli`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 6 (Invariância de Prefixo Estático)

---

## 1. Contexto e Problema

A transição de desenvolvedores e organizações do ecossistema legado de agentes (`SKILL.md` + scripts em Python/Bash) para o padrão canônico **Agent Skill Language (ASL 3.0)** enfrenta a barreira do atrito de configuração (*Configuration Friction*):
1. Ferramentas e harnesses consolidados no mercado (Claude Code, Cursor, Claude Desktop, Antigravity) possuem rotinas pré-programadas de busca que filtram estritamente por arquivos `.md` (ex.: `glob("**/*.md")` ou `**/SKILL.md`).
2. Exigir que o desenvolvedor configure servidores MCP manuais, edite arquivos JSON (`.claude/mcp.json`, `claude_desktop_config.json`) ou altere arquivos de governança para cada skill inviabiliza a adoção orgânica e de baixo atrito.
3. Se um usuário ou agente de IA cria um arquivo `.skill` do zero, ou renomeia um `.md` para `.skill`, harnesses legados sem servidor MCP perdem a capacidade de localização direta por extensão.

O objetivo desta decisão é viabilizar a **Adoção de Toque Zero (*Zero-Touch Ingestion*)**: tanto na **migração de skills legadas** quanto na **criação de novas skills do zero**, o desenvolvedor ou a IA manipula unicamente arquivos `.skill`, e todo o ecossistema existente continua funcionando sem qualquer configuração manual adicional.

---

## 2. Proposta Detalhada da Decisão

Implementar no ASL o motor de **Projeção Sombra Automática (*Shadow Projection Engine*)**:
Sempre que um arquivo `.skill` for criado (com ou sem `.md` prévio), modificado ou detectado no sistema de arquivos, o ASL projeta e mantém automaticamente um arquivo `.md` correspondente no mesmo diretório como uma casca de compatibilidade transparente para os harnesses legados.

```
[Criação do Zero OU Renomeação: minha-skill.skill]
                        │
                        ▼
    ┌────────────────────────────────────────────────────────┐
    │              ASL SHADOW PROJECTION ENGINE              │
    │                                                        │
    │ 1. Lê minha-skill.skill e obtém digest SHA-256         │
    │ 2. Avalia Matriz de Edge Cases (Colisão, Delete, Hash) │
    │ 3. minha-skill.md existe e possui o mesmo digest?      │
    │    ├── SIM ──► No-op imediato (0 I/O, evita loops)     │
    │    └── NÃO ──► Gera minha-skill.md.tmp e renameat()    │
    └──────────────────────────┬─────────────────────────────┘
                               │
                               ▼
        minha-skill.md (Projeção Sombra Gerada em < 1 ms)
                               │
            ┌──────────────────┴──────────────────┐
            ▼                                     ▼
[Harnesses Legados (Cursor, Claude)]     [Repositório Git & Devs]
Localizam o .md via glob e aprendem      Visibilidade no GitHub Web;
a invocar `asl run minha-skill.skill`    Zero atrito de transição.
```

### 2.1 Formato Canônico do Arquivo Sombra Gerado

```markdown
<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->
<!-- CANONICAL SOURCE: ./minha-skill.skill | DIGEST: asl:sha256:... -->
---
asl_version: "3.0"
name: "minha-skill"
description: "Descrição da skill extraída do manifesto original"
asl_canonical_source: "./minha-skill.skill"
asl_digest: "asl:sha256:19f41c0e7ca32215..."
---
# minha-skill

> ⚡ **Esta habilidade é governada e executada pelo runtime hermético ASL 3.0.**
> Arquivo canônico: [`minha-skill.skill`](./minha-skill.skill)

### Diretiva para Agentes de IA (Claude Code, Cursor, Codex):
Para executar esta skill de forma determinística, segura e com 93% de economia de tokens:
```bash
asl run ./minha-skill.skill --input '<json_de_entrada>'
```

---

## Instruções Semânticas Oficiais
(Espelho exato da seção semântica do arquivo .skill para fallback cognitivo in-context)
```

### 2.2 Salvaguardas de Concorrência e Sistema Operacional
1. **Prevenção de Loops Infinitos de Eventos (*Event Storms*)**:
   - A escrita só é disparada se o digest SHA-256 do arquivo `.skill` diferir do carimbo presente na segunda linha do `.md`. Se os digests forem idênticos, a operação é um no-op ($O(1)$) sem qualquer escrita no disco.
2. **Escrita Atômica Transacional**:
   - O arquivo é escrito primeiro em `<nome>.md.tmp` e renomeado atomicamente via chamada de sistema `rename()`, eliminando arquivos corrompidos ou leituras de arquivos com 0 bytes.
3. **Paridade Absoluta Humano vs. IA**:
   - A detecção é baseada em chamadas de sistema do kernel (`FSEvents` / `inotify`) ou hooks de execução JIT do harness. Seja o comando `touch`/`mv` disparado por um dedo humano no Finder/VS Code ou por uma chamada `Bash` do Claude Code, o comportamento é idêntico e determinístico.
4. **Desambiguação Cognitiva para o Agente**:
   - A tag `<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION -->` informa à IA que aquele `.md` não é um arquivo órfão nem indica falha na renomeação, mas uma projeção automática derivada de `minha-skill.skill`.

---

### 2.3 Matriz Exaustiva de Edge Cases e Comportamentos Formais

Abaixo, formaliza-se a especificação de tratamento para todos os cenários de borda possíveis:

| ID | Cenário de Borda (Edge Case) | Comportamento Formal do ASL | Garantia Arquitetural |
| :--- | :--- | :--- | :--- |
| **EC-1** | **Criação Direta do Zero (Greenfield)**<br>Usuário ou IA cria `nova.skill` sem nunca ter existido nenhum `.md` antes. | O engine detecta a criação do `.skill` e gera imediatamente o arquivo `nova.md` pela primeira vez no mesmo diretório. | Suporte nativo a novos projetos sem legado prévio. |
| **EC-2** | **Exclusão do `.skill` (Deletion Lifecycle)**<br>Usuário ou IA deleta `deploy.skill` (`rm deploy.skill`). | O engine detecta a deleção da fonte canônica e **remove automaticamente o arquivo sombra `deploy.md` associado**. | Impede arquivos órfãos com links quebrados no repositório. |
| **EC-3** | **Edição Manual Acidental no `.md` Sombra**<br>Desenvolvedor edita `deploy.md` em vez de `deploy.skill`. | Na próxima checagem/execução, o digest do `.md` diverge do `.skill`. O ASL **sobrescreve o `.md` restaurando a projeção canônica do `.skill`** e emite advertência. | A fonte da verdade permanece estritamente atômica em `.skill` (Axioma 1). |
| **EC-4** | **Conflito de Nome Pré-existente**<br>Já existia um `deploy.md` legítimo (manual/documentação) antes do `deploy.skill` ser criado. | O ASL inspeciona o cabeçalho do `deploy.md`. Se **NÃO** contiver a marca `ASL AUTO-GENERATED`, o ASL **não sobrescreve**, cria `deploy.asl.md` e emite alerta. | Proteção total contra perda acidental de dados pré-existentes. |
| **EC-5** | **Renomeação de `.skill` (`mv a.skill b.skill`)**<br>Arquivo canônico muda de nome. | O engine remove `a.md` e gera `b.md` atomicamente em $< 1\text{ ms}$. | Consistência relacional $1:1$ garantida. |
| **EC-6** | **Layouts de Pasta (`skills/foo/SKILL.md`)**<br>Projetos que usam pastas isoladas por skill. | Se o arquivo for `skills/foo/SKILL.skill`, o engine projeta `skills/foo/SKILL.md`. Se for `skills/foo.skill`, projeta `skills/foo.md`. | Compatibilidade universal com layouts planos ou aninhados. |
| **EC-7** | **Git Branch Switching / Checkout em Lote**<br>Troca de branch altera dezenas de skills simultaneamente. | O engine aplica **Debounce em Lote (Batch Aggregation)** com janela de 50ms, recalculando os digests e gerando apenas as sombras divergentes. | Imunidade a tempestades de eventos de I/O (*Event Storms*). |
| **EC-8** | **Ambientes Headless e CI/CD (Sem Daemon)**<br>Esteiras GitHub Actions ou containers sem serviço de background. | Os arquivos `.md` sombra são versionados no Git junto com os `.skill`. O comando `asl check` no CI valida que 100% dos `.md` estão em conformidade canônica. | Zero dependência de serviços em segundo plano no CI/CD. |
| **EC-9** | **Filesystem Somente-Leitura (Read-Only)**<br>Execução em container Docker com flag `--read-only`. | Se o `.md` já existe, lê normalmente. Se tentar gerar e receber `EROFS`, não entra em pânico: registra log de aviso e segue execução in-memory. | Resiliência operacional sem crashes (Axioma 5). |
| **EC-10**| **Skills Puras de Prompt (Sem Bloco `asl`)**<br>Skill que contém apenas instruções em linguagem natural. | O `.md` sombra é gerado com as instruções semânticas completas e diretiva informando que a execução pode ocorrer in-context. | Zero fricção para prompts puramente cognitivos. |
| **EC-11**| **Arquivos Ocultos e Swaps de Editores**<br>Vim/VS Code geram `.deploy.skill.swp` ou `deploy.skill~`. | O engine ignora terminantemente qualquer arquivo que inicie com `.` ou termine em `~`, `.tmp`, `.swp` ou `.bak`. | Prevenção contra poluição e falsos disparos. |

---

## 3. Alternativas Consideradas

- **Alternativa A: Forçar configuração manual de servidores MCP em cada projeto**:
  - *Descarte*: Complexidade inaceitável para o usuário final. Exige abrir arquivos de configuração internos de ferramentas (`~/.claude/mcp.json`, etc.), gerando alto atrito de adesão.
- **Alternativa B: Utilizar apenas Symlinks de Sistema Operacional (`ln -s`)**:
  - *Descarte*: Embora elegante no Unix, symlinks falham ou exigem privilégios de administrador no Windows padrão (`core.symlinks=false`), quebrando a promessa de universalidade multiplataforma do ASL.
- **Alternativa C: Projeção Sombra Automática de Arquivo Físico com Matriz de Edge Cases (Escolhida)**:
  - *Justificativa*: Funciona universalmente em qualquer sistema operacional (Mac, Linux, Windows), suporta criação do zero e migração, não depende de drivers de kernel especiais, é rastreável pelo Git e fornece compatibilidade instantânea com 100% dos harnesses legados.

---

## 4. Consequências e Trade-offs

### Positivas
- **Adoção com Fricção Zero**: Funciona tanto para quem cria `.skill` do zero quanto para quem migra de `.md`.
- **Limpeza Automática do Repositório**: A exclusão de um `.skill` limpa automaticamente o `.md` órfão correspondente (EC-2).
- **Preservação de Descoberta em Harnesses Legados**: Scanners que buscam `*.md` continuam encontrando o arquivo imediatamente.
- **Transição Rápida para Execução Determinística**: O conteúdo do `.md` sombra instrui a IA a invocar `asl run`, colhendo instantaneamente a redução de 93% no consumo de tokens e a execução em microssegundos.
- **Fallback Cognitivo Íntegro**: Caso o binário `asl` não esteja disponível no ambiente do agente, o `.md` retém a totalidade do texto semântico, permitindo execução mental in-context.

### Negativas / Trade-offs
- O repositório passa a conter dois arquivos adjacentes (`.skill` e `.md`). O cabeçalho claro de auto-geração e o mecanismo anti-drift (EC-3) eliminam o risco de inconsistência.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do `.skill`)**: O arquivo `.skill` permanece sendo a única e atômica fonte canônica da verdade. O `.md` é meramente um artefato derivado de projeção.
- [x] **Axioma 2 (Zero Dependências Externas)**: O mecanismo de projeção sombra é implementado em Rust puro dentro do binário `asl`.
- [x] **Axioma 3 (Object-Capability Estrito)**: A geração restringe-se estritamente ao diretório do arquivo `.skill`, sem escalada de diretórios.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Lógica de geração em `asl-parser` / `asl-cli`, sem acoplamento indevido.
- [x] **Axioma 5 (Término Determinístico)**: A geração do `.md` é uma função pura de tempo finito $O(N)$ sobre o tamanho do arquivo.
- [x] **Axioma 6 (Invariância de Prefixo Estático)**: O `.md` sombra gerado preserva os blocos de forma contígua e estática.
- [x] **Axioma 7 (Limite Cognitivo de < 450 linhas)**: As rotinas de geração são modulares e concisas.
