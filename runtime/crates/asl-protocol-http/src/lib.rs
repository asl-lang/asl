use asl_protocol_mcp::McpServer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub struct McpHttpServer<'a> {
    mcp_server: McpServer<'a>,
    port: u16,
}

impl<'a> McpHttpServer<'a> {
    pub fn new(mcp_server: McpServer<'a>, port: u16) -> Self {
        Self { mcp_server, port }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn run(&self, is_running: Arc<AtomicBool>) -> std::io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let server = Server::http(&addr)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        self.serve_requests(server, is_running)
    }

    pub fn serve_requests(&self, server: Server, is_running: Arc<AtomicBool>) -> std::io::Result<()> {
        while is_running.load(Ordering::Relaxed) {
            let mut request = match server.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(Some(rq)) => rq,
                Ok(None) => continue,
                Err(e) => {
                    eprintln!("[ASL MCP HTTP] Erro ao receber requisição: {}", e);
                    continue;
                }
            };

            let url = request.url().to_string();
            let method = request.method().clone();

            if method == Method::Get && (url == "/health" || url == "/") {
                let body = serde_json::json!({
                    "status": "ok",
                    "server": "asl-mcp-http",
                    "version": "3.0.0"
                })
                .to_string();
                let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                    .map_err(|_| std::io::Error::other("Invalid header"))?;
                let resp = Response::from_string(body)
                    .with_status_code(StatusCode(200))
                    .with_header(header);
                let _ = request.respond(resp);
            } else if method == Method::Get && url.starts_with("/sse") {
                let sse_event = "event: endpoint\ndata: /messages\n\n";
                let header_ct = Header::from_bytes(&b"Content-Type"[..], &b"text/event-stream"[..])
                    .map_err(|_| std::io::Error::other("Invalid header"))?;
                let header_cache = Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..])
                    .map_err(|_| std::io::Error::other("Invalid header"))?;
                let resp = Response::from_string(sse_event)
                    .with_status_code(StatusCode(200))
                    .with_header(header_ct)
                    .with_header(header_cache);
                let _ = request.respond(resp);
            } else if method == Method::Post && url.starts_with("/messages") {
                let mut content = String::new();
                if let Err(e) = request.as_reader().read_to_string(&mut content) {
                    eprintln!("[ASL MCP HTTP] Falha ao ler corpo da requisição: {}", e);
                    let resp = Response::from_string(r#"{"error": "Falha na leitura do corpo"}"#)
                        .with_status_code(StatusCode(400));
                    let _ = request.respond(resp);
                    continue;
                }

                if let Some(rpc_res) = self.mcp_server.handle_message(&content) {
                    let out_json = serde_json::to_string(&rpc_res).unwrap_or_default();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                        .map_err(|_| std::io::Error::other("Invalid header"))?;
                    let resp = Response::from_string(out_json)
                        .with_status_code(StatusCode(200))
                        .with_header(header);
                    let _ = request.respond(resp);
                } else {
                    let resp = Response::empty(StatusCode(204));
                    let _ = request.respond(resp);
                }
            } else {
                let resp = Response::from_string("Not Found").with_status_code(StatusCode(404));
                let _ = request.respond(resp);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asl_core_traits::CapabilityContext;
    use asl_spec::{Result, SkillDocument, SkillManifest};
    use asl_vm_starlark::StarlarkEngine;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    struct DummyContext;
    impl CapabilityContext for DummyContext {
        fn read_file(&self, _path: &str) -> Result<Option<String>> {
            Ok(None)
        }
        fn sha256(&self, _data: &str) -> String {
            "".to_string()
        }
        fn check_fuel(&self) -> Result<u64> {
            Ok(1_000_000)
        }
        fn fuel_consumed(&self) -> u64 {
            0
        }
    }

    #[test]
    fn test_mcp_http_health_and_messages() {
        let server = Server::http("127.0.0.1:0").expect("Deve vincular porta efêmera");
        let port = server.server_addr().to_ip().unwrap().port();

        let engine = StarlarkEngine::new();
        let ctx = DummyContext;
        let manifest: SkillManifest = serde_yaml::from_str(
            r#"
asl_version: "3.0"
name: "test-tool"
description: "Ferramenta de teste HTTP"
interface:
  entrypoint: "run"
"#,
        )
        .unwrap();

        let doc = SkillDocument {
            manifest,
            semantic_section: "Test".into(),
            deterministic_code: "def run(ctx, input): return {'ok': True}".into(),
            digest: "sha256:test".into(),
        };

        let mcp_server = McpServer::new(vec![doc], &engine, &ctx);
        let http_server = McpHttpServer::new(mcp_server, port);

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        std::thread::scope(|s| {
            s.spawn(move || {
                http_server.serve_requests(server, running_clone).unwrap();
            });

            // Teste 1: GET /health
            {
                let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
                let req = "GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
                stream.write_all(req.as_bytes()).unwrap();

                let mut resp = String::new();
                stream.read_to_string(&mut resp).unwrap();
                assert!(resp.contains("200 OK"));
                assert!(resp.contains("\"server\":\"asl-mcp-http\""));
            }

            // Teste 2: GET /sse
            {
                let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
                let req = "GET /sse HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
                stream.write_all(req.as_bytes()).unwrap();

                let mut resp = String::new();
                stream.read_to_string(&mut resp).unwrap();
                assert!(resp.contains("200 OK"));
                assert!(resp.contains("event: endpoint"));
                assert!(resp.contains("data: /messages"));
            }

            // Teste 3: POST /messages com tools/list
            {
                let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
                let body = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
                let req = format!(
                    "POST /messages HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                stream.write_all(req.as_bytes()).unwrap();

                let mut resp = String::new();
                stream.read_to_string(&mut resp).unwrap();
                assert!(resp.contains("200 OK"));
                assert!(resp.contains("test-tool"));
            }

            running.store(false, Ordering::Relaxed);
        });
    }
}
