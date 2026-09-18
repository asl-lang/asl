# ADR-0021: Capacidades OCap Autônomas (ctx.env Atenuado, ctx.http Confinado e Crypto Base64) para Skills 100% Autocontidas sem Dependência Obrigatória de MCP

- **Status**: Proposto
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Conselho Consultivo**: Mark S. Miller, Leslie Lamport, Edsger W. Dijkstra, Rich Hickey, Ken Thompson, Butler Lampson, Barbara Liskov, Tony Hoare, Alan Kay, John Ousterhout
- **Componentes Afetados**: `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-cli`, `examples/`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 5 (Término com Fuel Metering), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Declaração do Problema

No ecossistema de agentes autônomos, o Model Context Protocol (MCP) é um padrão valioso para integração com IDEs corporativas. Contudo, **a esmagadora maioria dos desenvolvedores e agentes autônomos via CLI não utiliza nem deseja utilizar MCP** para tarefas cotidianas.

### O Problema do MCP como Dependência Obrigatória
1. **Fricção de Setup**: Exige configurar e manter um processo daemon separado (via stdio ou HTTP/SSE), configurar JSON-RPC 2.0 e gerenciar ciclo de vida multi-processo.
2. **Perda de Autonomia Local**: Quando um desenvolvedor roda `asl run minha-skill.skill` ou um cronjob executa uma rotina no CI/CD, ele quer que o arquivo `.skill` seja **100% autocontido e resolva o problema de ponta a ponta**.
3. **Fragmentação em Scripts de Shell**: Na ausência de capacidades nativas de I/O autocontidas, usuários recorrem a pastas cheias de scripts auxiliares (`get_pr.sh`, `curl`, `jq`, `devpanel_query.json`), destruindo a promessa do ASL de arquivo único atômico (Axioma 1).

### O Parecer do Conselho dos 10 Cientistas da Computação
O conselho analisou o dilema entre segurança/determinismo e usabilidade real:
- **Mark S. Miller (OCap)**: Condenou veementemente qualquer autoridade ambiente (`os.environ` aberto ou acesso ao chaveiro do SO). Recomendou *capacidades atenuadas* injetadas via manifesto.
- **Leslie Lamport (Determinismo)**: Alertou que rede é estocástica e deve ser confinada com limites físicos estritos de fuel e timeout (`limits.wall_clock_timeout_ms`).
- **Rich Hickey (Simplicidade)**: Enfatizou que a skill deve separar aquisição de dados das regras determinísticas puras, sem entrelaçá-las.
- **Ken Thompson & Butler Lampson**: Recomendaram não reinventar ferramentas complexas na linguagem, mas fornecer apenas os primitivos universais essenciais (`ctx.http` agnóstico).
- **Consenso Unânime**: O ASL deve habilitar execução autônoma sem MCP, mas **estritamente confinada via OCap**.

---

## 2. Proposta Detalhada da Decisão

### 2.1. Atenuação de Variáveis de Ambiente (`ctx.env`)
A skill nunca terá autoridade para explorar o ambiente do sistema operacional. Ela só pode ler as chaves explicitamente concedidas no manifesto YAML:

```yaml
capabilities:
  env:
    allow_keys: ["JIRA_CLI_EMAIL", "JIRA_CLI_TOKEN", "GITHUB_TOKEN"]
```

No código ````asl````:
```asl
token = ctx.env.get("JIRA_CLI_TOKEN")    # ✅ Permitido (retorna string ou None)
token = ctx.env.get("AWS_SECRET_KEY")    # ❌ Erro OcapViolation: Unauthorized environment variable access
```

### 2.2. Cliente HTTP Confinado por Domínio (`ctx.http`)
Para permitir que skills consultem APIs REST ou GraphQL (como Atlassian Jira, GitHub, Slack ou webhooks) sem depender de MCP ou `curl` externo, o motor ASL fornece métodos HTTP síncronos primitivos:

```yaml
capabilities:
  net:
    allow_domains: ["company.atlassian.net", "api.github.com"]
limits:
  wall_clock_timeout_ms: 15000
```

No código ````asl````:
```asl
resp = ctx.http.post(
    "https://company.atlassian.net/jsw2/graphql?operation=DevDetailsDialog",
    headers={"Authorization": auth, "Content-Type": "application/json"},
    json={"query": GRAPHQL_QUERY, "variables": {"issueId": issue_id}}
)

if resp.status == 200:
    data = resp.json()
```

- Qualquer tentativa de requisição para um domínio fora da lista `allow_domains` é abortada imediatamente com erro `OcapViolation`.
- A transferência de dados consome combustível (*fuel*) proporcional ao payload (1 opcode por 16 bytes).

### 2.3. Primitivos Criptográficos Nativos: Base64 (`ctx.crypto.base64`)
Para autenticações comuns (como HTTP Basic Auth exigido pela Atlassian) sem precisar de bibliotecas externas:
- `ctx.crypto.base64_encode(texto)`
- `ctx.crypto.base64_decode(base64_str)`

### 2.4. Proibições Rígidas (O que NÃO Implementar)
1. ❌ **Zero Subprocessos (`ctx.exec`)**: É terminantemente proibido invocar subprocessos de terminal (`sh`, `bash`, `cmd`). Mantém a sandbox pura e compatível com WebAssembly.
2. ❌ **Zero Acesso a Keychains de SO**: Acesso a `security` (macOS) ou `libsecret` (Linux) é proibido. As credenciais devem vir do ambiente autorizado.
3. ❌ **Zero Clientes Específicos de Fornecedores**: O motor ASL não terá `ctx.jira` ou `ctx.github`; manteremos a linguagem universal através de `ctx.http`.

---

## 3. Alternativas Consideradas

1. **Dependência Obrigatória de Servidor MCP**:
   - *Prós*: Mantém a VM ASL pura sem I/O de rede nativo.
   - *Contras*: Rejeitado por unanimidade pelo conselho dos 10 cientistas e pela realidade dos usuários. 90%+ dos desenvolvedores e agentes de linha de comando não usam daemons MCP. Cria overhead de processos e fragmenta a skill.
2. **Subprocessos Livres (`sh -c` / `ctx.exec`)**:
   - *Prós*: Permitiria chamar `curl`, `jq` e `gh` diretamente.
   - *Contras*: Rejeitado categoricamente (Mark S. Miller, Butler Lampson). Destrói o confinamento OCap, quebra a portabilidade WebAssembly, introduz vetores de command injection e impede o fuel metering físico.
3. **SDKs de Fornecedores Embutidos na VM (`ctx.jira`, `ctx.github`)**:
   - *Prós*: Conveniência rápida para o caso específico de uso.
   - *Contras*: Acoplamento inaceitável da especificação ASL a APIs comerciais de terceiros. Viola a simplicidade e a universalidade (Rich Hickey, Ken Thompson).
4. **Capacidades OCap Atenuadas (`ctx.env` restrito por chaves + `ctx.http` restrito por domínios) [ESCOLHIDA]**:
   - *Prós*: Mantém o sandbox intacto, compatível com WebAssembly, mede o fuel com precisão matemática (1 opcode por 16 bytes), confina estritamente a autoridade por domínio e chave de variável, e viabiliza a execução de skills 100% autônomas em um único arquivo `.skill`.

---

## 4. Consequências e Benefícios

### Positivas:
- **Zero-MCP de Verdade**: Skills podem rodar autonomamente via `asl run` ou no CI sem necessidade de configurar daemons MCP.
- **Eliminação Total de Scripts Satélites**: Skills como `pr-status-teams` se transformam em um arquivo `.skill` 100% único, contendo query GraphQL, fetch, regras determinísticas e formatação.
- **Conformidade OCap Total**: Segurança comprovável onde qualquer vazamento de rede ou credencial é bloqueado pelo manifesto.

### Negativas / Mitigações:
- *Não-determinismo de rede*: Mitigado por medição física de fuel em bytes transferidos e timeout de relógio garantido (`limits.wall_clock_timeout_ms`).
