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

/// Porta de abstração de capacidades ocap injetadas
pub trait CapabilityContext: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Option<String>>;
    fn sha256(&self, data: &str) -> String;
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

/// Porta do Transpilador Semântico de Regras (asl:rules)
pub trait RulesTranspilerPort: Send + Sync {
    /// Transpila regras declarativas para código Starlark L1 verificável
    fn transpile(
        &self,
        rules_source: &str,
        manifest: &SkillManifest,
    ) -> Result<TranspilationResult>;
}

/// Resultado atômico da transpilação de regras
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranspilationResult {
    /// Código Starlark L1 hermético compilado
    pub starlark_code: String,
    /// Mapeamento de linhas para depuração e rastreabilidade
    pub source_map: Vec<SourceMapEntry>,
    /// Invariantes semânticos extraídos para o otimizador de KV-Cache
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
