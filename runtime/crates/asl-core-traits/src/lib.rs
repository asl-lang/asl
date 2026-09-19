use asl_spec::{ExecutionResult, Limits, Result, SkillDocument, SkillManifest};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Porta de execução de lógica determinística (Hexagonal Engine Port)
pub trait EnginePort: Send + Sync {
    /// Identificador do motor (ex: "starlark-hermetic", "wasm-component")
    fn name(&self) -> &'static str;

    /// Executa uma função determinística do skill com isolamento de contexto e limits
    fn execute(
        &self,
        code: &str,
        entrypoint: &str,
        input_args: &Value,
        context: &dyn CapabilityContext,
        limits: &Limits,
    ) -> Result<ExecutionResult>;
}

/// Structured HTTP response returned from sandboxed network capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpResponsePayload {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

/// Porta de abstração de capacidades ocap injetadas
pub trait CapabilityContext: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Option<String>>;
    fn write_file(&self, _path: &str, _content: &str) -> Result<()> {
        Err(asl_spec::AslError::CapabilityViolation(
            "Filesystem write capability is not enabled in this context".to_string(),
        ))
    }
    fn file_exists(&self, _path: &str) -> bool {
        false
    }
    fn list_dir(&self, _path: &str) -> Result<Vec<String>> {
        Err(asl_spec::AslError::CapabilityViolation(
            "Directory listing capability is not enabled in this context".to_string(),
        ))
    }
    fn sha256(&self, data: &str) -> String;
    fn base64_encode(&self, data: &str) -> String;
    fn base64_decode(&self, encoded: &str) -> Result<String>;
    fn env_var(&self, key: &str) -> Result<Option<String>>;
    fn http_request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&str>,
    ) -> Result<HttpResponsePayload>;
    fn check_fuel(&self) -> Result<u64>;
    fn consume_fuel(&self, _amount: u64) -> Result<()> {
        Ok(())
    }
    fn fuel_consumed(&self) -> u64;
}

/// Porta de parsing de documentos ASL (.skill, .tool, .asl)
pub trait ParserPort: Send + Sync {
    fn parse(&self, raw_content: &str) -> Result<SkillDocument>;
}

/// Porta de compilação AOT de gramáticas de decodificação para o LLM
pub trait GrammarCompilerPort: Send + Sync {
    fn compile_to_gbnf(&self, json_schema: &Value) -> Result<String>;
    fn compile_to_regex_cfg(&self, json_schema: &Value) -> Result<String>;
}

/// Semantic Rules Transpiler Port
pub trait RulesTranspilerPort: Send + Sync {
    /// Transpiles declarative rules to verified ASL VM deterministic code
    fn transpile(
        &self,
        rules_source: &str,
        manifest: &SkillManifest,
    ) -> Result<TranspilationResult>;
}

/// Atomic result of rules transpilation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranspilationResult {
    /// Hermetic ASL VM deterministic code compiled from declarative rules
    pub starlark_code: String,
    /// Line mapping for debugging and source maps
    pub source_map: Vec<SourceMapEntry>,
    /// Semantic invariants extracted for KV-Cache prefix optimization
    pub static_invariants: Vec<String>,
}

/// Mapeamento de linhas fonte (Rules -> Starlark)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMapEntry {
    pub rules_line: usize,
    pub starlark_line: usize,
    pub description: String,
}

/// Erro de transpilação detalhado com linha e sugestão
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranspileError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
    pub suggestion: Option<String>,
}

/// Validates a JSON instance against a JSON Schema using the in-memory validator.
/// If schema is null or empty object, validation succeeds immediately.
pub fn validate_json_schema(schema: &Value, instance: &Value) -> Result<()> {
    if schema.is_null() || schema.as_object().map(|o| o.is_empty()).unwrap_or(false) {
        return Ok(());
    }
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| asl_spec::AslError::SchemaViolation(e.to_string()))?;
    if let Err(error) = validator.validate(instance) {
        return Err(asl_spec::AslError::SchemaViolation(error.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_schema_validation_rejection() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["username"],
            "properties": { "username": { "type": "string" } }
        });
        let invalid_input = serde_json::json!({ "username": 12345 });
        assert!(validate_json_schema(&schema, &invalid_input).is_err());

        let valid_input = serde_json::json!({ "username": "alice" });
        assert!(validate_json_schema(&schema, &valid_input).is_ok());

        // Empty schema should accept anything
        assert!(validate_json_schema(&serde_json::json!({}), &invalid_input).is_ok());
        assert!(validate_json_schema(&serde_json::Value::Null, &invalid_input).is_ok());
    }
}

