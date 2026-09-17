use asl_spec::{ExecutionResult, Limits, Result, SkillDocument};
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

/// Porta de parsing de documentos .skill
pub trait ParserPort: Send + Sync {
    fn parse(&self, raw_content: &str) -> Result<SkillDocument>;
}

/// Porta de compilação AOT de gramáticas de decodificação para o LLM
pub trait GrammarCompilerPort: Send + Sync {
    fn compile_to_gbnf(&self, json_schema: &Value) -> Result<String>;
    fn compile_to_regex_cfg(&self, json_schema: &Value) -> Result<String>;
}
