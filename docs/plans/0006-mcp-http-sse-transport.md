# Plano de Implementação: Transporte MCP Remoto sobre HTTP / Server-Sent Events (SSE)

- **ADR Vinculado**: `docs/adrs/0006-mcp-http-sse-transport.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (3/3 Fases - 100%)
- **Meta**: Expor skills do ASL através de transporte remoto HTTP/SSE compatível com a especificação Model Context Protocol, permitindo conexão de agentes em rede via endpoints `/sse` e `/messages`.

---

## Fase 1: [x] Concluída - Criação e Configuração da Crate `asl-protocol-http`

### 1.1 Objetivo da Fase
Criar `runtime/crates/asl-protocol-http` e registrá-la no workspace Cargo com as dependências `tiny_http = "0.12"` e `asl-protocol-mcp`.

### 1.2 Código a Implementar
No arquivo `runtime/crates/asl-protocol-http/Cargo.toml`:
```toml
[package]
name = "asl-protocol-http"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
asl-spec.workspace = true
asl-core-traits.workspace = true
asl-protocol-mcp.workspace = true
serde_json.workspace = true
tiny_http = "0.12"
```

### 1.3 Verificação Local
```bash
cargo check -p asl-protocol-http
```

---

## Fase 2: [x] Concluída - Implementação do Servidor MCP HTTP/SSE (`asl-protocol-http/src/lib.rs`)

### 2.1 Objetivo da Fase
Implementar `McpHttpServer`:
- Tratamento de `GET /sse` com emissão do evento inicial `endpoint` apontando para `/messages`.
- Tratamento de `POST /messages` delegando o corpo JSON para `McpServer::handle_message`.
- Tratamento de `GET /health` retornando metadados de diagnóstico.

### 2.2 Testes Unitários da Fase
- Teste em porta efêmera (ex: `127.0.0.1:0`) enviando requisição HTTP POST com payload JSON-RPC `tools/list` e validando o JSON retornado.

### 2.3 Verificação Local
```bash
cargo test -p asl-protocol-http
cargo clippy -p asl-protocol-http -- -D warnings
```

---

## Fase 3: [x] Concluída - Integração no CLI e Validação de Guardrails

### 3.1 Objetivo da Fase
Adicionar ao CLI `asl serve` as opções `--transport <stdio|http>` e `--port <porta>` e verificar todos os guardrails.

### 3.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 3.3 Finalização (Commit & Push)
```bash
git add runtime/crates/asl-protocol-http runtime/crates/asl-cli docs/adrs/ docs/plans/ runtime/Cargo.toml
git commit -m "feat(protocol-http): implementar transporte MCP remoto HTTP/SSE (Marco 3)"
git push origin main
```
