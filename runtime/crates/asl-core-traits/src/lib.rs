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
