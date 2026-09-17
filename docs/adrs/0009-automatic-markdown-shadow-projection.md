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
3. Se um usuário (humano ou agente de IA) simplesmente renomear um arquivo de `minha-skill.md` para `minha-skill.skill`, harnesses legados sem servidor MCP configurado perdem a capacidade imediata de localização direta por extensão.

O objetivo desta decisão é viabilizar a **Adoção de Toque Zero (*Zero-Touch Ingestion*)**: o desenvolvedor ou a IA apenas altera a extensão de `.md` para `.skill`, e todo o ecossistema existente continua funcionando sem qualquer configuração manual adicional.

---

## 2. Proposta Detalhada da Decisão

Implementar no ASL o motor de **Projeção Sombra Automática (*Shadow Projection Engine*)**:
Sempre que um arquivo `.skill` for criado, modificado ou detectado no sistema de arquivos, o ASL projeta e mantém automaticamente um arquivo `.md` correspondente no mesmo diretório como uma casca de compatibilidade transparente para os harnesses legados.

```
[Desenvolvedor ou IA salva: minha-skill.skill]
                       │
                       ▼
    ┌────────────────────────────────────────────────────────┐
    │              ASL SHADOW PROJECTION ENGINE              │
    │                                                        │
    │ 1. Lê minha-skill.skill e obtém digest SHA-256         │
    │ 2. minha-skill.md existe e possui o mesmo digest?      │
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

O arquivo `.md` gerado é estritamente derivado e possui a seguinte estrutura:

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

### 2.2 Salvaguardas de Engenharia Contra Falhas Comuns
Para evitar patologias clássicas de sistemas de sincronização de arquivos:

1. **Prevenção de Loops Infinitos de Eventos (*Event Storms*)**:
   - A escrita só é disparada se o digest SHA-256 do arquivo `.skill` diferir do carimbo presente na segunda linha do `.md`. Se os digests forem idênticos, a operação é um no-op ($O(1)$) sem qualquer escrita no disco.
2. **Escrita Atômica Transacional**:
   - O arquivo é escrito primeiro em `<nome>.md.tmp` e renomeado atomicamente via chamada de sistema `rename()`, eliminando arquivos corrompidos ou leituras de arquivos com 0 bytes.
3. **Paridade Absoluta Humano vs. IA**:
   - A detecção é baseada em chamadas de sistema do kernel (`FSEvents` / `inotify`) ou hooks de execução JIT do harness. Seja o comando `mv` disparado por um dedo humano no Finder/VS Code ou por uma chamada `Bash` do Claude Code, o comportamento é idêntico e determinístico.
4. **Desambiguação Cognitiva para o Agente**:
   - A tag `<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION -->` informa à IA que aquele `.md` não é um arquivo órfão nem indica falha na renomeação, mas uma projeção automática derivada de `minha-skill.skill`.

---

## 3. Alternativas Consideradas

- **Alternativa A: Forçar configuração manual de servidores MCP em cada projeto**:
  - *Descarte*: Complexidade inaceitável para o usuário final. Exige abrir arquivos de configuração internos de ferramentas (`~/.claude/mcp.json`, etc.), gerando alto atrito de adesão.
- **Alternativa B: Utilizar apenas Symlinks de Sistema Operacional (`ln -s`)**:
  - *Descarte*: Embora elegante no Unix, symlinks falham ou exigem privilégios de administrador no Windows padrão (`core.symlinks=false`), quebrando a promessa de universalidade multiplataforma do ASL.
- **Alternativa C: Projeção Sombra Automática de Arquivo Físico com Hash Guardrail (Escolhida)**:
  - *Justificativa*: Funciona universalmente em qualquer sistema operacional (Mac, Linux, Windows), não depende de drivers de kernel especiais, é rastreável pelo Git e fornece compatibilidade instantânea com 100% dos harnesses legados.

---

## 4. Consequências e Trade-offs

### Positivas
- **Adoção com Fricção Zero**: O desenvolvedor ou agente realiza uma única operação elementar: renomear `skill.md` para `skill.skill`.
- **Preservação de Descoberta em Harnesses Legados**: Scanners que buscam `*.md` continuam encontrando o arquivo imediatamente.
- **Transição Rápida para Execução Determinística**: O conteúdo do `.md` sombra instrui a IA a invocar `asl run`, colhendo instantaneamente a redução de 93% no consumo de tokens e a execução em microssegundos.
- **Fallback Cognitivo Íntegro**: Caso o binário `asl` não esteja disponível no ambiente do agente, o `.md` retém a totalidade do texto semântico, permitindo execução mental in-context.

### Negativas / Trade-offs
- O repositório passa a conter dois arquivos adjacentes (`.skill` e `.md`). O cabeçalho claro de auto-geração minimiza qualquer risco de edição manual acidental.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do `.skill`)**: O arquivo `.skill` permanece sendo a única e atômica fonte canônica da verdade. O `.md` é meramente um artefato derivado de projeção.
- [x] **Axioma 2 (Zero Dependências Externas)**: O mecanismo de projeção sombra é implementado em Rust puro dentro do binário `asl`.
- [x] **Axioma 3 (Object-Capability Estrito)**: A geração restringe-se estritamente ao diretório do arquivo `.skill`, sem escalada de diretórios.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Lógica de geração em `asl-parser` / `asl-cli`, sem acoplamento indevido.
- [x] **Axioma 5 (Término Determinístico)**: A geração do `.md` é uma função pura de tempo finito $O(N)$ sobre o tamanho do arquivo.
- [x] **Axioma 6 (Invariância de Prefixo Estático)**: O `.md` sombra gerado preserva os blocos de forma contígua e estática.
- [x] **Axioma 7 (Limite Cognitivo de < 450 linhas)**: As rotinas de geração são modulares e concisas.
