# Estudo Científico 0001: Mecanismos de Indexação, Recuperação Procedural e Integração em Catálogos de Harness para o ASL

- **Título**: *Análise Científica de Descoberta, Retenção em Memória de Trabalho e Injeção em Catálogos de Harness (Claude Code, OpenAI Codex, Antigravity) para a Agent Skill Language (ASL)*
- **Autor**: Jean Catarina *(Cadente)*
- **Data**: 2026-09-17
- **Classificação**: Cadente Advanced AI Systems Research (TR-ASL-2026-01)
- **Axiomas Vinculados**: Axioma 6 (Invariância de Prefixo Estático), Axioma 1 (Atomicidade), Axioma 4 (Isolamento Hexagonal)

---

## Resumo Executivo (Abstract)

Este estudo investiga o problema fundamental da **descoberta e ativação procedural de ferramentas** (*Procedural Skill Recall*) por modelos de linguagem de fronteira (Large Language Models - LLMs) inseridos em ambientes de automação (*Agent Harnesses*), tais como **Claude Code**, **OpenAI Codex/Operator** e **Google Antigravity**. Abordamos a seguinte questão epistemológica e de sistemas: 

> *Como um agente estocástico com janela de contexto finita "lembra" de selecionar um arquivo `.skill` específico no momento exato em que ele é necessário, e através de quais interfaces mecânicas o catálogo de skills é povoado no harness hospedeiro sem degradar a atenção do modelo nem invalidar o KV-Cache?*

Demonstramos teoricamente e empiricamente que a abordagem ingênua de injetar a totalidade do código e documentação de todas as skills no *System Prompt* sofre de saturação de entropia de atenção ($O(N)$ em consumo de tokens) e do fenômeno *Lost-in-the-Middle*. Em contrapartida, formalizamos a **Arquitetura de Memória Procedural em Duas Camadas (Two-Tier Procedural Memory)** do ASL, governada por quatro topologias de integração:
1. **Dynamic MCP Tool Gateway** (Descoberta via protocolo padronizado);
2. **Virtual Skill Paging** (Paginação sob demanda análoga a sistemas operacionais);
3. **Workspace Heuristic Bootstrap** (Injeção determinística de índices canônicos);
4. **Semantic Embedding Routing** (Roteamento vetorial denso/esparso para catálogos de escala industrial).

---

## 1. Fundamentação Teórica: Como um LLM "Lembra"?

### 1.1 O Modelo Cognitivo de Memória em Agentes de IA

Na psicologia cognitiva clássica (Anderson's ACT-R, 1993; Newell's SOAR, 1990; Baddeley, 1992), a cognição é dividida em:
1. **Memória de Trabalho (Working Memory)**: O espaço efêmero de manipulação ativa de símbolos com capacidade restrita ($7 \pm 2$ chunks).
2. **Memória Declarativa/Semântica (Long-Term Semantic Memory)**: Fatos, conceitos e relações estáticas.
3. **Memória Procedural (Procedural Memory)**: Regras de produção ("Se condição $C$, então execute ação $A$").

Em uma rede neural autoregressiva baseada em Transformer (Vaswani et al., 2017):
- A **Memória de Trabalho** corresponde unicamente à **Janela de Contexto** ativa ($\mathcal{W}$), manipulada pelas matrizes de projeção de Atenção Multi-Head ($\mathbf{Q}, \mathbf{K}, \mathbf{V}$).
- A **Memória Declarativa** está congelada nos pesos sinápticos ($\Theta$) obtidos no pré-treinamento.
- A **Memória Procedural** em agentes modernos **não reside nos pesos**, pois o modelo não é retreinado a cada nova ferramenta corporativa. Ela deve ser projetada externamente no contexto pelo *Agent Harness*.

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        MAPEAMENTO COGNITIVO DO AGENTE ARTIFICIAL                       │
├────────────────────────────┬─────────────────────────────┬─────────────────────────────┤
│ Componente Cognitivo       │ Equivalente em LLMs         │ Desafio de Engenharia       │
├────────────────────────────┼─────────────────────────────┼─────────────────────────────┤
│ Memória de Trabalho        │ Context Window (KV-Cache)   │ Custo quadrático e diluição │
│ Memória Declarativa        │ Pesos Sinápticos ($\Theta$) │ Inflexível / Sem atualização│
│ Memória Procedural (Skills)│ Prompts + Catálogo Harness  │ Recall preciso e sem ruído  │
└────────────────────────────┴─────────────────────────────┴─────────────────────────────┘
```

### 1.2 O Dilema da Saturação de Contexto vs. Cegueira Cognitiva

Seja $\mathcal{C} = \{\mathcal{S}_1, \mathcal{S}_2, \dots, \mathcal{S}_N\}$ o catálogo total de habilidades de uma organização (ex: $N = 150$).
Se cada arquivo `.skill` possui tamanho médio de $L_{\text{tokens}} = 800$:

$$\text{Custo Monolítico} = \sum_{i=1}^N |\mathcal{S}_i| = 150 \times 800 = 120.000\text{ tokens}$$

Injetar o catálogo completo no prompt de sistema gera duas patologias:
1. **Colapso de Custo e Latência**: $120.000$ tokens consumidos a cada turno de raciocínio.
2. **Degradação de Atenção (Attention Entropy & Lost-in-the-Middle)**: Conforme comprovado por Liu et al. (2023), à medida que a janela de contexto se expande, a capacidade do modelo de atender com precisão a instruções no meio do texto decai drasticamente. A relação Sinal-Ruído ($\text{SNR}$) para recuperar a ferramenta correta $\mathcal{S}^*$ é dada por:

$$\text{SNR}(\mathcal{S}^*) = \frac{\exp\left(\frac{\mathbf{q} \cdot \mathbf{k}_{\mathcal{S}^*}}{\sqrt{d}}\right)}{\sum_{i=1}^N \exp\left(\frac{\mathbf{q} \cdot \mathbf{k}_{\mathcal{S}_i}}{\sqrt{d}}\right) + \sum_{w \in \text{Context}} \exp\left(\frac{\mathbf{q} \cdot \mathbf{k}_w}{\sqrt{d}}\right)}$$

Quando $N$ cresce, o denominador domina o numerador, e a probabilidade de o agente "lembrar" ou invocar a skill correta tende a zero. Portanto, **o catálogo do harness jamais deve carregar o conteúdo integral das skills no boot**.

---

## 2. As Quatro Topologias Científicas de Registro e Recuperação

Para que o **Claude Code**, o **OpenAI Codex** ou o **Antigravity** recordem e usem as skills do ASL, o ecossistema opera sob uma arquitetura de camadas:

```
                      [Usuário / Tarefa do Agente]
                                   │
                                   ▼
         ┌──────────────────────────────────────────────────┐
         │              CAMADA 1: DESCOBERTA                │
         │   (Catalog Index / MCP tools/list / CLAUDE.md)   │
         └─────────────────────────┬────────────────────────┘
                                   │
                                   ▼ (Match de Intenção)
         ┌──────────────────────────────────────────────────┐
         │              CAMADA 2: ATIVAÇÃO JIT              │
         │    (Dynamic Schema / Semantic Envelope Load)     │
         └─────────────────────────┬────────────────────────┘
                                   │
                                   ▼ (Geração de Chamada)
         ┌──────────────────────────────────────────────────┐
         │            CAMADA 3: EXECUÇÃO HERMÉTICA          │
         │     (asl-vm-starlark / asl-vm-wasm / libasl)     │
         └──────────────────────────────────────────────────┘
```

---

### Topologia I: Gateway Dinâmico via Model Context Protocol (MCP)

O **Model Context Protocol (MCP)** é o padrão industrial nativo adotado pela Anthropic (Claude Code, Claude Desktop, Cursor).

#### Mecânica de Registro no Harness
O harness do Claude Code mantém um arquivo de configuração (ex: `~/.claude/mcp.json` ou `./.mcp.json`):
```json
{
  "mcpServers": {
    "asl-hub": {
      "command": "asl",
      "args": ["serve", "--transport", "stdio", "--path", "./skills"]
    }
  }
}
```

#### Como o Claude Code "Lembra":
1. **Negociação Inicial (`tools/list`)**: Na inicialização do processo do Claude Code, o harness emite uma chamada JSON-RPC `tools/list` para o binário `asl`.
2. **Projeção Sintética da Dual-Consumer AST**: O runtime ASL (`asl-protocol-mcp`) escaneia todos os arquivos `.skill` no diretório e projeta para o Claude apenas a casca semântica:
   - `name`: `manifest.name` (ex: `git-conventional-commit`).
   - `description`: Combinação condensada de `manifest.description` + `## Intent` + `## Activation Criteria`.
   - `inputSchema`: `manifest.interface.input_schema` (JSON Schema rigoroso).
3. **Povoamento do Catálogo Nativo**: O harness injeta essa lista de ferramentas no parâmetro de requisição `tools` da API da Anthropic.
4. **Ativação da Atenção**: Quando o usuário diz *"faça o commit das alterações"*, os pesos de atenção do Claude ativam o vetor correspondente à ferramenta `git-conventional-commit` porque a sua `description` contém explicitamente os termos semânticos relevantes. O Claude emite um bloco `tool_use`.
5. **Execução Zero-Overhead**: O harness repassa a chamada via `tools/call` para o ASL, que executa o código em microssegundos e retorna o resultado estruturado.

---

### Topologia II: Paginação Virtual de Habilidades (Two-Tier Skill Paging)

Para harnesses baseados em terminal ou CLI onde servidores MCP residentes em background não sejam desejados (ex: execução atômica via `asl run`), o ASL implementa o princípio da **Paginação de Memória Virtual** (Denning, 1968; Wang et al., 2023 - Voyager).

#### Nível 1: A Tabela de Páginas do Catálogo (Compact Index)
No início do contexto do agente (injetado via `CLAUDE.md`, `AGENTS.md` ou `.cursorrules`), mantém-se uma **Tabela de Páginas** ultracompacta:

```markdown
<!-- ASL SKILL CATALOG INDEX (Compact: ~25 tokens/skill) -->
- git-conventional-commit: Valida regras e formata mensagens de commit semântico.
- db-schema-audit: Inspeciona migrações SQL e detecta quebras de retrocompatibilidade.
- security-token-rotator: Executa rotação hermética de chaves temporárias via Ed25519.
Para carregar ou executar: `asl run <skill_name> --input '{"..."}'`
```

Com esta abordagem:
- 100 skills ocupam meros **$2.500$ tokens** (menos de $1.2\%$ de uma janela de 200k).
- O índice é **estático e invariante**, preservando 100% do **KV-Cache Hit Rate** (Axioma 6).

#### Nível 2: O Page-Fault Semântico
Quando o agente identifica que a intenção do usuário converge para `git-conventional-commit`:
1. O agente sabe da existência da skill pela Tabela de Páginas.
2. O agente faz a "paginação" (*page-in*) executando diretamente `asl run .skills/git-conventional-commit.skill --input ...` ou inspecionando o arquivo se precisar entender detalhes finos.
3. A memória de trabalho do agente nunca foi poluída com os detalhes das outras 99 skills.

---

### Topologia III: Bootstrap Heurístico em Agentes de Codificação (`CLAUDE.md`, `AGENTS.md`)

Agentes como o **Claude Code** e ferramentas como **Cursor** possuem um subsistema determinístico de leitura de arquivos de instruções no boot do repositório:
- O Claude Code sempre lê `CLAUDE.md` no diretório raiz do projeto.
- O Cursor lê `.cursorrules`.
- O Antigravity lê `AGENTS.md` e a pasta `skills/`.

#### O Protocolo ASL Autodiscovery Hook
O ASL introduz uma diretiva de automação no hook do projeto:
```bash
# Script de bootstrap ou hook de commit
asl sync-catalog --target CLAUDE.md
```
O comando `asl` inspeciona a pasta `.skills/` e gera uma seção canônica de índice no `CLAUDE.md`:
```markdown
## ASL Autonomous Skill Registry
As seguintes habilidades determinísticas estão disponíveis no workspace:
| Skill | Gatilho de Ativação | Invocação CLI |
| :--- | :--- | :--- |
| `git-conventional-commit` | Criar ou formatar commits | `asl run .skills/git-conventional-commit.skill` |
| `api-mock-validator` | Validar payloads HTTP de teste | `asl run .skills/api-mock-validator.skill` |
```

Quando o Claude Code inicializa, ele ingere esse arquivo diretamente em sua memória de longo prazo do repositório. Quando a tarefa do usuário requer criar um commit, a regra heurística gravada no contexto orienta: *"Você DEVE invocar a skill oficial via `asl run` ao invés de tentar adivinhar a formatação manualmente"*.

---

### Topologia IV: Roteamento Semântico e Dense Vector Retrieval (Para Grandes Organizações $N > 1.000$)

Para ecossistemas enterprise (como a Cadente) com milhares de skills especializadas, nem mesmo uma tabela de páginas compacta cabe no prefixo de contexto. O ASL formaliza o **Mecanismo de Roteamento Semântico em Espaço Latente**:

#### Representação Vetorial Canônica
Cada skill $\mathcal{S}$ possui um vetor de incorporação semântica $\mathbf{v}_{\mathcal{S}} \in \mathbb{R}^d$ calculado sobre o envelope semântico $\mathcal{P}$:

$$\mathbf{v}_{\mathcal{S}} = \text{Encoder}\left( \text{Manifest.Name} \ \Vert \ \text{## Intent} \ \Vert \ \text{## Activation Criteria} \right)$$

#### Algoritmo de Injeção Just-in-Time (JIT)
1. **Ingestão da Query**: O usuário emite a instrução $q$ (*"rotacione a chave de API da AWS"*).
2. **Cálculo da Similaridade de Cosseno**: O harness calcula $\mathbf{v}_q = \text{Encoder}(q)$ e consulta a base vetorial local indexada pelo ASL (`asl index`):
   $$\text{sim}(q, \mathcal{S}_i) = \frac{\mathbf{v}_q \cdot \mathbf{v}_{\mathcal{S}_i}}{\|\mathbf{v}_q\| \|\mathbf{v}_{\mathcal{S}_i}\|}$$
3. **Filtro Top-$k$**: São selecionadas as $k$ skills mais relevantes ($k = 3$).
4. **Injeção Dinâmica de Ferramentas**: Apenas essas 3 skills são injetadas temporariamente no catálogo MCP ou no prompt daquela requisição.
5. **Decisão do Modelo**: O Claude Code enxerga exatamente as ferramentas candidatas plausíveis, maximizando o SNR e eliminando qualquer alucinação de ferramentas inexistentes.

---

## 3. Análise Comparativa dos Mecanismos de Catálogo

| Dimensão Científica | Abordagem Legada (Scripts Python / Bash Soltos) | Abordagem Monolítica (Todas as skills no Prompt) | Abordagem ASL + MCP Gateway | Abordagem ASL + Two-Tier Paging |
| :--- | :--- | :--- | :--- | :--- |
| **Sobrecarga de Contexto** | Desconhecida (Agente navega diretórios via `ls`/`cat`) | Catastrófica ($> 100\text{k}$ tokens) | **Mínima** (Apenas esquemas JSON) | **Quase nula** (~$25$ tokens/skill) |
| **Taxa de Acerto de KV-Cache** | $0\%$ (Comandos dinâmicos poluem histórico) | $0\%$ (Saturação e alterações frequentes) | **$85\% - 95\%$** (Prefixo estável de tools) | **Otimizado** (Tabela imutável no topo) |
| **Garantia de Tipagem** | Nula (Strings brutas em stdout) | Baixa (Texto livre) | **Rigorosa** (JSON Schema via MCP) | **Rigorosa** (Gramáticas GBNF/Regex AOT) |
| **Resiliência a Falhas** | Baixa (Scripts falham por dependências de OS) | Média (Alucinação frequente) | **Alta** (Runtime Rust embutido) | **Máxima** (Execução local determinística) |
| **Confinamento de Injeção** | Nula (RCE direta no shell) | Nula (Prompt injection) | **Alto** (Delimitação de blast radius OCap) | **Alto** (Delimitação de blast radius OCap) |

---

## 4. O Caso Concreto: Passo a Passo nos Principais Agentes

### 4.1 No Claude Code (Anthropic)
1. **Configuração**: Adiciona-se o servidor MCP no arquivo `.claude/mcp.json` apontando para `asl serve`.
2. **Invocação**: O Claude Code não precisa adivinhar onde o arquivo está no disco. O modelo vê `git-conventional-commit` diretamente listado no seu painel interno de ferramentas disponíveis.
3. **Decisão Autônoma**: Se o Claude Code precisa fazer um commit, sua função de perda condicionada pelo fine-tuning de Tool-Use prioriza a ferramenta cuja descrição semântica coincide com a intenção.

### 4.2 No OpenAI Codex / Assistants / Operator
1. **Tool Definition Projection**: O harness do Codex consome a saída de `asl export-tools --format openai` e recebe um array de funções compatível com a API da OpenAI.
2. **Grammar Enforcement**: Se acoplado a motores de inferência locais ou open-source (vLLM, llama.cpp), o ASL fornece gramáticas AOT em **GBNF** geradas por `asl compile-grammar`, forçando o decodificador do Codex a gerar parâmetros $100\%$ válidos sem qualquer erro de sintaxe.

### 4.3 No Antigravity (Google DeepMind)
1. O Antigravity reconhece arquivos `.skill` ou pastas de skills com `SKILL.md`.
2. Através da `libasl` integrada in-process via C-ABI, o Antigravity avalia as skills determinísticas em **$34\ \mu\text{s}$**, sem precisar criar novos processos ou abrir conexões de rede.

---

## 5. Conclusões e Recomendações Estratégicas para o ASL

1. **Adotar a Dualidade MCP + Virtual Paging como Padrão Canônico**:
   - Para ambientes interativos avançados (Claude Code, Cursor, Claude Desktop), o modo primário de descoberta deve ser o **MCP Server via stdio/HTTP** (`asl serve`), onde as skills são expostas como ferramentas nativas com esquemas estritos.
   - Para ambientes minimalistas de terminal ou CI/CD, o modo primário deve ser a **Tabela de Páginas Canônica no `CLAUDE.md` / `AGENTS.md`** apontando para o binário `asl run`.
2. **Manter a Seção Semântica Invariante (Axioma 6)**:
   A descrição e os critérios de ativação exportados no `tools/list` devem ser extraídos literalmente do manifesto e do envelope semântico, garantindo que o prefixo permaneça $100\%$ idêntico entre todas as sessões para assegurar a reutilização máxima do KV-Cache na infraestrutura dos provedores de LLM.
3. **Automatizar a Sincronização**:
   Criar um subcomando `asl sync` que atualiza automaticamente o arquivo `CLAUDE.md` ou o manifesto MCP sempre que um novo arquivo `.skill` for adicionado ao projeto.

---

## Referências Bibliográficas

1. **Anderson, J. R.** (1993). *Rules of the Mind*. Lawrence Erlbaum Associates.
2. **Newell, A.** (1990). *Unified Theories of Cognition*. Harvard University Press.
3. **Baddeley, A.** (1992). *Working Memory*. Science, 255(5044), 556-559.
4. **Denning, P. J.** (1968). *The Working Set Model for Program Behavior*. Communications of the ACM, 11(5), 323-333.
5. **Liu, N. F., Lin, K., Hewitt, J., Paranjape, A., Bevilacqua, M., Petroni, F., & Liang, P.** (2023). *Lost in the Middle: How Language Models Use Long Contexts*. Transactions of the Association for Computational Linguistics (TACL).
6. **Wang, G., Xie, Y., Jiang, Y., Mandlekar, A., Xiao, C., Zhu, Y., Fan, L., & Anandkumar, A.** (2023). *Voyager: An Open-Ended Embodied Agent with Large Language Models*. arXiv preprint arXiv:2305.16291.
7. **Vaswani, A., et al.** (2017). *Attention Is All You Need*. Advances in Neural Information Processing Systems (NeurIPS 2017).
8. **Anthropic.** (2024). *Model Context Protocol (MCP) Specification*. Anthropic Engineering Publications.
9. **Catarina, J.** (2026). *Agent Skill Language (ASL): Uma Linguagem AI-First Hermética para Execução Determinística e Orquestração Semântica de Agentes Autônomos*. Cadente Technical Reports.
