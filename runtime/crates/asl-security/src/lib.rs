use asl_core_traits::{CapabilityContext, HttpResponsePayload};
use asl_spec::{AslError, Result, SkillCapabilities};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub mod crypto;
pub mod net;

/// Contexto de segurança puramente em memória (Mock) para testes herméticos rápidos
pub struct MockSecurityContext {
    virtual_fs: HashMap<String, String>,
    mock_env: HashMap<String, String>,
    mock_http: HashMap<String, HttpResponsePayload>,
    fuel_budget: u64,
    fuel_consumed: AtomicU64,
}

impl MockSecurityContext {
    pub fn new(initial_fuel: u64) -> Self {
        Self {
            virtual_fs: HashMap::new(),
            mock_env: HashMap::new(),
            mock_http: HashMap::new(),
            fuel_budget: initial_fuel,
            fuel_consumed: AtomicU64::new(0),
        }
    }

    pub fn with_file(mut self, path: impl Into<String>, content: impl Into<String>) -> Self {
        self.virtual_fs.insert(path.into(), content.into());
        self
    }

    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.mock_env.insert(key.into(), value.into());
        self
    }

    pub fn with_http(mut self, url: impl Into<String>, resp: HttpResponsePayload) -> Self {
        self.mock_http.insert(url.into(), resp);
        self
    }

    pub fn consume_fuel(&self, amount: u64) {
        self.fuel_consumed.fetch_add(amount, Ordering::Relaxed);
    }
}

impl CapabilityContext for MockSecurityContext {
    fn read_file(&self, path: &str) -> Result<Option<String>> {
        self.consume_fuel(1);
        Ok(self.virtual_fs.get(path).cloned())
    }

    fn sha256(&self, data: &str) -> String {
        self.consume_fuel(1);
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn base64_encode(&self, data: &str) -> String {
        self.consume_fuel(1);
        crypto::base64_encode(data)
    }

    fn base64_decode(&self, encoded: &str) -> Result<String> {
        self.consume_fuel(1);
        crypto::base64_decode(encoded)
    }

    fn env_var(&self, key: &str) -> Result<Option<String>> {
        self.consume_fuel(1);
        Ok(self.mock_env.get(key).cloned())
    }

    fn http_request(
        &self,
        _method: &str,
        url: &str,
        _headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<HttpResponsePayload> {
        self.consume_fuel(10);
        if let Some(resp) = self.mock_http.get(url) {
            Ok(resp.clone())
        } else {
            Ok(HttpResponsePayload {
                status: 200,
                headers: vec![("content-type".to_string(), "application/json".to_string())],
                body: body.unwrap_or("{}").to_string(),
            })
        }
    }

    fn check_fuel(&self) -> Result<u64> {
        Ok(self.fuel_budget.saturating_sub(self.fuel_consumed.load(Ordering::Relaxed)))
    }

    fn fuel_consumed(&self) -> u64 {
        self.fuel_consumed.load(Ordering::Relaxed)
    }
}

/// Contexto de segurança para execução real com confinamento de diretórios raiz
pub struct ConfinedSecurityContext {
    allowed_read_roots: Vec<PathBuf>,
    allowed_domains: Vec<String>,
    allowed_env_keys: Vec<String>,
    wall_clock_timeout_ms: u64,
    fuel_budget: u64,
    fuel_consumed: AtomicU64,
}

impl ConfinedSecurityContext {
    pub fn from_capabilities(caps: &SkillCapabilities, initial_fuel: u64) -> Self {
        let roots = caps
            .fs
            .confined_read_roots
            .iter()
            .map(|r| {
                let p = PathBuf::from(r);
                let abs = if p.is_relative() {
                    std::env::current_dir().map(|c| c.join(&p)).unwrap_or_else(|_| p.clone())
                } else {
                    p.clone()
                };
                std::fs::canonicalize(&abs).unwrap_or(abs)
            })
            .collect();

        Self {
            allowed_read_roots: roots,
            allowed_domains: caps.net.allow_domains.clone(),
            allowed_env_keys: caps.env.allow_keys.clone(),
            wall_clock_timeout_ms: 15_000,
            fuel_budget: initial_fuel,
            fuel_consumed: AtomicU64::new(0),
        }
    }

    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.wall_clock_timeout_ms = timeout_ms;
        self
    }

    pub fn consume_fuel(&self, amount: u64) {
        self.fuel_consumed.fetch_add(amount, Ordering::Relaxed);
    }
}

impl CapabilityContext for ConfinedSecurityContext {
    fn read_file(&self, path_str: &str) -> Result<Option<String>> {
        self.consume_fuel(1);
        let target_path = Path::new(path_str);

        // Se nenhuma raiz foi autorizada, o acesso é sumariamente negado
        if self.allowed_read_roots.is_empty() {
            return Err(AslError::CapabilityViolation(format!(
                "Read access denied: no confined root authorized for '{}'",
                path_str
            )));
        }

        // Canonicalize target path or its parent directory
        let canonical_target = if target_path.exists() {
            std::fs::canonicalize(target_path).map_err(|e| AslError::Io(e.to_string()))?
        } else if let Some(parent) = target_path.parent() {
            let canonical_parent = if parent.as_os_str().is_empty() {
                std::fs::canonicalize(".").map_err(|e| AslError::Io(e.to_string()))?
            } else if parent.exists() {
                std::fs::canonicalize(parent).map_err(|e| AslError::Io(e.to_string()))?
            } else {
                parent.to_path_buf()
            };
            if let Some(file_name) = target_path.file_name() {
                canonical_parent.join(file_name)
            } else {
                canonical_parent
            }
        } else {
            target_path.to_path_buf()
        };

        // Verify that canonical path strictly resides within an authorized root
        let is_allowed = self
            .allowed_read_roots
            .iter()
            .any(|root| canonical_target.starts_with(root));

        if !is_allowed {
            return Err(AslError::CapabilityViolation(format!(
                "Confined directory breakout attempt detected for '{}'",
                path_str
            )));
        }

        if !canonical_target.exists() {
            return Ok(None);
        }

        match std::fs::read_to_string(&canonical_target) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AslError::Io(e.to_string())),
        }
    }

    fn sha256(&self, data: &str) -> String {
        self.consume_fuel(1);
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn base64_encode(&self, data: &str) -> String {
        self.consume_fuel(1);
        crypto::base64_encode(data)
    }

    fn base64_decode(&self, encoded: &str) -> Result<String> {
        self.consume_fuel(1);
        crypto::base64_decode(encoded)
    }

    fn env_var(&self, key: &str) -> Result<Option<String>> {
        self.consume_fuel(1);
        if !self.allowed_env_keys.iter().any(|k| k == key) {
            return Err(AslError::CapabilityViolation(format!(
                "Environment variable '{}' is not authorized in capabilities.env.allow_keys ({:?})",
                key, self.allowed_env_keys
            )));
        }
        Ok(std::env::var(key).ok())
    }

    fn http_request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<HttpResponsePayload> {
        let body_len = body.map(|b| b.len()).unwrap_or(0);
        let fuel_cost = 100 + (body_len as u64 / 16);
        self.consume_fuel(fuel_cost);

        net::execute_http_request(
            method,
            url,
            headers,
            body,
            &self.allowed_domains,
            self.wall_clock_timeout_ms,
        )
    }

    fn check_fuel(&self) -> Result<u64> {
        Ok(self.fuel_budget.saturating_sub(self.fuel_consumed.load(Ordering::Relaxed)))
    }

    fn fuel_consumed(&self) -> u64 {
        self.fuel_consumed.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_security_context() {
        let ctx = MockSecurityContext::new(1000).with_file("test.txt", "hello world");
        let content = ctx.read_file("test.txt").unwrap();
        assert_eq!(content, Some("hello world".to_string()));

        let missing = ctx.read_file("missing.txt").unwrap();
        assert_eq!(missing, None);
    }

    #[test]
    fn test_confined_security_context_escape_prevention() {
        // Sem permissão: deve falhar
        let empty_caps = SkillCapabilities::default();
        let ctx_denied = ConfinedSecurityContext::from_capabilities(&empty_caps, 1000);
        let res = ctx_denied.read_file("Cargo.toml");
        assert!(matches!(res, Err(AslError::CapabilityViolation(_))));

        // Com raiz em '.', tentar escapar para /etc ou diretório pai
        let mut caps = SkillCapabilities::default();
        caps.fs.confined_read_roots.push(".".to_string());
        let ctx_allowed = ConfinedSecurityContext::from_capabilities(&caps, 1000);

        // Acesso legal dentro da raiz
        let valid_read = ctx_allowed.read_file("Cargo.toml");
        assert!(valid_read.is_ok());

        // Tentativa maliciosa de Directory Traversal
        let escape_attempt = ctx_allowed.read_file("../../../../../etc/passwd");
        assert!(matches!(escape_attempt, Err(AslError::CapabilityViolation(_))));
    }

    #[test]
    fn test_security_context_crypto_and_fuel() {
        let mock_ctx = MockSecurityContext::new(5000);
        assert_eq!(mock_ctx.fuel_consumed(), 0);
        let hash = mock_ctx.sha256("hello");
        assert_eq!(hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        assert_eq!(mock_ctx.fuel_consumed(), 1);
        assert_eq!(mock_ctx.check_fuel().unwrap(), 4999);

        let empty_caps = SkillCapabilities::default();
        let confined_ctx = ConfinedSecurityContext::from_capabilities(&empty_caps, 7777);
        assert_eq!(confined_ctx.fuel_consumed(), 0);
        assert_eq!(confined_ctx.check_fuel().unwrap(), 7777);
        assert_eq!(confined_ctx.sha256("asl"), "a12e45b23513ff84c05054772fedffc35f0b8a1bc87fb819906b3318b86dfd7a");
        assert_eq!(confined_ctx.fuel_consumed(), 1);
        assert_eq!(confined_ctx.check_fuel().unwrap(), 7776);
    }

    #[test]
    fn test_env_var_ocap_allowlist() {
        let mut caps = SkillCapabilities::default();
        caps.env.allow_keys.push("ALLOWED_KEY_XYZ".to_string());
        let ctx = ConfinedSecurityContext::from_capabilities(&caps, 1000);

        // Unauthorized access must fail with CapabilityViolation
        let res_unauth = ctx.env_var("SECRET_PASSWD");
        assert!(matches!(res_unauth, Err(AslError::CapabilityViolation(_))));

        // Authorized access succeeds (returns Ok(None) if not in actual process env)
        let res_auth = ctx.env_var("ALLOWED_KEY_XYZ");
        assert!(res_auth.is_ok());
    }

    #[test]
    fn test_http_request_domain_violation() {
        let mut caps = SkillCapabilities::default();
        caps.net.allow_domains.push("api.github.com".to_string());
        let ctx = ConfinedSecurityContext::from_capabilities(&caps, 1000);

        // Unauthorized domain must fail with CapabilityViolation
        let res = ctx.http_request("GET", "https://evil.com/leak", &[], None);
        assert!(matches!(res, Err(AslError::CapabilityViolation(_))));
    }
}
