# ADR-0013: Auditoria Científica e Correções do Ecossistema Dual-Consumer

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina (Cadente)
- **Revisores Científicos**: Painel de Arquitetura de Sistemas Autônomos
- **Crates Afetadas**: `asl-spec`, `asl-core-traits`, `asl-parser`, `asl-vm-starlark`, `asl-protocol-http`, `asl-cli`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade e Integridade), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 5 (Zero-Cost Tooling), Axioma 6 (Invariância de KV-Cache), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Motivação

O **Agent Skill Language (ASL 3.0)** adota o paradigma **Dual-Consumer**, onde um único documento serve de semântica rica para agentes/LLMs (Markdown com prefixo estático estável) e de lógica determinística hermética para runtimes de execução (Starlark, WebAssembly e Regras Declarativas).

Uma auditoria exaustiva de 31 hipóteses técnicas foi realizada no ecossistema ASL para identificar potenciais inconsistências, vulnerabilidades ou desvios de especificação. A auditoria confirmou a solidez do núcleo da arquitetura, mas revelou 10 pontos práticos de melhoria e correção:
1. **Host Policy Confinement**: Falta de suporte na CLI para interseção das capabilities solicitadas pela skill com as restrições impostas pelo host (`--allowed-root`).
2. **Cálculo de Digest Canônico**: O algoritmo de canonicalização de digest filtrava inadvertidamente qualquer linha iniciando com `digest:` ou `signature:` mesmo fora do frontmatter YAML (e.g. no corpo Markdown).
3. **Aviso de Auto-Assinatura**: Validação criptográfica do CLI permitia aceitar documentos autoassinados sem chave pública externa explícita sem alerta ao operador.
4. **Regras Declarativas com Regex**: O nó `PatternCondition::MatchesRegex` gerava código Starlark stub `if True:`, em vez de avaliar regex real em runtime.
5. **Múltiplos Blocos `match` Sequenciais**: O transpilador de regras emitia retorno incondicional precoce, bloqueando avaliação de blocos `match` subsequentes quando condições anteriores falhavam.
6. **Conformidade RFC 8259 no Compilador GBNF**: Gramáticas JSON permitiam inteiros com zeros à esquerda e não tratavam adequadamente caracteres de controle ASCII.
7. **Entrypoint Padrão em Skills Puras**: Geração de código determinístico fallback ignorava o `interface.entrypoint` declarado no frontmatter, assumindo `def run(...)`.
8. **Endereço de Escuta do Servidor HTTP**: O servidor MCP HTTP iniciava ouvindo em `0.0.0.0` por padrão, expondo desnecessariamente o serviço em redes locais.
9. **Streaming SSE Keepalive**: Endpoint `/sse` fechava a conexão após o primeiro evento, necessitando suporte a keepalive contínuo para clientes de streaming.
10. **Contagem Unicode no Analisador de Prefixo**: A métrica de tamanho de prefixo utilizava comprimento em bytes ao invés de contagem escalar de caracteres Unicode.

---

## 2. Proposta Detalhada da Arquitetura

### 2.1 Preservação do Modelo Dual-Consumer
As correções são estritamente conservadoras: mantêm o conceito Dual-Consumer intacto, sem reestruturações desnecessárias ou adição de runtimes externos pesados.

### 2.2 Transpilação Real de Regex em Starlark Hermético
Injeta-se na biblioteca nativa do Starlark a função `_asl_matches_regex(haystack, pattern)` utilizando o motor Rust `regex`. No transpilador de regras, `when matches("pattern"):` transpila para chamadas diretas a esse helper em vez do stub `if True:`.

### 2.3 Cálculo Preciso de Digest Canônico
A exclusão dos campos `digest:`, `signature:` e `signer_pubkey:` é limitada cirurgicamente às linhas delimitadas pelo bloco de frontmatter YAML inicial (`---` ... `---`). O corpo Markdown e o código determinístico são preservados integralmente no hashing SHA-256.

### 2.4 Interseção de Capabilities no Host
A CLI `asl run` passa a receber `--allowed-root <path>`, garantindo que as raízes de arquivo solicitadas no manifesto da skill sejam intersectadas e validadas contra a política do operador do host antes da instanciação do contexto de segurança `ConfinedSecurityContext`.

### 2.5 Servidor HTTP MCP Seguro por Padrão
O bind padrão do `asl serve` passa a ser `127.0.0.1`, com suporte à flag `--host` para configuração explícita e streaming SSE keepalive persistente.

---

## 3. Alternativas Consideradas

### Alternativa A: Reestruturar o Runtime para Adicionar Suporte a Python Nativo
- **Rejeitada**: Violaria o Axioma 2 (Zero Dependências Externas) e o isolamento hermético de segurança da ASL.

### Alternativa B: Permitir que Skills Concedam Suas Próprias Capabilities Sem Validação do Host
- **Rejeitada**: Quebraria o modelo de segurança Object-Capability (Axioma 3), permitindo que skills arbitrárias acessassem o filesystem do host sem consentimento.

### Alternativa C (Adotada): Correções Mínimas, Robustas e Isoladas em Hexagonalidade
- **Aprovada**: Preserva a arquitetura, atende aos 7 Axiomas e passa 100% dos testes e guardrails.

---

## 4. Consequências e Benefícios

### Positivas:
- **Segurança Reforçada**: Bind padrão em loopback, validação de chaves públicas, e política de host para capabilities.
- **Conformidade Estrita**: GBNF de acordo com RFC 8259, digest imutável contra colisões no markdown, regex funcional em regras.
- **Zero Quebras**: Nenhuma alteração incompatível na especificação `.skill`, `.tool` ou `.asl`.

### Negativas / Restrições:
- O transpilador de regras exige a presença da função nativa `_asl_matches_regex` em qualquer ambiente Starlark compatível com ASL Rules.

---

## 5. Conformidade com os 7 Axiomas do ASL

- **Axioma 1 (Atomicidade e Integridade)**: Digest canônico agora protege o markdown contra falso corte de linhas `digest:`.
- **Axioma 2 (Zero Dependências Externas)**: Regex e tratamentos implementados puramente em Rust.
- **Axioma 3 (Confinamento OCap)**: Interseção estrita com `--allowed-root` na CLI.
- **Axioma 4 (Isolamento Hexagonal)**: Adaptadores continuam independentes, comunicando-se unicamente via `asl-core-traits`.
- **Axioma 5 (Zero-Cost Tooling)**: Transpilação em memória Starlark preservada com zero I/O em disco.
- **Axioma 6 (Invariância de KV-Cache)**: Contagem Unicode precisa sem afetar o particionamento do prefixo estático.
- **Axioma 7 (Limite Cognitivo < 450 linhas)**: Todos os arquivos de implementação e testes permanecem estritamente abaixo de 450 linhas.
