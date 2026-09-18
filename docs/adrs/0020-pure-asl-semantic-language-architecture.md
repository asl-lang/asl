# ADR-0020: Unificação da Linguagem Semântica ASL (Um Único ASL Otimizado para IA e Humanos)

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-parser`, `asl-spec`, `asl-cli`, `docs/`, `website/`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 5 (Término com Fuel Metering), Axioma 6 (Invariância de KV-Cache), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Declaração do Problema

Não deve existir uma divisão artificial entre "asl:rules" e "asl:deterministic". **Só existe um único ASL.**

O ASL (Agent Skill Language) é uma maneira mais semântica, concisa e de alto nível de escrever Starlark, com foco conjunto em **Inteligência Artificial e seres humanos**, garantindo **máxima economia de tokens sem jamais perder a clareza**.

### A Filosofia Central: Programar Igual Escrevendo um Markdown
O coração do ASL é simples: **programar deve ser exatamente igual a escrever um arquivo Markdown (`.md`)**.
- Em vez de cerimônias técnicas, configurações complexas de build e separação entre documentação e código executável, o próprio documento Markdown **é** o programa.
- O desenvolvedor ou agente de IA escreve o cabeçalho, instruções em prosa clara para a IA, regras e lógica em blocos concisos ````asl````.
- O resultado é executável instantaneamente (`./skill.skill`), publicável como servidor MCP (`asl serve`) e legível nativamente por qualquer modelo de linguagem com 100% de aproveitamento de KV-Cache.
- Por debaixo dos panos, o motor do ASL converte esse Markdown vivo em Starlark hermético e determinístico em memória RAM.

Além disso, desenvolvedores que vinham de ambientes shell (Bash) ou automações em Python precisavam de um mapeamento claro demonstrando como o ASL atende a qualquer cenário de automação de forma mais segura, determinística e com muito menos tokens.

---

## 2. Proposta Detalhada da Decisão

### 2.1. Tag Universal Única: Exclusivamente ```asl
A partir do ASL 3.0, **existe apenas uma única tag de código executável em todo o ecossistema ASL: ```asl**.

- **Regra do Parser (`asl-parser`)**:
  - Apenas blocos com a tag exata ````asl```` são processados pelo motor de execução.
  - Qualquer outra tag (`asl:deterministic`, `asl:rules`, `starlark`, `python-deterministic`, `python`, `bash`, `sh`, `rules`) é sumariamente ignorada pelo motor hermético e tratada como bloco descritivo/documental comum em Markdown.
  - Zero tolerância para retrocompatibilidade com tags descontinuadas.

### 2.2. Unificação Sintática: Pattern Matching e Funções no Mesmo ASL
Não há dois mundos ou duas linguagens: no ASL, pattern matching declarativo (`match / when / guard`) e funções imperativas (`def run(ctx, input):`) são recursos integrados da mesma linguagem, ambos delimitados unicamente pela tag ````asl````.
- **Foco em Economia de Tokens e Densidade Semântica**: Cláusulas `match / when` economizam até 70% de tokens em relação a dezenas de `if/else` procedurais.
- **Transparência para o Compilador**: O parser detecta automaticamente a presença de regras declarativas ou funções canônicas, compilando ambas para Starlark estrito hermético em memória.
- **Zero Atrito**: O autor nunca precisa se perguntar "qual tag usar". É sempre e apenas ````asl````.

### 2.3. Funcionamento Sob o Capô: Transpilação em Memória para Starlark Hermético
Embora desenvolvedores e agentes de IA escrevam **exclusivamente em ASL** (utilizando a tag universal ````asl````), **por debaixo dos panos a linguagem ASL transpila em memória (AOT) para Starlark hermético**.
- O autor escreve apenas código ASL (`match / when / guard` ou `def run(ctx, input)`).
- O compilador do ASL converte essas construções em Starlark estrito em memória RAM (sem gerar arquivos lixo no disco).
- O runtime executa esse Starlark dentro de uma sandbox hermética com medição de combustível (*fuel metering*) e isolamento de capacidades (*OCap*), garantindo término determinístico e segurança contra loops infinitos.
- Através de `asl expand <skill>`, desenvolvedores e agentes podem auditar o código Starlark hermético resultante compilado em memória.

### 2.4. Eliminação de Tags Externas e Nomenclatura Coesa
- A mensagem de erro em `asl-spec` é atualizada para: `Missing ASL code block (```asl)`.
- O ecossistema deixa explícito que ASL é a linguagem de autoria oficial de skills, enquanto Starlark atua como a camada de máquina/execução intermediária em memória sob o capô.

### 2.5. A Rosetta Stone do ASL (Guia Universal de Migração)
Para demonstrar que qualquer operação comum em Bash, Python ou Starlark pode ser realizada em ASL de maneira mais limpa, auditável e segura, a CLI (`asl docs migration` / `asl docs rosetta` / `asl docs bash` / `asl docs python`) e o website disponibilizam a tabela canônica:

#### Equivalência Bash -> ASL
| Bash | ASL Canônico (` ```asl `) | Vantagem ASL |
|:---|:---|:---|
| `cat file.txt` | `content = ctx.fs.read("file.txt")` | Sandbox OCap hermética sem escape |
| `echo "$data" > file.txt` | `ctx.fs.write("file.txt", data)` | Sem risco de injeção de comandos |
| `curl -X POST -d "$body" $URL` | `resp = ctx.http.post(url, json=body)` | Tipado e medição por combustível |
| `if [ "$cmd" = "deploy" ]` | `match input.cmd:\n  when "deploy":` | Pattern matching sem bugs de quoting |
| `echo "$str" \| grep "needle"` | `"needle" in text` ou regras `regex()` | Avaliação nativa em RAM sem subprocessos |
| `cmd1 \| cmd2` | `d1 = step1(input)\nd2 = step2(d1)` | Dados fluem como estruturas JSON tipadas |

#### Equivalência Python -> ASL
| Python | ASL Canônico (` ```asl `) | Vantagem ASL |
|:---|:---|:---|
| `import os / sys / subprocess` | *Proibido*: usar `ctx.fs`, `ctx.env` | Sem brechas de Remote Code Execution |
| `import json; json.loads(s)` | `data = json.decode(s)` | Built-in determinístico em tempo linear |
| `import hashlib; sha256()` | `h = ctx.crypto.sha256(data)` | Hashing nativo ultra-rápido em Rust |
| `import re; re.match(pat, s)` | `match text:\n  when regex(pat):` | Compilação AOT segura contra ReDoS |
| `dict.get("key", default)` | `input.get("key", default)` | Sintaxe idêntica, sem efeitos colaterais |
| `[x*2 for x in items if x > 0]` | `[x * 2 for x in items if x > 0]` | List/Dict comprehensions nativas |
| `def main():` | `def run(ctx, input): return ...` | Injeção explícita de contexto OCap |

---

## 3. Alternativas Consideradas

1. **Manter retrocompatibilidade com tags antigas**: Rejeitada para evitar bifurcação cognitiva em humanos e alucinação em agentes de IA.
2. **Manter Starlark como nome público**: Rejeitada para solidificar a soberania e independência da linguagem ASL.
3. **Múltiplas tags (asl:rules vs asl:deterministic)**: Rejeitada pois ambas são facetas unificadas da mesma linguagem ASL.

---

## 4. Consequências

### Positivas
- **Máxima Simplicidade**: Existe apenas ````asl````. Não há bifurcação cognitiva.
- **Identidade Própria e Soberana**: O ecossistema não se define em função de ferramentas externas.
- **Zero Legado**: Código limpo, sem shims de compatibilidade ou branches condicionais para tags legadas.
- **Facilidade Absoluta para Agentes de IA**: Modelos geradores só precisam ser instruídos a emitir ````asl````.

### Negativas / Migração
- Documentos antigos contendo `asl:deterministic` ou `asl:rules` devem ser renomeados para `asl`. Como a decisão é de não retrocompatibilidade, a transição é imediata e inequívoca.
