use asl_core_traits::{CapabilityContext, HttpResponsePayload};
use asl_spec::{AslError, Result, SkillCapabilities};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

pub mod crypto;
pub mod fs;
pub mod net;

/// Pure in-memory security context (Mock) for fast hermetic unit tests
pub struct MockSecurityContext {
    virtual_fs: std::sync::RwLock<HashMap<String, String>>,
    mock_env: HashMap<String, String>,
    mock_http: HashMap<String, HttpResponsePayload>,
    fuel_budget: u64,
    fuel_consumed: AtomicU64,
}

impl MockSecurityContext {
    pub fn new(initial_fuel: u64) -> Self {
        Self {
            virtual_fs: std::sync::RwLock::new(HashMap::new()),
            mock_env: HashMap::new(),
            mock_http: HashMap::new(),
            fuel_budget: initial_fuel,
            fuel_consumed: AtomicU64::new(0),
        }
    }

    pub fn with_file(self, path: impl Into<String>, content: impl Into<String>) -> Self {
        self.virtual_fs.write().unwrap().insert(path.into(), content.into());
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

    pub fn consume_fuel(&self, amount: u64) -> Result<()> {
        let prev = self.fuel_consumed.fetch_add(amount, Ordering::SeqCst);
        let total = prev.saturating_add(amount);
        if total > self.fuel_budget {
            return Err(AslError::LimitExceeded(format!(
                "Fuel limit exceeded: budget is {} opcodes, attempted to consume {}",
                self.fuel_budget, total
            )));
        }
        Ok(())
    }
}

impl CapabilityContext for MockSecurityContext {
    fn read_file(&self, path: &str) -> Result<Option<String>> {
        self.consume_fuel(1)?;
        Ok(self.virtual_fs.read().unwrap().get(path).cloned())
    }

    fn write_file(&self, path: &str, content: &str) -> Result<()> {
        self.consume_fuel(1 + (content.len() as u64 / 16))?;
        self.virtual_fs.write().unwrap().insert(path.to_string(), content.to_string());
        Ok(())
    }

    fn file_exists(&self, path: &str) -> bool {
        let _ = self.consume_fuel(1);
        self.virtual_fs.read().unwrap().contains_key(path)
    }

    fn list_dir(&self, _path: &str) -> Result<Vec<String>> {
        self.consume_fuel(1)?;
        let mut keys: Vec<String> = self.virtual_fs.read().unwrap().keys().cloned().collect();
        keys.sort();
        Ok(keys)
    }

    fn sha256(&self, data: &str) -> String {
        let _ = self.consume_fuel(1);
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn base64_encode(&self, data: &str) -> String {
        let _ = self.consume_fuel(1);
        crypto::base64_encode(data)
    }

    fn base64_decode(&self, encoded: &str) -> Result<String> {
        self.consume_fuel(1)?;
        crypto::base64_decode(encoded)
    }

    fn env_var(&self, key: &str) -> Result<Option<String>> {
        self.consume_fuel(1)?;
        Ok(self.mock_env.get(key).cloned())
    }

    fn http_request(
        &self,
        _method: &str,
        url: &str,
        _headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<HttpResponsePayload> {
        let body_len = body.map(|b| b.len()).unwrap_or(0);
        self.consume_fuel(10 + (body_len as u64 / 16))?;
        let resp = if let Some(resp) = self.mock_http.get(url) {
            resp.clone()
        } else {
            HttpResponsePayload {
                status: 200,
                headers: vec![("content-type".to_string(), "application/json".to_string())],
                body: body.unwrap_or("{}").to_string(),
            }
        };
        self.consume_fuel(resp.body.len() as u64 / 16)?;
        Ok(resp)
    }

    fn consume_fuel(&self, amount: u64) -> Result<()> {
        self.consume_fuel(amount)
    }

    fn check_fuel(&self) -> Result<u64> {
        Ok(self.fuel_budget.saturating_sub(self.fuel_consumed.load(Ordering::Relaxed)))
    }

    fn fuel_consumed(&self) -> u64 {
        self.fuel_consumed.load(Ordering::Relaxed)
    }
}

/// Security context for confined execution with root directory confinement
pub struct ConfinedSecurityContext {
    allowed_read_roots: Vec<PathBuf>,
    allowed_write_roots: Vec<PathBuf>,
    allowed_domains: Vec<String>,
    allowed_env_keys: Vec<String>,
    wall_clock_timeout_ms: u64,
    fuel_budget: u64,
    fuel_consumed: AtomicU64,
}

impl ConfinedSecurityContext {
    pub fn from_capabilities(caps: &SkillCapabilities, initial_fuel: u64) -> Self {
        let read_roots = caps
            .fs
            .confined_read_roots
            .iter()
            .map(|r| fs::resolve_and_canonicalize_root(r))
            .collect();

        let write_roots = caps
            .fs
            .allow_write
            .iter()
            .map(|w| fs::resolve_and_canonicalize_root(w))
            .collect();

        Self {
            allowed_read_roots: read_roots,
            allowed_write_roots: write_roots,
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

    pub fn consume_fuel(&self, amount: u64) -> Result<()> {
        let prev = self.fuel_consumed.fetch_add(amount, Ordering::SeqCst);
        let total = prev.saturating_add(amount);
        if total > self.fuel_budget {
            return Err(AslError::LimitExceeded(format!(
                "Fuel limit exceeded: budget is {} opcodes, attempted to consume {}",
                self.fuel_budget, total
            )));
        }
        Ok(())
    }
}

impl CapabilityContext for ConfinedSecurityContext {
    fn read_file(&self, path_str: &str) -> Result<Option<String>> {
        self.consume_fuel(1)?;
        let canonical_target = fs::check_path_confinement(path_str, &self.allowed_read_roots)?;
        fs::safe_read_file(&canonical_target)
    }

    fn write_file(&self, path_str: &str, content: &str) -> Result<()> {
        let fuel_cost = 1 + (content.len() as u64 / 16);
        self.consume_fuel(fuel_cost)?;
        let canonical_target = fs::check_path_confinement(path_str, &self.allowed_write_roots)?;
        fs::safe_write_file(&canonical_target, content)
    }

    fn file_exists(&self, path_str: &str) -> bool {
        let _ = self.consume_fuel(1);
        if let Ok(canonical_target) = fs::check_path_confinement(path_str, &self.allowed_read_roots) {
            canonical_target.exists()
        } else {
            false
        }
    }

    fn list_dir(&self, path_str: &str) -> Result<Vec<String>> {
        self.consume_fuel(1)?;
        let canonical_target = fs::check_path_confinement(path_str, &self.allowed_read_roots)?;
        fs::safe_list_dir(&canonical_target)
    }

    fn sha256(&self, data: &str) -> String {
        let _ = self.consume_fuel(1);
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn base64_encode(&self, data: &str) -> String {
        let _ = self.consume_fuel(1);
        crypto::base64_encode(data)
    }

    fn base64_decode(&self, encoded: &str) -> Result<String> {
        self.consume_fuel(1)?;
        crypto::base64_decode(encoded)
    }

    fn env_var(&self, key: &str) -> Result<Option<String>> {
        self.consume_fuel(1)?;
        if !self.allowed_env_keys.iter().any(|k| k == key || k == "*") {
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
        self.consume_fuel(fuel_cost)?;

        let res = net::execute_http_request(
            method,
            url,
            headers,
            body,
            &self.allowed_domains,
            self.wall_clock_timeout_ms,
        )?;

        let resp_body_len = res.body.len();
        self.consume_fuel(resp_body_len as u64 / 16)?;
        Ok(res)
    }

    fn consume_fuel(&self, amount: u64) -> Result<()> {
        self.consume_fuel(amount)
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
    fn test_mock_http_fuel_metering() {
        let resp = HttpResponsePayload {
            status: 200,
            headers: vec![],
            body: "a".repeat(160),
        };
        let ctx = MockSecurityContext::new(5000).with_http("https://api.test/data", resp);
        assert_eq!(ctx.fuel_consumed(), 0);

        let req_body = "b".repeat(32);
        let _ = ctx.http_request("POST", "https://api.test/data", &[], Some(&req_body)).unwrap();

        // 10 base + 32/16 (2 req) + 160/16 (10 resp) = 22 fuel consumed
        assert_eq!(ctx.fuel_consumed(), 22);
    }

    #[test]
    fn test_confined_security_context_escape_prevention() {
        // Without root permission: must fail
        let empty_caps = SkillCapabilities::default();
        let ctx_denied = ConfinedSecurityContext::from_capabilities(&empty_caps, 1000);
        let res = ctx_denied.read_file("Cargo.toml");
        assert!(matches!(res, Err(AslError::CapabilityViolation(_))));

        // With root '.' configured, attempting to escape parent directory must fail
        let mut caps = SkillCapabilities::default();
        caps.fs.confined_read_roots.push(".".to_string());
        let ctx_allowed = ConfinedSecurityContext::from_capabilities(&caps, 1000);

        // Legal access within root
        let valid_read = ctx_allowed.read_file("Cargo.toml");
        assert!(valid_read.is_ok());

        // Malicious directory traversal attempt
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
