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

    #[error("Missing ASL code block (```asl)")]
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

fn default_asl_version() -> String {
    "3.0".to_string()
}

fn default_interface() -> SkillInterface {
    SkillInterface::default()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    #[serde(default = "default_asl_version")]
    pub asl_version: String,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub signer_pubkey: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default = "default_interface")]
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
                "The 'name' field in the manifest cannot be empty".to_string(),
            ));
        }

        if !self.asl_version.starts_with("3.") && self.asl_version != "3.0" {
            return Err(AslError::InvalidFrontmatter(format!(
                "Incompatible ASL version '{}'. Expected ASL 3.x",
                self.asl_version
            )));
        }

        let ep = self.interface.entrypoint.trim();
        if ep.is_empty() {
            return Err(AslError::InvalidFrontmatter(
                "The 'interface.entrypoint' field cannot be empty".to_string(),
            ));
        }

        if !ep.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            || ep.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
        {
            return Err(AslError::InvalidFrontmatter(format!(
                "Invalid entrypoint '{}': must be a valid identifier",
                ep
            )));
        }

        Ok(())
    }
}

fn default_entrypoint() -> String {
    "run".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInterface {
    #[serde(default = "default_protocol")]
    pub protocol: String,
    #[serde(default = "default_entrypoint")]
    pub entrypoint: String,
    #[serde(default = "default_schema")]
    pub input_schema: serde_json::Value,
    #[serde(default)]
    pub output_schema: Option<serde_json::Value>,
}

impl Default for SkillInterface {
    fn default() -> Self {
        Self {
            protocol: default_protocol(),
            entrypoint: default_entrypoint(),
            input_schema: default_schema(),
            output_schema: None,
        }
    }
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
    pub env: EnvCapabilities,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvCapabilities {
    #[serde(default)]
    pub allow_keys: Vec<String>,
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

impl SkillDocument {
    pub fn draft_scaffold(name: &str) -> Self {
        let manifest = SkillManifest {
            asl_version: "3.0".to_string(),
            digest: None,
            signature: None,
            signer_pubkey: None,
            name: name.to_string(),
            version: Some("0.1.0".to_string()),
            description: format!("Draft skill '{}' under construction", name),
            license: None,
            interface: SkillInterface::default(),
            capabilities: SkillCapabilities::default(),
            limits: SkillLimits::default(),
        };
        let semantic_section = format!(
            "# {}\n\nDraft skill initialized via touch. Provide instructions here.",
            name
        );
        let deterministic_code = "def run(ctx, input):\n    return {}\n".to_string();
        let digest = "asl:sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string();
        Self {
            manifest,
            semantic_section,
            deterministic_code,
            rules_code: None,
            digest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub fuel_consumed: u64,
    pub execution_time_ns: u64,
    pub diagnostics: Vec<String>,
}

/// Tríade canônica de extensões de arquivo suportadas pelo ASL.
/// Apenas .skill gera projeção sombra (.md). Os formatos .tool e .asl operam sem sombra.
pub const ASL_EXTENSIONS: &[&str] = &[
    "skill", // Habilidade executável modular (com projeção sombra .md)
    "tool",  // Ferramenta executável / MCP tool (sem projeção sombra)
    "asl",   // Extensão nativa raiz da linguagem (sem projeção sombra)
];

/// Verifica se uma extensão de arquivo pertence à família canônica do ASL (insensível a maiúsculas).
pub fn is_asl_extension(ext: &str) -> bool {
    let lower = ext.to_ascii_lowercase();
    ASL_EXTENSIONS.iter().any(|&e| e == lower)
}

/// Verifica se um caminho de arquivo possui uma extensão canônica do ASL.
pub fn is_asl_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(is_asl_extension)
        .unwrap_or(false)
}

/// Verifica se o arquivo é elegível para projeção sombra (.md).
/// Apenas arquivos .skill geram sombra Markdown para retrocompatibilidade com descoberta legada.
pub fn is_shadow_eligible(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("skill"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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
        assert!(err2.to_string().contains("Missing ASL code block"));
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

        // Incompatible version
        manifest.asl_version = "2.0".to_string();
        assert!(manifest.validate().is_err());
        manifest.asl_version = "3.0".to_string();

        // Invalid entrypoint with hyphens or starting with a digit
        manifest.interface.entrypoint = "run-fn".to_string();
        assert!(manifest.validate().is_err());
        manifest.interface.entrypoint = "1run".to_string();
        assert!(manifest.validate().is_err());
        manifest.interface.entrypoint = "run_fn_123".to_string();
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_asl_extensions_recognition() {
        assert_eq!(ASL_EXTENSIONS.len(), 3);

        // Canonical triad (.skill, .tool, .asl) must be accepted
        for &ext in ASL_EXTENSIONS {
            assert!(is_asl_extension(ext), "Extension {} should be valid", ext);
            assert!(is_asl_extension(&ext.to_uppercase()), "Uppercase extension {} should be valid", ext);
            let path = Path::new("test").with_extension(ext);
            assert!(is_asl_file(&path), "File with extension {} should be recognized", ext);
        }

        // Invalid non-ASL extensions must be rejected
        assert!(!is_asl_extension("py"));
        assert!(!is_asl_extension("rs"));
        assert!(!is_asl_extension("json"));
        assert!(!is_asl_extension("md"));
        assert!(!is_asl_file(Path::new("README.md")));
        assert!(!is_asl_file(Path::new("script.sh")));
    }

    #[test]
    fn test_shadow_eligibility() {
        // Only .skill is eligible for shadow projection (.md)
        assert!(is_shadow_eligible(Path::new("my.skill")));
        assert!(is_shadow_eligible(Path::new("SKILL.SKILL")));

        // Nem .tool nem .asl geram .md sombra
        assert!(!is_shadow_eligible(Path::new("format.tool")));
        assert!(!is_shadow_eligible(Path::new("core.asl")));
        assert!(!is_shadow_eligible(Path::new("app.asl")));
    }
}
