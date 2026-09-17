# ADR-0006: Transporte MCP Remoto sobre HTTP / Server-Sent Events (SSE)

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Conselho de Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-protocol-http` (nova micro-crate), `asl-cli`, `runtime/Cargo.toml`

---

## 1. Contexto e Problema

O protocolo Model Context Protocol (MCP) especifica dois mecanismos canônicos de transporte:
1. **Stdio**: Invocação local orientada a subprocessos via pipes padrão de entrada e saída.
2. **HTTP com Server-Sent Events (SSE)**: Transporte remoto via rede para agentes executando em ambientes distribuídos, instâncias na nuvem ou ferramentas em contêineres Docker/Kubernetes onde canais de `stdio` não estão acessíveis.

No ASL 3.0, tínhamos implementado o transporte `stdio` em `asl-protocol-mcp`. Para habilitar o deployment do ASL como um microsserviço de ferramentas acessível remotamente por múltiplos agentes (Claude Desktop via URL, OpenAI agents, etc.), faz-se necessária a implementação do transporte oficial HTTP/SSE.

---

## 2. Proposta Detalhada da Decisão

Criar a micro-crate `asl-protocol-http` contendo o adaptador de transporte `McpHttpServer`.

### 2.1 Especificação do Transporte HTTP/SSE (MCP Spec)

1. **Endpoint `GET /sse`**:
   - Responde com cabeçalho `Content-Type: text/event-stream; charset=utf-8` e `Cache-Control: no-cache`.
   - Emite imediatamente o evento inicial `endpoint`:
     ```
     event: endpoint
     data: /messages
     ```
   - Mantém o canal de streaming ativo para push assíncrono de notificações.

2. **Endpoint `POST /messages`**:
   - Recebe a chamada JSON-RPC 2.0 (`initialize`, `tools/list`, `tools/call`).
   - Delega o parsing e a execução ao manipulador `McpServer::handle_message`.
   - Retorna a resposta JSON-RPC com cabeçalho `Content-Type: application/json; charset=utf-8`.

3. **Endpoint `GET /health`**:
   - Retorna `{"status": "ok", "version": "3.0.0", "skills": N}` para probes de liveness em clusters Kubernetes/Docker.

### 2.2 Escolha Tecnológica: `tiny_http`
Adotamos `tiny_http`, uma biblioteca HTTP 1.1 em Rust puro, síncrona, robusta, sem dependências C ou assíncronas pesadas (Tokio/Actix), com footprint de memória mínimo (< 2 MB de RSS) e compilação instantânea.

### 2.3 Integração CLI
O comando `asl serve` passa a aceitar argumentos de transporte:
```bash
asl serve --transport http --port 8080 <caminho_skills>
```

---

## 3. Alternativas Consideradas

- **Alternativa A: Usar Axum/Tokio completo**:
  - *Descarte*: Traz uma árvore de dezenas de dependências assíncronas pesadas que aumentam substancialmente o tempo de compilação do workspace e o tamanho binário, desnecessário para o throughput típico de MCP.
- **Alternativa B: Apenas manter transporte stdio**:
  - *Descarte*: Inviabiliza a utilização de skills ASL em agentes na nuvem e microsserviços remotos.
- **Alternativa C: `tiny_http` puro em `asl-protocol-http` (Escolhida)**:
  - *Justificativa*: Simplicidade máxima, zero dependências externas C, baixo footprint cognitivo e aderência aos 7 Axiomas.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - Suporte completo aos dois modos da especificação MCP (Stdio e HTTP/SSE).
  - Capacidade de servir centenas de skills para múltiplos agentes remotos em um único daemon HTTP.
- **Negativas / Mitigações**:
  - Conexões simultâneas de streaming SSE de longa duração consomem threads do pool HTTP (mitigado por keep-alive e timeouts configuráveis).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: As skills servidas remotamente mantêm seus arquivos atômicos `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: `tiny_http` é 100% Rust puro sobre `std::net`.
- [x] **Axioma 3 (Confinamento ocap)**: A segurança de execução continua estritamente sob `ConfinedSecurityContext`.
- [x] **Axioma 4 (Isolamento hexagonal)**: Camada de transporte periférica que consome `McpServer` sem vazamento de abstração.
- [x] **Axioma 5 (Término determinístico)**: Inalterado.
- [x] **Axioma 6 (Prefixo estático)**: Inalterado.
- [x] **Axioma 7 (Limite de < 400 linhas)**: O arquivo `asl-protocol-http/src/lib.rs` terá < 250 linhas.
