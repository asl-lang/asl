use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AslError {
    #[error("I/O error: {0}")]
    Io(String),

    #[error("YAML serialization error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Frontmatter invalid: {0}")]
    InvalidFrontmatter(String),

    #[error("Missing deterministic code block (```asl or ```asl:deterministic)")]
    MissingDeterministicBlock,

    #[error("Starlark execution error: {0}")]
    StarlarkError(String),

    #[error("WASM execution error: {0}")]
    WasmError(String),

    #[error("Entrypoint '{0}' not found in skill code")]
    EntrypointNotFound(String),

    #[error("Capability violation: {0}")]
    CapabilityViolation(String),

    #[error("Execution limit exceeded: {0}")]
    LimitExceeded(String),

    #[error("Schema violation: {0}")]
    SchemaViolation(String),

    #[error("Rules transpilation error: {0}")]
    RulesTranspileError(String),
}

pub type Result<T> = std::result::Result<T, AslError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub asl_version: String,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub signer_pubkey: Option<String>,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: Option<String>,
    pub interface: SkillInterface,
    #[serde(default)]
    pub capabilities: SkillCapabilities,
    #[serde(default)]
    pub limits: SkillLimits,
}

impl SkillManifest {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(AslError::InvalidFrontmatter(
                "O campo 'name' no manifesto não pode ser vazio".to_string(),
            ));
        }

        if !self.asl_version.starts_with("3.") && self.asl_version != "3.0" {
            return Err(AslError::InvalidFrontmatter(format!(
                "Versão ASL '{}' incompatível. Esperado ASL 3.x",
                self.asl_version
            )));
        }

        let ep = self.interface.entrypoint.trim();
        if ep.is_empty() {
            return Err(AslError::InvalidFrontmatter(
                "O campo 'interface.entrypoint' não pode ser vazio".to_string(),
            ));
        }

        if !ep.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            || ep.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        {
            return Err(AslError::InvalidFrontmatter(format!(
                "Entrypoint '{}' inválido: deve ser um identificador válido",
                ep
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInterface {
    #[serde(default = "default_protocol")]
    pub protocol: String,
    pub entrypoint: String,
    #[serde(default = "default_schema")]
    pub input_schema: serde_json::Value,
    #[serde(default)]
    pub output_schema: Option<serde_json::Value>,
}

fn default_protocol() -> String {
    "mcp-tool-v1".to_string()
}

fn default_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {}
    })
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillCapabilities {
    #[serde(default)]
    pub fs: FsCapabilities,
    #[serde(default)]
    pub net: NetCapabilities,
    #[serde(default)]
    pub wasi_components: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FsCapabilities {
    #[serde(default)]
    pub confined_read_roots: Vec<String>,
    #[serde(default)]
    pub allow_write: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetCapabilities {
    #[serde(default)]
    pub allow_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLimits {
    #[serde(default = "default_fuel")]
    pub max_fuel_opcodes: u64,
    #[serde(default = "default_memory")]
    pub max_heap_kib: u64,
    #[serde(default = "default_timeout")]
    pub wall_clock_timeout_ms: u64,
}

pub type Limits = SkillLimits;

fn default_fuel() -> u64 {
    1_000_000
}

fn default_memory() -> u64 {
    8_192
}

fn default_timeout() -> u64 {
    1_000
}

impl Default for SkillLimits {
    fn default() -> Self {
        Self {
            max_fuel_opcodes: default_fuel(),
            max_heap_kib: default_memory(),
            wall_clock_timeout_ms: default_timeout(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDocument {
    pub manifest: SkillManifest,
    pub semantic_section: String,
    pub deterministic_code: String,
    #[serde(default)]
    pub rules_code: Option<String>,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub fuel_consumed: u64,
    pub execution_time_ns: u64,
    pub diagnostics: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_limits_default() {
        let limits = SkillLimits::default();
        assert_eq!(limits.max_fuel_opcodes, 1_000_000);
        assert_eq!(limits.max_heap_kib, 8_192);
        assert_eq!(limits.wall_clock_timeout_ms, 1_000);
    }

    #[test]
    fn test_manifest_deserialization_minimal() {
        let yaml_data = r#"
asl_version: "3.0"
name: "minimal-skill"
interface:
  entrypoint: "run"
"#;
        let manifest: SkillManifest = serde_yaml::from_str(yaml_data).expect("Should deserialize");
        assert_eq!(manifest.asl_version, "3.0");
        assert_eq!(manifest.name, "minimal-skill");
        assert_eq!(manifest.interface.protocol, "mcp-tool-v1");
        assert_eq!(manifest.limits.max_fuel_opcodes, 1_000_000);
    }

    #[test]
    fn test_asl_error_display() {
        let err = AslError::EntrypointNotFound("missing_fn".to_string());
        assert_eq!(err.to_string(), "Entrypoint 'missing_fn' not found in skill code");

        let err2 = AslError::MissingDeterministicBlock;
        assert!(err2.to_string().contains("Missing deterministic code block"));
    }

    #[test]
    fn test_manifest_validation() {
        let mut manifest = SkillManifest {
            asl_version: "3.0".to_string(),
            name: "valid-name".to_string(),
            description: "".to_string(),
            version: None,
            license: None,
            digest: None,
            signature: None,
            signer_pubkey: None,
            interface: SkillInterface {
                protocol: "mcp-tool-v1".to_string(),
                entrypoint: "run".to_string(),
                input_schema: serde_json::json!({}),
                output_schema: None,
            },
            capabilities: Default::default(),
            limits: Default::default(),
        };

        assert!(manifest.validate().is_ok());

        // Nome vazio
        manifest.name = "   ".to_string();
        assert!(manifest.validate().is_err());
        manifest.name = "valid_name".to_string();

        // Versão incompatível
        manifest.asl_version = "2.0".to_string();
        assert!(manifest.validate().is_err());
        manifest.asl_version = "3.0".to_string();

        // Entrypoint inválido com hífens ou iniciando com dígito
        manifest.interface.entrypoint = "run-fn".to_string();
        assert!(manifest.validate().is_err());
        manifest.interface.entrypoint = "1run".to_string();
        assert!(manifest.validate().is_err());
        manifest.interface.entrypoint = "run_fn_123".to_string();
        assert!(manifest.validate().is_ok());
    }
}
