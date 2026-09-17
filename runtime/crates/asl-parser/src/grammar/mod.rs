pub mod gbnf;
pub mod regex_cfg;

use asl_core_traits::GrammarCompilerPort;
use asl_spec::Result;
use serde_json::Value;

pub struct GbnfGrammarCompiler;

impl GbnfGrammarCompiler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GbnfGrammarCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl GrammarCompilerPort for GbnfGrammarCompiler {
    fn compile_to_gbnf(&self, json_schema: &Value) -> Result<String> {
        gbnf::compile_schema_to_gbnf(json_schema)
    }

    fn compile_to_regex_cfg(&self, json_schema: &Value) -> Result<String> {
        regex_cfg::compile_schema_to_regex(json_schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_grammar_compiler_port_integration() {
        let compiler = GbnfGrammarCompiler::new();
        let schema = json!({
            "type": "object",
            "required": ["message"],
            "properties": {
                "message": { "type": "string" }
            }
        });

        let gbnf = compiler.compile_to_gbnf(&schema).expect("GBNF deve compilar");
        assert!(gbnf.contains("root ::="));
        assert!(gbnf.contains("\"\\\"message\\\"\""));

        let regex = compiler.compile_to_regex_cfg(&schema).expect("Regex deve compilar");
        assert!(regex.contains("\"message\""));
    }
}
